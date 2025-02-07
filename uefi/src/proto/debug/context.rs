// SPDX-License-Identifier: MIT OR Apache-2.0

// note from the spec:
// When the context record field is larger than the register being stored in it, the upper bits of the
// context record field are unused and ignored
/// Universal EFI_SYSTEM_CONTEXT definition
/// This is passed to debug callbacks
#[repr(C)]
#[allow(missing_debug_implementations)]
pub union SystemContext {
    ebc: *mut SystemContextEBC,
    riscv_32: *mut SystemContextRiscV32,
    riscv_64: *mut SystemContextRiscV64,
    riscv_128: *mut SystemContextRiscV128,
    ia32: *mut SystemContextIA32,
    x64: *mut SystemContextX64,
    ipf: *mut SystemContextIPF,
    arm: *mut SystemContextARM,
    aarch64: *mut SystemContextAARCH64,
}

/// System context for virtual EBC processors
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextEBC {
    r0: u64,
    r1: u64,
    r2: u64,
    r3: u64,
    r4: u64,
    r5: u64,
    r6: u64,
    r7: u64,
    flags: u64,
    control_flags: u64,
    ip: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextRiscV32 {
    // Integer registers
    zero: u32,
    ra: u32,
    sp: u32,
    gp: u32,
    tp: u32,
    t0: u32,
    t1: u32,
    t2: u32,
    s0fp: u32,
    s1: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    t3: u32,
    t4: u32,
    t5: u32,
    t6: u32,
    // Float registers for F, D, and Q Standard Extensions
    ft0: u128,
    ft1: u128,
    ft2: u128,
    ft3: u128,
    ft4: u128,
    ft5: u128,
    ft6: u128,
    ft7: u128,
    fs0: u128,
    fs1: u128,
    fa0: u128,
    fa1: u128,
    fa2: u128,
    fa3: u128,
    fa4: u128,
    fa5: u128,
    fa6: u128,
    fa7: u128,
    fs2: u128,
    fs3: u128,
    fs4: u128,
    fs5: u128,
    fs6: u128,
    fs7: u128,
    fs8: u128,
    fs9: u128,
    fs10: u128,
    fs11: u128,
    ft8: u128,
    ft9: u128,
    ft10: u128,
    ft11: u128,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextRiscV64 {
    // Integer registers
    zero: u64,
    ra: u64,
    sp: u64,
    gp: u64,
    tp: u64,
    t0: u64,
    t1: u64,
    t2: u64,
    s0fp: u64,
    s1: u64,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
    t3: u64,
    t4: u64,
    t5: u64,
    t6: u64,
    // Floating registers for F, D, and Q Standard Extensions
    ft0: u128,
    ft1: u128,
    ft2: u128,
    ft3: u128,
    ft4: u128,
    ft5: u128,
    ft6: u128,
    ft7: u128,
    fs0: u128,
    fs1: u128,
    fa0: u128,
    fa1: u128,
    fa2: u128,
    fa3: u128,
    fa4: u128,
    fa5: u128,
    fa6: u128,
    fa7: u128,
    fs2: u128,
    fs3: u128,
    fs4: u128,
    fs5: u128,
    fs6: u128,
    fs7: u128,
    fs8: u128,
    fs9: u128,
    fs10: u128,
    fs11: u128,
    ft8: u128,
    ft9: u128,
    ft10: u128,
    ft11: u128,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextRiscV128 {
    // Integer registers
    zero: u128,
    ra: u128,
    sp: u128,
    gp: u128,
    tp: u128,
    t0: u128,
    t1: u128,
    t2: u128,
    s0fp: u128,
    s1: u128,
    a0: u128,
    a1: u128,
    a2: u128,
    a3: u128,
    a4: u128,
    a5: u128,
    a6: u128,
    a7: u128,
    s2: u128,
    s3: u128,
    s4: u128,
    s5: u128,
    s6: u128,
    s7: u128,
    s8: u128,
    s9: u128,
    s10: u128,
    s11: u128,
    t3: u128,
    t4: u128,
    t5: u128,
    t6: u128,
    // Floating registers for F, D, and Q Standard Extensions
    ft0: u128,
    ft1: u128,
    ft2: u128,
    ft3: u128,
    ft4: u128,
    ft5: u128,
    ft6: u128,
    ft7: u128,
    fs0: u128,
    fs1: u128,
    fa0: u128,
    fa1: u128,
    fa2: u128,
    fa3: u128,
    fa4: u128,
    fa5: u128,
    fa6: u128,
    fa7: u128,
    fs2: u128,
    fs3: u128,
    fs4: u128,
    fs5: u128,
    fs6: u128,
    fs7: u128,
    fs8: u128,
    fs9: u128,
    fs10: u128,
    fs11: u128,
    ft8: u128,
    ft9: u128,
    ft10: u128,
    ft11: u128,
}

/// System context for IA-32 processors (x86)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextIA32 {
    exception_data: u32, // additional data pushed on the stack by some types of exceptions
    fx_save_state: FxSaveStateIA32,
    dr0: u32,
    dr1: u32,
    dr2: u32,
    dr3: u32,
    dr6: u32,
    dr7: u32,
    cr0: u32,
    cr1: u32, // Noted as "Reserved" in the UEFI Specification
    cr2: u32,
    cr3: u32,
    cr4: u32,
    eflags: u32,
    ldtr: u32,
    tr: u32,
    gdtr: [u32; 2],
    idtr: [u32; 2],
    eip: u32,
    gs: u32,
    fs: u32,
    es: u32,
    ds: u32,
    cs: u32,
    ss: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
}

/// FP / MMX / XMM registers for IA-32
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FxSaveStateIA32 {
    fcw: u16,
    fsw: u16,
    ftw: u16,
    opcode: u16,
    eip: u32,
    cs: u16,
    reserved_1: u16,
    data_offset: u32,
    ds: u16,
    reserved_2: [u8; 10],
    st0mm0: [u8; 10],
    reserved_3: [u8; 6],
    st1mm1: [u8; 10],
    reserved_4: [u8; 6],
    st2mm2: [u8; 10],
    reserved_5: [u8; 6],
    st3mm3: [u8; 10],
    reserved_6: [u8; 6],
    st4mm4: [u8; 10],
    reserved_7: [u8; 6],
    st5mm5: [u8; 10],
    reserved_8: [u8; 6],
    st6mm6: [u8; 10],
    reserved_9: [u8; 6],
    st7mm7: [u8; 10],
    reserved_10: [u8; 6],
    xmm0: [u8; 16],
    xmm1: [u8; 16],
    xmm2: [u8; 16],
    xmm3: [u8; 16],
    xmm4: [u8; 16],
    xmm5: [u8; 16],
    xmm6: [u8; 16],
    xmm7: [u8; 16],
    reserved_11: [u8; 14 * 16],
}

/// System context for x64 processors
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextX64 {
    exception_data: u64, // additional data pushed on the stack by some types of exceptions
    fx_save_state: FxSaveStateX64,
    dr0: u64,
    dr1: u64,
    dr2: u64,
    dr3: u64,
    dr6: u64,
    dr7: u64,
    cr0: u64,
    cr1: u64, // Noted as "Reserved" in the UEFI Specification
    cr2: u64,
    cr3: u64,
    cr4: u64,
    cr8: u64,
    rflags: u64,
    ldtr: u64,
    tr: u64,
    gdtr: [u64; 2],
    idtr: [u64; 2],
    rip: u64,
    gs: u64,
    fs: u64,
    es: u64,
    ds: u64,
    cs: u64,
    ss: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rsp: u64,
    rbx: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}

/// FP / MMX / XMM registers for X64
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FxSaveStateX64 {
    fcw: u16,
    fsw: u16,
    ftw: u16,
    opcode: u16,
    rip: u64,
    data_offset: u64,
    reserved_1: [u8; 8],
    st0mm0: [u8; 10],
    reserved_2: [u8; 6],
    st1mm1: [u8; 10],
    reserved_3: [u8; 6],
    st2mm2: [u8; 10],
    reserved_4: [u8; 6],
    st3mm3: [u8; 10],
    reserved_5: [u8; 6],
    st4mm4: [u8; 10],
    reserved_6: [u8; 6],
    st5mm5: [u8; 10],
    reserved_7: [u8; 6],
    st6mm6: [u8; 10],
    reserved_8: [u8; 6],
    st7mm7: [u8; 10],
    reserved_9: [u8; 6],
    xmm0: [u8; 16],
    xmm1: [u8; 16],
    xmm2: [u8; 16],
    xmm3: [u8; 16],
    xmm4: [u8; 16],
    xmm5: [u8; 16],
    xmm6: [u8; 16],
    xmm7: [u8; 16],
    reserved_11: [u8; 14 * 16], // spec goes right from `Reserved9` to `Reserved11`
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextIPF {
    reserved: u64,
    r1: u64,
    r2: u64,
    r3: u64,
    r4: u64,
    r5: u64,
    r6: u64,
    r7: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    r16: u64,
    r17: u64,
    r18: u64,
    r19: u64,
    r20: u64,
    r21: u64,
    r22: u64,
    r23: u64,
    r24: u64,
    r25: u64,
    r26: u64,
    r27: u64,
    r28: u64,
    r29: u64,
    r30: u64,
    r31: u64,
    f2: [u64; 2],
    f3: [u64; 2],
    f4: [u64; 2],
    f5: [u64; 2],
    f6: [u64; 2],
    f7: [u64; 2],
    f8: [u64; 2],
    f9: [u64; 2],
    f10: [u64; 2],
    f11: [u64; 2],
    f12: [u64; 2],
    f13: [u64; 2],
    f14: [u64; 2],
    f15: [u64; 2],
    f16: [u64; 2],
    f17: [u64; 2],
    f18: [u64; 2],
    f19: [u64; 2],
    f20: [u64; 2],
    f21: [u64; 2],
    f22: [u64; 2],
    f23: [u64; 2],
    f24: [u64; 2],
    f25: [u64; 2],
    f26: [u64; 2],
    f27: [u64; 2],
    f28: [u64; 2],
    f29: [u64; 2],
    f30: [u64; 2],
    f31: [u64; 2],
    pr: u64,
    b0: u64,
    b1: u64,
    b2: u64,
    b3: u64,
    b4: u64,
    b5: u64,
    b6: u64,
    b7: u64,
    // application registers
    ar_rsc: u64,
    ar_bsp: u64,
    ar_bspstore: u64,
    ar_rnat: u64,
    ar_fcr: u64,
    ar_eflag: u64,
    ar_csd: u64,
    ar_ssd: u64,
    ar_cflg: u64,
    ar_fsr: u64,
    ar_fir: u64,
    ar_fdr: u64,
    ar_ccv: u64,
    ar_unat: u64,
    ar_fpsr: u64,
    ar_pfs: u64,
    ar_lc: u64,
    ar_ec: u64,
    // control registers
    cr_dcr: u64,
    cr_itm: u64,
    cr_iva: u64,
    cr_pta: u64,
    cr_ipsr: u64,
    cr_isr: u64,
    cr_iip: u64,
    cr_ifa: u64,
    cr_itir: u64,
    cr_iipa: u64,
    cr_ifs: u64,
    cr_iim: u64,
    cr_iha: u64,
    // debug registers
    dbr0: u64,
    dbr1: u64,
    dbr2: u64,
    dbr3: u64,
    dbr4: u64,
    dbr5: u64,
    dbr6: u64,
    dbr7: u64,
    ibr0: u64,
    ibr1: u64,
    ibr2: u64,
    ibr3: u64,
    ibr4: u64,
    ibr5: u64,
    ibr6: u64,
    ibr7: u64,
    // virtual Registers
    int_nat: u64, // nat bits for r1-r31
}

/// System context for ARM processors
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextARM {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    sp: u32,
    lr: u32,
    pc: u32,
    cpsr: u32,
    dfsr: u32,
    dfar: u32,
    ifsr: u32,
}

/// System context for AARCH64 processors
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemContextAARCH64 {
    // General Purpose Registers
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    x28: u64,
    fp: u64, // x29 - Frame Pointer
    lr: u64, // x30 - Link Register
    sp: u64, // x31 - Stack Pointer
    // FP/SIMD Registers
    v0: [u64; 2],
    v1: [u64; 2],
    v2: [u64; 2],
    v3: [u64; 2],
    v4: [u64; 2],
    v5: [u64; 2],
    v6: [u64; 2],
    v7: [u64; 2],
    v8: [u64; 2],
    v9: [u64; 2],
    v10: [u64; 2],
    v11: [u64; 2],
    v12: [u64; 2],
    v13: [u64; 2],
    v14: [u64; 2],
    v15: [u64; 2],
    v16: [u64; 2],
    v17: [u64; 2],
    v18: [u64; 2],
    v19: [u64; 2],
    v20: [u64; 2],
    v21: [u64; 2],
    v22: [u64; 2],
    v23: [u64; 2],
    v24: [u64; 2],
    v25: [u64; 2],
    v26: [u64; 2],
    v27: [u64; 2],
    v28: [u64; 2],
    v29: [u64; 2],
    v30: [u64; 2],
    v31: [u64; 2],
    elr: u64,  // Exception Link Register
    spsr: u64, // Saved Processor Status Register
    fpsr: u64, // Floating Point Status Register
    esr: u64,  // Exception Syndrome Register
    far: u64,  // Fault Address Register
}
