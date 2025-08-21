#![no_std]
#![no_main]

use ch58x::{Gpioa, Gpiob, Peripherals, Tmr0, interrupt::ExternalInterrupt};
use ch58x_hal::{
    pfic::PficExt,
    sys::{Config, SysExt},
    sysclk,
};
use embassy_executor::Spawner;
use embassy_time::Timer;
use riscv::asm::delay;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = Peripherals::take().unwrap();

    let sys = peripherals.sys.set(Config::pll(6));
    sysclk::init(peripherals.systick, &sys, &peripherals.pfic);

    /*let tmr0r = peripherals.tmr0;
    tmr0r
        .cnt_end()
        .write(|w| unsafe { w.cnt_end().bits(sys.fsys()) });
    tmr0r.ctrl_mod().write(|w| w.all_clear().set_bit());
    tmr0r.ctrl_mod().write(|w| w.count_en().set_bit());
    tmr0r.inter_en().modify(|_, w| w.ie_cyc_end().set_bit());*/

    peripherals.pfic.enable(ExternalInterrupt::TMR0);

    let gpioa = peripherals.gpioa;
    let gpiob = peripherals.gpiob;

    let pins = [
        (0, 15),
        (1, 18),
        (1, 0),
        (1, 7),
        (0, 12),
        (0, 10),
        (0, 11),
        (1, 9),
        (1, 8),
        (1, 15),
        (1, 14),
        (1, 13),
        (1, 12),
        (1, 5),
        (0, 4),
        (1, 3),
        (1, 4),
        (1, 2),
        (1, 1),
        (1, 6),
        (1, 21),
        (1, 20),
        (1, 19),
    ];

    loop {
        for anode in pins {
            let mut dir = [gpioa.dir().read().bits(), gpiob.dir().read().bits()];
            let mut out = [gpioa.out().read().bits(), gpiob.out().read().bits()];

            let mut value = 0b1111;

            // HI
            dir[anode.0] |= 1 << anode.1;
            out[anode.0] |= 1 << anode.1;

            for cathode in pins {
                if anode == cathode {
                    continue;
                }

                if value & 1 != 0 {
                    // LO
                    dir[cathode.0] |= 1 << cathode.1;
                    out[cathode.0] &= !(1 << cathode.1);
                } else {
                    // NC
                    dir[cathode.0] &= !(1 << cathode.1);
                }

                value >>= 1;
            }

            gpioa.out().write(|w| unsafe { w.bits(out[0]) });
            gpioa.dir().write(|w| unsafe { w.bits(dir[0]) });
            gpiob.out().write(|w| unsafe { w.bits(out[1]) });
            gpiob.dir().write(|w| unsafe { w.bits(dir[1]) });
        }

        delay(1000);

        for anode in pins {
            let mut dir = [gpioa.dir().read().bits(), gpiob.dir().read().bits()];
            let mut out = [gpioa.out().read().bits(), gpiob.out().read().bits()];

            let mut value = 0b0000;

            // HI
            dir[anode.0] |= 1 << anode.1;
            out[anode.0] |= 1 << anode.1;

            for cathode in pins {
                if anode == cathode {
                    continue;
                }

                if value & 1 != 0 {
                    // LO
                    dir[cathode.0] |= 1 << cathode.1;
                    out[cathode.0] &= !(1 << cathode.1);
                } else {
                    // NC
                    dir[cathode.0] &= !(1 << cathode.1);
                }

                value >>= 1;
            }

            gpioa.out().write(|w| unsafe { w.bits(out[0]) });
            gpioa.dir().write(|w| unsafe { w.bits(dir[0]) });
            gpiob.out().write(|w| unsafe { w.bits(out[1]) });
            gpiob.dir().write(|w| unsafe { w.bits(dir[1]) });
        }

        Timer::after_millis(5).await;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let gpioa = unsafe { Gpioa::steal() };
    let gpiob = unsafe { Gpiob::steal() };
    let pins = [
        (0, 15),
        (1, 18),
        (1, 0),
        (1, 7),
        (0, 12),
        (0, 10),
        (0, 11),
        (1, 9),
        (1, 8),
        (1, 15),
        (1, 14),
        (1, 13),
        (1, 12),
        (1, 5),
        (0, 4),
        (1, 3),
        (1, 4),
        (1, 2),
        (1, 1),
        (1, 6),
        (1, 21),
        (1, 20),
        (1, 19),
    ];

    loop {
        for anode in pins {
            let mut dir = [gpioa.dir().read().bits(), gpiob.dir().read().bits()];
            let mut out = [gpioa.out().read().bits(), gpiob.out().read().bits()];

            let mut value = 0b1111;

            // HI
            dir[anode.0] |= 1 << anode.1;
            out[anode.0] |= 1 << anode.1;

            for cathode in pins {
                if anode == cathode {
                    continue;
                }

                if value & 1 != 0 {
                    // LO
                    dir[cathode.0] |= 1 << cathode.1;
                    out[cathode.0] &= !(1 << cathode.1);
                } else {
                    // NC
                    dir[cathode.0] &= !(1 << cathode.1);
                }

                value >>= 1;
            }

            gpioa.out().write(|w| unsafe { w.bits(out[0]) });
            gpioa.dir().write(|w| unsafe { w.bits(dir[0]) });
            gpiob.out().write(|w| unsafe { w.bits(out[1]) });
            gpiob.dir().write(|w| unsafe { w.bits(dir[1]) });
        }

        delay(1000);

        for anode in pins {
            let mut dir = [gpioa.dir().read().bits(), gpiob.dir().read().bits()];
            let mut out = [gpioa.out().read().bits(), gpiob.out().read().bits()];

            let mut value = 0b0000;

            // HI
            dir[anode.0] |= 1 << anode.1;
            out[anode.0] |= 1 << anode.1;

            for cathode in pins {
                if anode == cathode {
                    continue;
                }

                if value & 1 != 0 {
                    // LO
                    dir[cathode.0] |= 1 << cathode.1;
                    out[cathode.0] &= !(1 << cathode.1);
                } else {
                    // NC
                    dir[cathode.0] &= !(1 << cathode.1);
                }

                value >>= 1;
            }

            gpioa.out().write(|w| unsafe { w.bits(out[0]) });
            gpioa.dir().write(|w| unsafe { w.bits(dir[0]) });
            gpiob.out().write(|w| unsafe { w.bits(out[1]) });
            gpiob.dir().write(|w| unsafe { w.bits(dir[1]) });
        }

        delay(100000);
    }
}

#[riscv_rt::external_interrupt(ExternalInterrupt::TMR0)]
fn tmr0() {
    let tmr0 = unsafe { Tmr0::steal() };
    if tmr0.int_flag().read().if_cyc_end().bit() {
        let gpioa = unsafe { Gpioa::steal() };
        let gpiob = unsafe { Gpiob::steal() };
        let pins = [
            (0, 15),
            (1, 18),
            (1, 0),
            (1, 7),
            (0, 12),
            (0, 10),
            (0, 11),
            (1, 9),
            (1, 8),
            (1, 15),
            (1, 14),
            (1, 13),
            (1, 12),
            (1, 5),
            (0, 4),
            (1, 3),
            (1, 4),
            (1, 2),
            (1, 1),
            (1, 6),
            (1, 21),
            (1, 20),
            (1, 19),
        ];

        for anode in pins {
            let mut dir = [gpioa.dir().read().bits(), gpiob.dir().read().bits()];
            let mut out = [gpioa.out().read().bits(), gpiob.out().read().bits()];

            let mut value = 0b1111;

            // HI
            dir[anode.0] |= 1 << anode.1;
            out[anode.0] |= 1 << anode.1;

            for cathode in pins {
                if anode == cathode {
                    continue;
                }

                if value & 1 != 0 {
                    // LO
                    dir[cathode.0] |= 1 << cathode.1;
                    out[cathode.0] &= !(1 << cathode.1);
                } else {
                    // NC
                    dir[cathode.0] &= !(1 << cathode.1);
                }

                value >>= 1;
            }

            gpioa.out().write(|w| unsafe { w.bits(out[0]) });
            gpioa.dir().write(|w| unsafe { w.bits(dir[0]) });
            gpiob.out().write(|w| unsafe { w.bits(out[1]) });
            gpiob.dir().write(|w| unsafe { w.bits(dir[1]) });
        }

        delay(1000);

        for anode in pins {
            let mut dir = [gpioa.dir().read().bits(), gpiob.dir().read().bits()];
            let mut out = [gpioa.out().read().bits(), gpiob.out().read().bits()];

            let mut value = 0b0000;

            // HI
            dir[anode.0] |= 1 << anode.1;
            out[anode.0] |= 1 << anode.1;

            for cathode in pins {
                if anode == cathode {
                    continue;
                }

                if value & 1 != 0 {
                    // LO
                    dir[cathode.0] |= 1 << cathode.1;
                    out[cathode.0] &= !(1 << cathode.1);
                } else {
                    // NC
                    dir[cathode.0] &= !(1 << cathode.1);
                }

                value >>= 1;
            }

            gpioa.out().write(|w| unsafe { w.bits(out[0]) });
            gpioa.dir().write(|w| unsafe { w.bits(dir[0]) });
            gpiob.out().write(|w| unsafe { w.bits(out[1]) });
            gpiob.dir().write(|w| unsafe { w.bits(dir[1]) });
        }

        delay(100000);

        tmr0.int_flag().write(|w| w.if_cyc_end().set_bit());
    }
}
