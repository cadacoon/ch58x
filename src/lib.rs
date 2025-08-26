#![no_std]
#![feature(abi_riscv_interrupt)]
#![allow(mismatched_lifetime_syntaxes, non_camel_case_types)]

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

pub struct Executor {
    inner: embassy_executor::raw::Executor,
    not_send: core::marker::PhantomData<*mut ()>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            inner: embassy_executor::raw::Executor::new(core::ptr::null_mut()),
            not_send: core::marker::PhantomData,
        }
    }

    pub fn run(&'static mut self, init: impl FnOnce(embassy_executor::Spawner)) -> ! {
        init(self.inner.spawner());

        loop {
            unsafe { self.inner.poll() };

            unsafe { Pfic::steal() }
                .sctlr()
                .modify(|_, w| w.wfitowfe().set_bit());
            riscv::asm::wfi();
        }
    }
}

#[export_name = "__pender"]
fn __pender(_: *mut ()) {
    unsafe { Pfic::steal() }
        .sctlr()
        .modify(|_, w| w.setevent().set_bit());
}
