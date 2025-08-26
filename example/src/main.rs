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

    let driver = usb::Driver::new(peripherals.usb);
    let config = embassy_usb::Config::new(0xc0de, 0xcafe);
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 7];

    let mut state = embassy_usb::class::cdc_acm::State::new();

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    let mut class = embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, &mut state, 64);

    let mut usb = builder.build();

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

struct Disconnected {}

impl From<embassy_usb::driver::EndpointError> for Disconnected {
    fn from(val: embassy_usb::driver::EndpointError) -> Self {
        match val {
            embassy_usb::driver::EndpointError::BufferOverflow => panic!("Buffer overflow"),
            embassy_usb::driver::EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn echo<'d>(
    class: &mut embassy_usb::class::cdc_acm::CdcAcmClass<'d, usb::Driver>,
) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        class.write_packet(data).await?;
    }
}
