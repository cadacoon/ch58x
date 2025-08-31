use crate::interrupt::CoreInterrupt;

#[riscv_rt::core_interrupt(CoreInterrupt::GPIOA)]
fn gpioa() {}

#[riscv_rt::core_interrupt(CoreInterrupt::GPIOB)]
fn gpiob() {}
