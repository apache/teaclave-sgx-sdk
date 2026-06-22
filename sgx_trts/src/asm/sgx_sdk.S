/* Licensed to the Apache Software Foundation (ASF) under one
   or more contributor license agreements.  See the NOTICE file
   distributed with this work for additional information
   regarding copyright ownership.  The ASF licenses this file
   to you under the Apache License, Version 2.0 (the
   "License"); you may not use this file except in compliance
   with the License.  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing,
   software distributed under the License is distributed on an
   "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
   KIND, either express or implied.  See the License for the
   specific language governing permissions and limitations
   under the License.. */

.file "get_tds.S"
.section absolute

.equ SE_GUARD_PAGE_SHIFT, 16
.equ SE_GUARD_PAGE_SIZE,  (1 << SE_GUARD_PAGE_SHIFT)
.equ RED_ZONE_SIZE,       128
.equ STATIC_STACK_SIZE,   2656

.equ OCMD_ERET,           -1

.equ last_sp,             (SE_WORDSIZE * 1)
.equ stack_base_addr,     (SE_WORDSIZE * 2)
.equ stack_limit_addr,    (SE_WORDSIZE * 3)
.equ first_ssa_gpr,       (SE_WORDSIZE * 4)
.equ xsave_size,          (SE_WORDSIZE * 7)
.equ self_addr,           0
.equ stack_guard,         (SE_WORDSIZE * 5)
.equ aex_notify_flag,     (SE_WORDSIZE * 10)

/* SSA GPR */
.equ ssa_sp_t,            32
.equ ssa_sp_u,            144
.equ ssa_bp_u,            152
.equ ssa_exit_info,       160
.equ aex_notify,          167

.equ EXIT_INFO_VALID,     0x80000000
/* OCALL command */
.equ OCALL_FLAG,          0x04F434944

.equ RDRAND_RETRY_TIMES,  10

.equ ENCLAVE_INIT_NOT_STARTED,  0
.equ ENCLAVE_INIT_IN_PROGRESS,  1
.equ ENCLAVE_INIT_DONE,         2
.equ ENCLAVE_CRASHED,           3


.section .nipx,"ax",@progbits

/*
 * ---------------------------------------------------------------------
 *     Get the address of Tds
 * ---------------------------------------------------------------------
 */
DECLARE_LOCAL_FUNC get_tds
    READ_TD_DATA self_addr
    ret

.section .nipd,"aw"

XSAVE_ENABLED:
    .int 0

.section .text,"ax",@progbits

DECLARE_LOCAL_FUNC get_xsave_enabled
    lea_pic XSAVE_ENABLED, %rcx
    xor     %rax, %rax
    movl    (%rcx), %eax
    ret

DECLARE_LOCAL_FUNC set_xsave_enabled
    lea_pic XSAVE_ENABLED, %rax
    movl    %edi, (%rax)
    ret
