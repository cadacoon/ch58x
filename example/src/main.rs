#![no_std]
#![no_main]

use ch58x::{Peripherals, sys::SysExt, sysclk, usb};
use embassy_executor::Spawner;
use embassy_futures::join::join;

#[embassy_executor::main(entry = "riscv_rt::entry", executor = "ch58x::Executor")]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take().unwrap();

    let sys = peripherals.sys.set(ch58x::sys::Config {
        clock32ksrc: ch58x::sys::Clock32KSrc::LSI,
        clocksyssrc: ch58x::sys::ClockSysSrc::PLL(6),
    });
    sysclk::init(peripherals.systick, &sys, &peripherals.pfic);

    let mut buf = [0; 256];
    let driver = usb::Driver::new(peripherals.usb, &mut buf);
    let config = embassy_usb::Config::new(0xc0de, 0xcafe);
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 7];

    let mut state = embassy_usb::class::cdc_acm::State::new();
    let mut build = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );
    let mut class = embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut build, &mut state, 64);

    let mut usb = build.build();
    let usb_fut = usb.run();
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            let _ = echo(&mut class).await;
        }
    };
    join(usb_fut, echo_fut).await;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

async fn echo<'a>(
    class: &mut embassy_usb::class::cdc_acm::CdcAcmClass<'a, usb::Driver<'a>>,
) -> Result<(), embassy_usb::driver::EndpointError> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        class.write_packet(data).await?;
    }
}
