use crate::Sys;
use riscv::asm::{delay, nop};

pub trait SysExt {
    fn set(self, config: Config) -> Self;
    fn fsys(&self) -> u32;
}

impl SysExt for Sys {
    fn set(self, config: Config) -> Self {
        match config.clock32ksrc {
            Clock32KSrc::LSE => {
                // power-up external low speed oscillator
                with_safe_access_mode(|| {
                    self.ck32k_config()
                        .modify(|_, w| w.clk_xt32k_pon().set_bit());
                });
                delay(self.fsys() / 10 / 4);
                // ... and use it as 32k clock source
                with_safe_access_mode(|| {
                    self.ck32k_config()
                        .modify(|_, w| w.clk_osc32k_xt().set_bit());
                });
                delay(self.fsys() / 1000);
            }
            Clock32KSrc::LSI => {
                // power-up internal low speed oscillator and use it as 32k clock source
                with_safe_access_mode(|| {
                    self.ck32k_config().modify(|_, w| {
                        w.clk_osc32k_xt()
                            .clear_bit()
                            .clk_int32k_pon()
                            .set_bit()
                            .clk_xt32k_pon()
                            .clear_bit()
                    });
                });
            }
        }

        with_safe_access_mode(|| {
            self.pll_config()
                .modify(|r, w| unsafe { w.pll_cfg_dat().bits(r.pll_cfg_dat().bits() & !(1 << 5)) });
        });
        match config.clocksyssrc {
            ClockSysSrc::Clock32K => {
                // use 32k clock as system clock
                with_safe_access_mode(|| {
                    self.clk_sys_cfg()
                        .modify(|_, w| unsafe { w.clk_sys_mod().bits(0b11) });
                });
            }
            ClockSysSrc::HSE(div) => {
                // power-up external high speed oscillator
                if self.hfck_pwr_ctrl().read().clk_xt32m_pon().bit_is_clear() {
                    with_safe_access_mode(|| {
                        self.hfck_pwr_ctrl()
                            .modify(|_, w| w.clk_xt32m_pon().set_bit());
                    });
                    delay(2400);
                }
                // ... and use it as system clock source
                with_safe_access_mode(|| {
                    self.clk_sys_cfg().write(|w| unsafe {
                        w.clk_sys_mod().bits(0b00).clk_pll_div().bits(div & 0x1F)
                    });
                    nop();
                    nop();
                    nop();
                    nop();
                });
                nop();
                nop();

                with_safe_access_mode(|| {
                    self.flash_cfg().write(|w| unsafe { w.bits(0x51) });
                });
            }
            ClockSysSrc::PLL(div) => {
                // power-up pll
                if self.hfck_pwr_ctrl().read().clk_pll_pon().bit_is_clear() {
                    with_safe_access_mode(|| {
                        self.hfck_pwr_ctrl()
                            .modify(|_, w| w.clk_pll_pon().set_bit());
                    });
                    delay(4000);
                }
                // ... and use it as system clock source
                with_safe_access_mode(|| {
                    self.clk_sys_cfg().write(|w| unsafe {
                        w.clk_sys_mod().bits(0b01).clk_pll_div().bits(div & 0x1F)
                    });
                    nop();
                    nop();
                    nop();
                    nop();
                });

                if div == 6 {
                    with_safe_access_mode(|| {
                        self.flash_cfg().write(|w| unsafe { w.bits(0x02) });
                    });
                } else {
                    with_safe_access_mode(|| {
                        self.flash_cfg().write(|w| unsafe { w.bits(0x52) });
                    });
                }
            }
        }
        with_safe_access_mode(|| {
            self.pll_config().modify(|_, w| w.flash_io_mod().set_bit());
        });

        self
    }

    fn fsys(&self) -> u32 {
        let clk_sys_cfg = self.clk_sys_cfg().read();
        match clk_sys_cfg.clk_sys_mod().bits() {
            0b00 => 32_000_000 / clk_sys_cfg.clk_pll_div().bits() as u32,
            0b01 => 32_000_000 * 15 / clk_sys_cfg.clk_pll_div().bits() as u32,
            0b10 => 32_000_000,
            _ => 32_000,
        }
    }
}

pub enum Clock32KSrc {
    LSE,
    LSI,
}

pub enum ClockSysSrc {
    Clock32K,
    HSE(u8),
    PLL(u8),
}

pub struct Config {
    pub clock32ksrc: Clock32KSrc,
    pub clocksyssrc: ClockSysSrc,
}

pub fn with_safe_access_mode<R>(f: impl FnOnce() -> R) -> R {
    critical_section::with(|_| {
        unsafe {
            Sys::steal().safe_access_sig().write(|w| w.bits(0x57));
            Sys::steal().safe_access_sig().write(|w| w.bits(0xA8));
        }

        let value = f();

        unsafe {
            Sys::steal().safe_access_sig().write(|w| w.bits(0x00));
        }

        value
    })
}
