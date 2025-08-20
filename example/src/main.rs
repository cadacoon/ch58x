#![no_std]
#![no_main]

use ch58x::Peripherals;
use ch58x_hal::{
    sys::{Config, SysExt},
    sysclk, usb,
};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_usb::class::cdc_acm::CdcAcmClass;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = Peripherals::take().unwrap();

    let sys = peripherals.sys.set(Config::pll(6));
    sysclk::init(peripherals.systick, &sys, &peripherals.pfic);

    let driver = usb::Driver::new(peripherals.usb);

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("USB-serial example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = embassy_usb::class::cdc_acm::State::new();

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    let mut usb = builder.build();
    let usb_fut = usb.run();
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            let mut buf = [0; 64];
            loop {
                let n = class.read_packet(&mut buf).await.unwrap();
                let data = &buf[..n];
                class.write_packet(data).await.unwrap();
            }
        }
    };
    join(usb_fut, echo_fut).await;
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
