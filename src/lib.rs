#![no_std]
#![allow(mismatched_lifetime_syntaxes, non_camel_case_types)]

mod critical_section;
pub mod register;

mod generic;
pub use generic::*;

pub mod raw;
pub use raw::*;

extern crate embedded_hal as hal;

pub mod gpio;
pub mod pfic;
pub mod sys;
pub mod sysclk;
pub mod usb;
