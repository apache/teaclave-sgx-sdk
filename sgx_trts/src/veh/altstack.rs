// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Trusted alternate stacks.
//!
//! The exception path refuses an exception whose stack pointer is outside
//! the current thread's stack: it would write the exception frame there.
//! Code that switches stacks inside the enclave (coroutines, fibers) runs on
//! memory it allocated itself, so a CPUID emulated by a handler, or any
//! other handled exception, would crash the enclave with `StackOverRun`.
//!
//! An alternate stack is registered here. The exception path then accepts
//! an exception raised on it, and builds the frame inside that same range.
//! Only enclave memory can be registered.
//!
//! The table is lock-free: the exception path reads it and must not wait on
//! a lock that the interrupted code may hold.

use crate::trts;
use core::sync::atomic::{AtomicUsize, Ordering};
use sgx_types::error::{SgxResult, SgxStatus};

/// Maximum number of alternate stacks registered at once.
pub const MAX_ALT_STACKS: usize = 256;

struct Slot {
    /// Lowest address of the stack; non-zero once the slot is claimed.
    limit: AtomicUsize,
    /// One past the highest address of the stack; non-zero once the slot is
    /// published. Readers only trust a slot whose `base` is non-zero.
    base: AtomicUsize,
}

#[allow(clippy::declare_interior_mutable_const)]
const EMPTY: Slot = Slot {
    limit: AtomicUsize::new(0),
    base: AtomicUsize::new(0),
};

static SLOTS: [Slot; MAX_ALT_STACKS] = [EMPTY; MAX_ALT_STACKS];

/// Register `[limit, limit + size)` as a trusted alternate stack. Returns a
/// handle for [`unregister_alt_stack`].
pub fn register_alt_stack(limit: usize, size: usize) -> SgxResult<usize> {
    ensure!(limit != 0 && size != 0, SgxStatus::InvalidParameter);
    let base = limit.checked_add(size).ok_or(SgxStatus::InvalidParameter)?;
    ensure!(
        trts::is_within_enclave(limit as *const u8, size),
        SgxStatus::InvalidParameter
    );

    for (i, slot) in SLOTS.iter().enumerate() {
        if slot
            .limit
            .compare_exchange(0, limit, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            slot.base.store(base, Ordering::Release);
            return Ok(i + 1);
        }
    }
    Err(SgxStatus::OutOfMemory)
}

/// Remove a stack registered with [`register_alt_stack`].
pub fn unregister_alt_stack(handle: usize) -> SgxResult {
    ensure!(
        handle != 0 && handle <= MAX_ALT_STACKS,
        SgxStatus::InvalidParameter
    );
    let slot = &SLOTS[handle - 1];
    ensure!(
        slot.base.load(Ordering::Acquire) != 0,
        SgxStatus::InvalidParameter
    );
    slot.base.store(0, Ordering::Release);
    slot.limit.store(0, Ordering::Release);
    Ok(())
}

/// The registered alternate stack containing `addr`, as `(limit, base)`.
pub(crate) fn find(addr: usize) -> Option<(usize, usize)> {
    SLOTS.iter().find_map(|slot| {
        let base = slot.base.load(Ordering::Acquire);
        if base == 0 {
            return None;
        }
        let limit = slot.limit.load(Ordering::Acquire);
        (limit <= addr && addr < base).then_some((limit, base))
    })
}
