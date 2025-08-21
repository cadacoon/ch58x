#![no_std]
#![allow(mismatched_lifetime_syntaxes, non_camel_case_types)]

#[cfg(feature = "critical-section")]
mod critical_section;
mod generated;
mod generic;
pub mod register;

pub use generated::*;
pub use generic::*;
