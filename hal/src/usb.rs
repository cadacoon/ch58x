use core::{future::poll_fn, marker::PhantomData, task::Poll};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{
    Direction, EndpointAddress, EndpointAllocError, EndpointError, EndpointInfo, EndpointType,
    Event, Unsupported,
};
use pac::{Usb, interrupt::ExternalInterrupt};

pub struct Driver {
    usb: Usb,
}

impl Driver {
    pub fn new(usb: Usb) -> Self {
        Self { usb }
    }

    fn alloc_endpoint<D: Dir>(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Endpoint<D>, EndpointAllocError> {
        Ok(Endpoint {
            _phantom: PhantomData,
            info: EndpointInfo {
                addr: ep_addr.unwrap(),
                ep_type,
                max_packet_size,
                interval_ms,
            },
        })
    }
}

impl<'a> embassy_usb_driver::Driver<'a> for Driver {
    type EndpointOut = Endpoint<Out>;
    type EndpointIn = Endpoint<In>;
    type ControlPipe = ControlPipe;
    type Bus = Bus;

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        self.alloc_endpoint(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        self.alloc_endpoint(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn start(mut self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        (Self::Bus {}, Self::ControlPipe {})
    }
}

pub struct Bus {}

impl Bus {
    fn init(&mut self) {}
}

impl embassy_usb_driver::Bus for Bus {
    async fn enable(&mut self) {
        let usb = unsafe { Usb::steal() };
        critical_section::with(|_| usb.ctrl().modify(|_, w| w.uc_dev_pu_en().set_bit()));
    }

    async fn disable(&mut self) {}

    async fn poll(&mut self) -> Event {
        let usb = unsafe { Usb::steal() };
        poll_fn(|cx| {
            BUS_WAKER.register(cx.waker());

            let intfg = usb.int_fg().read();
            if intfg.uif_bus_rst().bit() {
                usb.dev_ad().reset();

                usb.uep0_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });
                usb.uep1_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });
                usb.uep2_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });
                usb.uep3_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });
                usb.uep4_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });
                usb.uep5_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });
                usb.uep6_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });
                usb.uep7_ctrl()
                    .write(|w| unsafe { w.uep_r_res().bits(0b10).uep_t_res().bits(0b10) });

                usb.int_fg().write(|w| w.uif_bus_rst().set_bit());
                critical_section::with(|_| {
                    usb.int_en().modify(|_, w| w.uie_bus_rst().set_bit());
                });

                Poll::Ready(Event::Reset)
            } else if intfg.uif_suspend().bit() {
                let misst = usb.mis_st().read();

                usb.int_fg().write(|w| w.uif_suspend().set_bit());
                critical_section::with(|_| usb.int_en().modify(|_, w| w.uie_suspend().set_bit()));

                if misst.ums_suspend().bit() {
                    Poll::Ready(Event::Suspend)
                } else {
                    Poll::Ready(Event::Resume)
                }
            } else {
                Poll::Pending
            }
        })
        .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        let usb = unsafe { Usb::steal() };
        match ep_addr.direction() {
            Direction::Out => {
                match ep_addr.index() {
                    1 => {
                        usb.uep4_1_mod().modify(|_, w| w.uep1_rx_en().bit(enabled));
                        usb.uep1_ctrl()
                            .write(|w| unsafe { w.uep_r_res().bits(0b10) }); // NAK
                    }
                    2 => {
                        usb.uep2_3_mod().modify(|_, w| w.uep2_rx_en().bit(enabled));
                        usb.uep2_ctrl()
                            .write(|w| unsafe { w.uep_r_res().bits(0b10) }); // NAK
                    }
                    3 => {
                        usb.uep2_3_mod().modify(|_, w| w.uep3_rx_en().bit(enabled));
                        usb.uep3_ctrl()
                            .write(|w| unsafe { w.uep_r_res().bits(0b10) }); // NAK
                    }
                    4 => {
                        usb.uep4_1_mod().modify(|_, w| w.uep4_rx_en().bit(enabled));
                        usb.uep4_ctrl()
                            .write(|w| unsafe { w.uep_r_res().bits(0b10) }); // NAK
                    }
                    5 => {
                        usb.uep567_mod().modify(|_, w| w.uep5_rx_en().bit(enabled));
                        usb.uep5_ctrl()
                            .write(|w| unsafe { w.uep_r_res().bits(0b10) }); // NAK
                    }
                    6 => {
                        usb.uep567_mod().modify(|_, w| w.uep6_rx_en().bit(enabled));
                        usb.uep6_ctrl()
                            .write(|w| unsafe { w.uep_r_res().bits(0b10) }); // NAK
                    }
                    7 => {
                        usb.uep567_mod().modify(|_, w| w.uep7_rx_en().bit(enabled));
                        usb.uep7_ctrl()
                            .write(|w| unsafe { w.uep_r_res().bits(0b10) }); // NAK
                    }
                    _ => unreachable!(),
                };
            }
            Direction::In => {
                match ep_addr.index() {
                    1 => {
                        usb.uep4_1_mod().modify(|_, w| w.uep1_tx_en().bit(enabled));
                        usb.uep1_ctrl()
                            .write(|w| unsafe { w.uep_t_res().bits(0b10) }); // NAK
                    }
                    2 => {
                        usb.uep2_3_mod().modify(|_, w| w.uep2_tx_en().bit(enabled));
                        usb.uep2_ctrl()
                            .write(|w| unsafe { w.uep_t_res().bits(0b10) }); // NAK
                    }
                    3 => {
                        usb.uep2_3_mod().modify(|_, w| w.uep3_tx_en().bit(enabled));
                        usb.uep3_ctrl()
                            .write(|w| unsafe { w.uep_t_res().bits(0b10) }); // NAK
                    }
                    4 => {
                        usb.uep4_1_mod().modify(|_, w| w.uep4_tx_en().bit(enabled));
                        usb.uep4_ctrl()
                            .write(|w| unsafe { w.uep_t_res().bits(0b10) }); // NAK
                    }
                    5 => {
                        usb.uep567_mod().modify(|_, w| w.uep5_tx_en().bit(enabled));
                        usb.uep5_ctrl()
                            .write(|w| unsafe { w.uep_t_res().bits(0b10) }); // NAK
                    }
                    6 => {
                        usb.uep567_mod().modify(|_, w| w.uep6_tx_en().bit(enabled));
                        usb.uep6_ctrl()
                            .write(|w| unsafe { w.uep_t_res().bits(0b10) }); // NAK
                    }
                    7 => {
                        usb.uep567_mod().modify(|_, w| w.uep7_tx_en().bit(enabled));
                        usb.uep7_ctrl()
                            .write(|w| unsafe { w.uep_t_res().bits(0b10) }); // NAK
                    }
                    _ => unreachable!(),
                };
            }
        }
        EP_WAKERS[ep_addr.index()].wake();
    }

    fn endpoint_set_stalled(&mut self, _ep_addr: EndpointAddress, _stalled: bool) {}

    fn endpoint_is_stalled(&mut self, _ep_addr: EndpointAddress) -> bool {
        false
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}

trait Dir {
    fn dir() -> Direction;
}

pub enum Out {}
impl Dir for Out {
    fn dir() -> Direction {
        Direction::Out
    }
}

pub enum In {}
impl Dir for In {
    fn dir() -> Direction {
        Direction::In
    }
}

pub struct Endpoint<D> {
    _phantom: PhantomData<D>,
    info: EndpointInfo,
}

impl embassy_usb_driver::Endpoint for Endpoint<Out> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let usb = unsafe { Usb::steal() };
        poll_fn(|cx| {
            EP_WAKERS[self.info.addr.index()].register(cx.waker());
            let enabled = match self.info.addr.index() {
                1 => usb.uep4_1_mod().read().uep1_rx_en().bit(),
                2 => usb.uep2_3_mod().read().uep2_rx_en().bit(),
                3 => usb.uep2_3_mod().read().uep3_rx_en().bit(),
                4 => usb.uep4_1_mod().read().uep4_rx_en().bit(),
                5 => usb.uep567_mod().read().uep5_rx_en().bit(),
                6 => usb.uep567_mod().read().uep6_rx_en().bit(),
                7 => usb.uep567_mod().read().uep7_rx_en().bit(),
                _ => unreachable!(),
            };
            if enabled {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl embassy_usb_driver::Endpoint for Endpoint<In> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let usb = unsafe { Usb::steal() };
        poll_fn(|cx| {
            EP_WAKERS[self.info.addr.index()].register(cx.waker());
            let enabled = match self.info.addr.index() {
                1 => usb.uep4_1_mod().read().uep1_tx_en().bit(),
                2 => usb.uep2_3_mod().read().uep2_tx_en().bit(),
                3 => usb.uep2_3_mod().read().uep3_tx_en().bit(),
                4 => usb.uep4_1_mod().read().uep4_tx_en().bit(),
                5 => usb.uep567_mod().read().uep5_tx_en().bit(),
                6 => usb.uep567_mod().read().uep6_tx_en().bit(),
                7 => usb.uep567_mod().read().uep7_tx_en().bit(),
                _ => unreachable!(),
            };
            if enabled {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl embassy_usb_driver::EndpointOut for Endpoint<Out> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let usb = unsafe { Usb::steal() };
        poll_fn(|cx| {
            EP_WAKERS[self.info.addr.index()].register(cx.waker());

            let intfg = usb.int_fg().read();
            let intst = usb.int_st().read();
            if intfg.uif_transfer().bit() && intst.uis_endp() == self.info.addr.index() as u8 {
                // OUT
                let res = if intst.uis_token() == 0b00 {
                    let len = usb.rx_len().read().bits() as usize;
                    if len == buf.len() {
                        Poll::Ready(Ok(len))
                    } else {
                        Poll::Ready(Err(EndpointError::BufferOverflow))
                    }
                } else {
                    Poll::Ready(Err(EndpointError::Disabled))
                };

                usb.uep0_ctrl().modify(|r, w| unsafe {
                    w.uep_r_res()
                        .bits(0b10)
                        .uep_r_tog()
                        .bit(!r.uep_r_tog().bit()) // NAK
                });

                usb.int_fg().write(|w| w.uif_transfer().set_bit());
                critical_section::with(|_| usb.int_en().modify(|_, w| w.uie_transfer().set_bit()));

                res
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl embassy_usb_driver::EndpointIn for Endpoint<In> {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        let usb = unsafe { Usb::steal() };

        usb.uep0_t_len()
            .write(|w| unsafe { w.uep0_t_len().bits(buf.len() as u8) });

        poll_fn(|cx| {
            EP_WAKERS[self.info.addr.index()].register(cx.waker());

            let intfg = usb.int_fg().read();
            let intst = usb.int_st().read();
            if intfg.uif_transfer().bit() && intst.uis_endp() == self.info.addr.index() as u8 {
                // IN
                let res = if intst.uis_token() == 0b10 {
                    Poll::Ready(Ok(()))
                } else {
                    Poll::Ready(Err(EndpointError::Disabled))
                };

                usb.uep0_ctrl().modify(|r, w| unsafe {
                    w.uep_t_res()
                        .bits(0b10)
                        .uep_t_tog()
                        .bit(!r.uep_t_tog().bit()) // NAK
                });

                usb.int_fg().write(|w| w.uif_transfer().set_bit());
                critical_section::with(|_| usb.int_en().modify(|_, w| w.uie_transfer().set_bit()));

                res
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

pub struct ControlPipe {}

impl embassy_usb_driver::ControlPipe for ControlPipe {
    fn max_packet_size(&self) -> usize {
        todo!()
    }

    async fn setup(&mut self) -> [u8; 8] {
        todo!()
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        first: bool,
        last: bool,
    ) -> Result<usize, EndpointError> {
        let usb = unsafe { Usb::steal() };
        if first {
            usb.uep0_ctrl()
                .write(|w| unsafe { w.uep_r_tog().set_bit().uep_r_res().bits(0) });
        }

        todo!()
    }

    async fn data_in(&mut self, data: &[u8], first: bool, last: bool) -> Result<(), EndpointError> {
        todo!()
    }

    async fn accept(&mut self) {
        todo!()
    }

    async fn reject(&mut self) {
        todo!()
    }

    async fn accept_set_address(&mut self, addr: u8) {
        todo!()
    }
}

const NEW_AW: AtomicWaker = AtomicWaker::new();
static BUS_WAKER: AtomicWaker = NEW_AW;
static EP_WAKERS: [AtomicWaker; 8] = [NEW_AW; 8];

#[riscv_rt::external_interrupt(ExternalInterrupt::USB)]
fn usb() {
    let usb = unsafe { Usb::steal() };

    let intfg = usb.int_fg().read();
    if intfg.uif_bus_rst().bit() {
        usb.int_en().modify(|_, w| w.uie_bus_rst().clear_bit());
        BUS_WAKER.wake();
    }
    if intfg.uif_suspend().bit() {
        usb.int_en().modify(|_, w| w.uie_suspend().clear_bit());
        BUS_WAKER.wake();
    }
    if intfg.uif_transfer().bit() {
        let intst = usb.int_st().read();
        match intst.uis_token().bits() {
            0 => {
                if intst.uis_tog_ok().bit() {
                    EP_WAKERS[intst.uis_endp().bits() as usize].wake();
                    usb.int_en().modify(|_, w| w.uie_transfer().clear_bit());
                } else {
                    usb.int_fg().write(|w| w.uif_transfer().set_bit());
                }
            }
            1 => {
                EP_WAKERS[intst.uis_endp().bits() as usize].wake();
                usb.int_en().modify(|_, w| w.uie_transfer().clear_bit());
            }
            _ => unreachable!(),
        }
    }
    if intfg.uif_hst_sof().bit() {
        usb.int_fg().write(|w| w.uif_hst_sof().set_bit());
    }
}
