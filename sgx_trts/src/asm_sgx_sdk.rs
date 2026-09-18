use core::arch::global_asm;

global_asm!(include_str!("asm/cet.S"));
global_asm!(include_str!("asm/macro.S"));

cfg_if! {
    if #[cfg(feature = "sim")] {
        global_asm!(include_str!("inst/sim/td.S"), options(att_syntax));
        global_asm!(include_str!("inst/sim/xsave_mask.S"), options(att_syntax));
    } else if #[cfg(feature = "hyper")] {
        global_asm!(include_str!("inst/hyper/td.S"), options(att_syntax));
        global_asm!(include_str!("inst/hyper/xsave_mask.S"), options(att_syntax));
    } else {
        global_asm!(include_str!("inst/hw/td.S"), options(att_syntax));
        global_asm!(include_str!("inst/hw/xsave_mask.S"), options(att_syntax));
    }
}

global_asm!(include_str!("asm/sgx_sdk.S"), options(att_syntax));
