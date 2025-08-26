#![no_std]
#![no_main]

use ch58x::{
    Peripherals,
    sys::{Config, SysExt},
    sysclk,
};
use embassy_executor::Spawner;
use embassy_time::Timer;
use riscv::asm::delay;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = unsafe { Peripherals::steal() };

    let sys = peripherals.sys.set(Config::pll(6));
    sysclk::init(peripherals.systick, &sys, &peripherals.pfic);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
