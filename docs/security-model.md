# Security Model

This document describes the trust model, trust boundaries, and security
assumptions of the Apache Teaclave™ SGX SDK. It has two audiences:

1. **Developers** writing Intel SGX enclaves and their host applications with
   this SDK, who need to know where the security boundary is and what
   responsibilities fall on their code.
2. **Automated reviewers (including LLM-based audit agents)**, who need an
   explicit map of trust postures onto the repository's file structure so that
   findings are calibrated — flagging real boundary issues without raising false
   positives on code where the concern does not apply.

The model here follows Intel SGX and the
[Intel SGX SDK / EDL](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/overview.html)
that this Rust SDK builds on. Nothing here weakens or replaces the SGX hardware
guarantees; the SDK provides ergonomic Rust bindings and a trusted `std` on top
of them.

> This document describes the security model of the **SDK and its boundary
> code**. Individual samples under `samplecode/` are illustrative
> proof-of-concept enclaves and are not hardened to this model.

---

## 1. Trust model

An SGX system is partitioned into two domains. The processor enforces the
partition in hardware:

| | Untrusted host (REE) | Enclave (TEE) |
|---|---|---|
| Runs | The host application, the OS, the hypervisor, the BIOS | The enclave: your trusted code + this SDK's trusted runtime |
| Trust posture | **Untrusted** | **Trusted** |
| Memory | Cannot read enclave memory (EPC is encrypted/access-controlled) | Can read its own memory *and* host memory |
| In this SDK | `sgx_u*` crates (e.g. `sgx_urts`, `sgx_oc`) | `sgx_t*` crates (e.g. `sgx_trts`, `sgx_tstd`) |

The asymmetry matters: **the enclave can read and write untrusted host memory,
but the host cannot read enclave memory.** That is why untrusted *input* is the
hazard, not untrusted *observation* of enclave internals.

### Trusted Computing Base (TCB)

The following are **trusted** and assumed correct:

- The SGX-capable CPU and its microcode (the hardware root of trust).
- The Intel-provided architectural enclaves and the platform's SGX TCB.
- The enclave image itself, once measured (`MRENCLAVE`) and loaded — including
  **every Rust crate linked into it** (see §5).
- This SDK's trusted runtime (`sgx_trts`, `sgx_tstd`, and the other `sgx_t*`
  crates) running inside the enclave.

### Adversary

The adversary is assumed to control **everything outside the enclave**,
including privileged software. Concretely, the adversary can:

- Run the entire untrusted host: the OS kernel, the hypervisor, and the host
  application. They start, stop, and pause the enclave at will.
- Invoke any **ECALL** with **arbitrary arguments** — values, pointers, lengths,
  and buffer contents are all attacker-chosen.
- Return **arbitrary values from every OCALL**. Every service the enclave asks
  the host for (file I/O, time, network, randomness sourced outside SGX) returns
  attacker-controlled data.
- Read, map, unmap, and **concurrently mutate any untrusted memory**, including
  buffers an ECALL points the enclave at, during the call
  (Time-of-Check-to-Time-of-Use, TOCTOU / "double fetch").
- Delete, withhold, replay, or roll back anything the enclave persists outside
  the EPC, including sealed blobs and protected files (rollback / availability).
- Observe and manipulate the platform: schedule, interrupt, fault pages, and
  measure timing.

### Out of scope

SGX — and therefore this SDK — does **not** defend against the following.
A finding that depends only on one of these is generally not an SDK bug:

- **Microarchitectural and side-channel attacks**: cache timing, page-fault
  channels, branch prediction, and speculative-execution / transient-execution
  attacks (e.g. L1TF/Foreshadow, MDS). Mitigation is the platform's job.
- **Availability / denial of service**: the host controls scheduling and power
  and can refuse to run the enclave at any time.
- **Rollback of sealed data** when no anti-rollback mechanism (e.g. monotonic
  counters, an external freshness service) is used.
- **Physical attacks beyond SGX's memory-encryption guarantee.**

---

## 2. The trust boundary

The boundary is the **enclave edge**, crossed by two call directions defined in
EDL (Enclave Definition Language):

- **ECALL** — the host calls into the enclave. **ECALL parameters are
  attacker-controlled.**
- **OCALL** — the enclave calls out to the host. **OCALL return values and
  output buffers are attacker-controlled.**

```
   UNTRUSTED HOST (untrusted)            ││            ENCLAVE (trusted)
                                         ││
  ┌──────────────┐   ECALL  ───────────► ││ ──────────►  enclave logic
  │ host app +   │   (params untrusted)  ││              (your code + sgx_tstd)
  │ OS / VMM     │ ◄─────────── OCALL ── ││ ◄──────────  OCALL request
  └──────────────┘   (return untrusted)  ││
                            ENCLAVE EDGE  ↑↑
        ECALL params and OCALL returns are attacker-controlled,
        and untrusted memory may mutate concurrently — validate
        and copy-in before use; never trust an OCALL result for a
        security decision.
```

### EDL pointer annotations and what they guarantee

The EDL files (see `sgx_edl/edl/`, e.g. `sgx_edl/edl/sgx_file.edl`) declare how
each pointer parameter crosses the edge. The generated edge routines act on
these annotations:

- **`[in]`** — the marshaller **copies the buffer into enclave memory** before
  the call. The enclave then operates on a private copy (mitigates TOCTOU), but
  the *contents* are still attacker-chosen and must be validated.
- **`[out]`** — a buffer is allocated in enclave memory and copied back out on
  return. Do not write secrets here unless the caller is authorized to see them.
- **`[in, out]`** — copied in and back out.
- **`[string]` / `[size=...]` / `[count=...]`** — define the length the
  marshaller copies. A wrong or attacker-influenced size is the classic edge
  bug.
- **`[user_check]`** — **no copy and no checking.** The enclave receives a raw
  untrusted pointer and **must validate it itself** (see below) and must assume
  it can change under concurrent host access. This is the highest-risk
  annotation.

### Boundary checks the SDK provides

`sgx_trts` exposes the primitive range checks every enclave needs when handling
raw untrusted pointers:

- `is_within_enclave(p, len)` and `is_within_host(p, len)` —
  `sgx_trts/src/enclave/mem.rs:347`. They validate that a pointer range lies
  fully inside the enclave, or fully outside it, with overflow-safe arithmetic.
- The C ABI wrappers `sgx_is_within_enclave` / `sgx_is_outside_enclave` —
  `sgx_trts/src/capi.rs:163`.
- The `EnclaveRange` trait (`is_enclave_range` / `is_host_range`) for
  type-aware checks — `sgx_trts/src/enclave/mem.rs`.

### Boundary invariants enclave code must enforce

These are obligations on **trusted (`sgx_t*` / enclave) code**, not provided
automatically:

1. **A `[user_check]` pointer must be validated before any dereference.** Confirm
   the entire range is outside the enclave with `is_within_host` (you are about
   to read host data) — never partially inside, which could trick the enclave
   into reading its own memory.
2. **Copy untrusted input in once, then validate (avoid double-fetch / TOCTOU).**
   `[in]` does this for you; for `[user_check]`, copy to enclave memory before
   validating and using. Never read the same untrusted field twice and assume it
   is unchanged.
3. **Treat every byte of an ECALL buffer as adversarial input.** Length,
   encoding, and structure must all be checked. Do not assume NUL-termination,
   well-formedness, or non-emptiness.
4. **Treat every OCALL result as adversarial.** Return codes, output buffers, file
   contents, timestamps, and any "randomness" obtained via the host are
   attacker-controlled. Never base a security decision on an unauthenticated
   OCALL result; validate, bounds-check, and cryptographically verify where the
   value matters.
5. **Do not leak secrets through `[out]` / `[in, out]` buffers or return
   values.** Anything copied out becomes visible to the host. Size outputs
   deliberately and write only what the caller is authorized to learn.
6. **Fail closed.** On any validation failure, return an error — never proceed
   with partially validated input.

---

## 3. Trust-posture map of the repository

Use this table to decide whether a given concern (especially "untrusted input")
applies to a crate. This is the key reference for an automated reviewer. The
SDK encodes trust in the `t` / `u` naming convention and in Cargo features.

| Crate / path | Domain | Trust posture | What to scrutinize |
|---|---|---|---|
| `sgx_trts` | Enclave runtime | **Boundary + trusted** | The lowest layer of the boundary. Range checks (`enclave/mem.rs`, `capi.rs`), the entry/exit edge, exception handling. `unsafe` deref of untrusted pointers. |
| `sgx_tstd` | Enclave `std` | **Trusted** | Anything that reaches host services via OCALL and presents results as trustworthy (fs, time, env, net). |
| `sgx_tseal` | Enclave | **Trusted** | Sealing/unsealing; AAD handling; assumptions about freshness of sealed blobs (rollback). |
| `sgx_tse`, `sgx_tdh` | Enclave | **Trusted** | Report generation, key derivation, local (DH) attestation. Verify reports before trusting a peer enclave. |
| `sgx_dcap/tvl`, `sgx_dcap/tkey_exchange` | Enclave | **Trusted** | DCAP quote *verification* inside the enclave and RA key exchange. Quote/collateral are untrusted input until verified. |
| `sgx_protected_fs/tfs` | Enclave | **Trusted** | Encrypted file I/O; integrity vs. rollback of the file on the host. |
| `sgx_rsrvmm`, `sgx_sync`, `sgx_alloc`, `sgx_unwind` | Enclave | **Trusted** | Enclave-resident runtime; `unsafe` memory handling. |
| `sgx_urts` | Host | **Untrusted side** | Loads/manages the enclave from the host. Runs in the adversary's domain — "missing validation" here is generally **not** an enclave-security finding. |
| `sgx_oc` | Host | **Untrusted side** | OCALL implementations. Whatever this returns, the enclave must re-validate; bugs here are host-side, not TCB. |
| `sgx_protected_fs/ufs`, `sgx_key_exchange/ukey_exchange`, `sgx_dcap/.../umsg` | Host | **Untrusted side** | Untrusted halves of protected-FS / RA. Not in the TCB. |
| `sgx_edl` | Interface definition | **Boundary** | The EDL is the contract. Check pointer annotations: is anything `[user_check]` that should be `[in]`? Are `[size]`/`[count]` correct? A wrong annotation is a boundary vulnerability. |
| `sgx_crypto`, `sgx_rand`, `sgx_serialize` | **Feature-split** | **Depends on feature** | These compile trusted (`tcrypto`/`trand`/`tserialize`) **or** untrusted (`ucrypto`/`urand`/`userialize`). Determine which side a given build links before judging. Inside the enclave they are TCB; `sgx_rand` trusted must use an in-enclave entropy source. |
| `sgx_types`, `sgx_ffi`, `sgx_libc` | Shared / FFI | **Below the type system** | ABI struct definitions and libc surface. The Rust type system does not protect callers; review `unsafe` and ABI correctness. |
| `sgx_macros`, `sgx_demangle`, `sgx_build_helper`, `sgx_no_tstd`, `sgx_tests` | Build-time / tooling | Build-time | Not in the runtime TCB. Review as ordinary tooling (but note build-time supply-chain risk, §5). |
| `samplecode/` | Example enclaves + hosts | **Illustrative, not hardened** | Teaching material. Note copy-risk patterns (e.g. trusting an OCALL result) but don't report them as production vulnerabilities. |

---

## 4. Sealing, attestation, secrets, and other assumptions

- **Sealing (`sgx_tseal`) gives confidentiality and integrity, not freshness or
  availability.** A sealed blob is bound to `MRENCLAVE` (this exact enclave) or
  `MRSIGNER` (any enclave from the same signer), but it is stored in the
  untrusted host, which can delete, withhold, or **roll it back** to an earlier
  version. Anti-rollback requires an external mechanism (monotonic counter or
  freshness service); it is not provided by sealing alone.
- **`MRENCLAVE` vs `MRSIGNER` is a security decision.** `MRSIGNER` sealing lets a
  *future or different* enclave from the same signer unseal the data — broader
  exposure than `MRENCLAVE`. Choose deliberately.
- **Attestation establishes trust; it is not automatic.** Local attestation
  (`sgx_tdh`) and remote attestation (`sgx_key_exchange`, `sgx_dcap`) let a peer
  verify an enclave's measurement before exchanging secrets. The report/quote
  and its collateral are **untrusted input until verified** by trusted
  verification code (e.g. `sgx_dcap_tvl`).
- **OCALL results are never a security oracle.** Time, file existence,
  configuration, and any value crossing back from the host can be forged. Use
  authenticated/verified data for security decisions.
- **Secrets must not cross to the host in cleartext** unless the application's
  threat model explicitly accepts it.
- **Randomness inside the enclave must come from SGX (`RDRAND`/`sgx_read_rand`),
  not from a host OCALL.**

---

## 5. Dependencies and the supply chain

This is an SGX-specific concern that differs sharply from ordinary
applications: **every crate linked into the enclave runs inside the enclave and
is therefore part of the TCB.** A vulnerability or backdoor in an enclave
dependency is not "just" code execution in a host process — it is code execution
**inside the enclave**, with access to whatever secrets and sealing/attestation
keys the enclave holds. The boundary of §2 stops attacker *input* at the edge;
it does **not** sandbox the enclave's own dependencies.

A headline feature of this SDK is that **"most Rust crates work without
modifications"** and that Tokio/Tonic and similar run unmodified in the enclave.
That ergonomics win cuts both ways: arbitrary third-party code is pulled
straight into the TCB.

### What runs where

| Dependency kind | Executes | Trust domain |
|---|---|---|
| Crates linked into the **enclave** (`sgx_t*` + your trusted deps) | At runtime, **inside the enclave** | **TCB** — fully trusted, no sandbox |
| Crates linked into the **host** app (`sgx_u*` + host deps) | At runtime, in the untrusted host | Untrusted domain (not enclave-security-relevant) |
| `[build-dependencies]`, proc-macros (`sgx_macros`), `edger8r`/EDL codegen, build scripts | At **build time** on the developer/CI host | Build host — build-time code execution; also affects `MRENCLAVE` |

### Consequences for the audit

- **The TCB includes the full transitive dependency tree linked into the
  enclave.** When reviewing an enclave, in-scope code is not only the enclave
  crate — it is every crate it links, including async runtimes, parsers, and
  crypto.
- **`no_std` vs std-aware (`BUILD_STD`) is a TCB-size decision, not only an
  ergonomics one.** The smaller the code linked into the enclave, the smaller
  the TCB and the smaller the `MRENCLAVE`-measured surface. Prefer the minimal
  dependency set that meets the requirement.
- **Software crypto and key handling run in enclave memory.** That is expected
  under SGX (memory is protected), but it means key material lives in EPC and
  the implementation's correctness and side-channel resistance are your concern.
- **Randomness must be SGX-backed.** Any crate that needs entropy inside the
  enclave must source it from `RDRAND` / `sgx_read_rand`, never from a host
  OCALL. Verify the entropy path of crypto dependencies.
- **Build-time dependencies affect the measurement.** Proc-macros, build
  scripts, and the EDL/edge code generator execute arbitrary code on the build
  host and determine the enclave binary that gets measured into `MRENCLAVE`.
  Treat the build host and CI as part of the trust chain.

### Recommendations

- **Pin and lock** enclave dependencies (commit `Cargo.lock`) so the measured
  enclave is reproducible.
- Run **`cargo audit`** (RUSTSEC advisories) and consider **`cargo deny`** to
  enforce licenses and ban yanked/duplicate crates.
- Consider **`cargo vet`** or vendoring for the subset of crates linked into the
  enclave.
- **Minimize the enclave dependency tree** and review each new enclave
  dependency as a TCB addition, with the same scrutiny as first-party enclave
  code.

---

## 6. Guidance for automated / LLM security review

> **Note to the reviewing agent:** When you audit an enclave, **do not stop at
> the first-party enclave code — also review its dependencies whenever your
> tooling and context allow it.** Everything linked into the enclave runs inside
> the TCB (§5). If you can access the dependency sources or lockfile, enumerate
> the enclave's transitive dependencies, flag any that are unpinned, yanked, or
> carry known RUSTSEC advisories, and pay particular attention to crates that
> handle crypto, randomness/entropy, or deserialization inside the enclave. If
> you cannot reach the dependency sources, say so explicitly and list the
> enclave's declared dependencies as unreviewed TCB surface rather than silently
> skipping them.

To keep findings high-signal:

**Where real findings concentrate**
- ECALL entry points and anything reachable from them that reads ECALL
  parameters.
- Every `[user_check]` pointer: is the full range validated with
  `is_within_host` before dereference? Is it copied in once (no double-fetch /
  TOCTOU)?
- EDL annotations (`sgx_edl/`): a `[user_check]` that should be `[in]`, or a
  wrong `[size]`/`[count]`, is a boundary vulnerability.
- Code that trusts an **OCALL return value** (time, file contents, config,
  status) for a security decision.
- `unsafe` blocks in `sgx_trts` and the shared/FFI crates (`sgx_types`,
  `sgx_libc`, `sgx_ffi`) that dereference untrusted pointers or cross the ABI.
- Sealing/attestation code: unverified reports/quotes, `MRSIGNER`-vs-`MRENCLAVE`
  choices, and rollback assumptions on sealed blobs.
- Output buffers / return values that might leak more than intended.
- Enclave dependencies (§5): the in-scope TCB is the enclave's full transitive
  crate tree — including software crypto, the entropy/RNG path, and
  deserialization of attacker-influenced data inside the enclave.

**Expected non-findings (avoid these false positives)**
- "Missing input validation in the host" — host code (`sgx_urts`, `sgx_oc`,
  `*ufs`, `u*`) is in the untrusted domain; the enclave must validate regardless,
  so host-side validation is not a security control.
- Side-channel / speculative-execution / availability issues attributed to the
  SDK — these are outside the SGX threat model (§1) unless the SDK does something
  specifically worse than the platform baseline.
- Treating `samplecode/` as production code. Note copy-risk patterns, but frame
  them as illustrative.
- Flagging `unsafe` in FFI/shared crates merely for existing — FFI is `unsafe`
  by necessity. The finding must be a concrete mismatch or misuse.

**Before reporting**, state which side of the enclave edge the code runs on and
which adversary capability (§1) the issue depends on. If a finding does not trace
to a concrete adversary capability crossing the boundary, it is likely a false
positive.

---

## 7. Reporting vulnerabilities

Security issues in the SDK itself should be reported privately first, per
[`SECURITY.md`](../SECURITY.md), before any public disclosure.
