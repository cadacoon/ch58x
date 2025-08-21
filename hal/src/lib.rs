#![no_std]

extern crate ch58x as pac;
extern crate embedded_hal as hal;

pub mod gpio;
pub mod pfic;
pub mod sys;
pub mod sysclk;
pub mod usb;
