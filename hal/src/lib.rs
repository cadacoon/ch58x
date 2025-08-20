#![no_std]

pub extern crate ch58x as pac;
pub extern crate embedded_hal as hal;
pub extern crate riscv;

pub mod pfic;
pub mod sys;
pub mod sysclk;
pub mod usb;
