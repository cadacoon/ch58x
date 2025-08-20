#![no_std]
#![allow(mismatched_lifetime_syntaxes, non_camel_case_types)]

#[cfg(feature = "critical-section")]
mod critical_section;
mod generated;
mod generic;
pub mod register;

pub use generated::*;
pub use generic::*;

#[unsafe(no_mangle)]
unsafe fn __post_init(_a0: usize) {
    // Pipeline Control Bit && Dynamic Prediction Control
    register::corecfgr::write({
        let value = register::corecfgr::Corecfgr::from_bits(0x1F);
        value
    });

    // Enable nested interrupts and hardware stack push function
    register::intsyscr::write({
        let mut value = register::intsyscr::Intsyscr::from_bits(0);
        value.set_hwstken(true);
        value.set_inesten(true);
        value
    })
}
