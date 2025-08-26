#![no_std]
#![no_main]

use ch58x::{
    Peripherals,
    sys::{Config, SysExt},
    sysclk,
};
use embassy_executor::Spawner;

#[embassy_executor::main(entry = "riscv_rt::entry", executor = "ch58x::Executor")]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = Peripherals::take().unwrap();

    let sys = peripherals.sys.set(Config::pll(6));
    sysclk::init(peripherals.systick, &sys, &peripherals.pfic);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
