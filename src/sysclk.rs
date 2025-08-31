use crate::{
    interrupt::{CoreInterrupt, Priority},
    pfic::PficExt,
    sys::SysExt,
    Pfic, Sys, Systick,
};
use core::{
    cell::{OnceCell, RefCell},
    sync::atomic::{AtomicU32, Ordering},
};
use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
use embassy_time_driver::TICK_HZ;
use embassy_time_queue_utils::Queue;

pub struct Driver {
    systick: OnceCell<Systick>,
    cnt_per_tick: AtomicU32,
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

unsafe impl Sync for Driver {}

embassy_time_driver::time_driver_impl!(static DRIVER: Driver = Driver {
    systick: OnceCell::new(),
    cnt_per_tick: AtomicU32::new(0),
    queue: Mutex::new(RefCell::new(Queue::new()))
});

impl Driver {
    fn init(&'static self, systick: Systick, sys: &Sys) {
        self.systick.set(systick).unwrap();
        let systick = self.systick.get().unwrap();

        let cnt_per_second = sys.fsys() as u64 / 8;
        let cnt_per_tick = cnt_per_second / TICK_HZ;
        self.cnt_per_tick
            .store(cnt_per_tick as u32, Ordering::Relaxed);

        systick.ctl().write(|w| w.init().set_bit().ste().set_bit());
        systick.cmp().reset();
        systick.s().write(|w| w.cntif().clear_bit());
    }

    fn now_cnt(&self) -> u64 {
        self.systick.get().unwrap().cnt().read().bits()
    }

    fn wake(&self) {
        let systick = self.systick.get().unwrap();

        // disarm alarm
        systick.ctl().modify(|_, w| w.stie().clear_bit());
        systick.s().write(|w| w.cntif().clear_bit());

        critical_section::with(|cs| {
            let mut next_cnt = self
                .queue
                .borrow(cs)
                .borrow_mut()
                .next_expiration(self.now_cnt());
            while !self.set(cs, next_cnt) {
                next_cnt = self
                    .queue
                    .borrow(cs)
                    .borrow_mut()
                    .next_expiration(self.now_cnt());
            }
        });
    }

    fn set(&self, _cs: CriticalSection, next_cnt: u64) -> bool {
        let systick = self.systick.get().unwrap();

        // already passed
        if next_cnt <= self.now_cnt() {
            return false;
        }

        // arm alarm
        systick.cmp().write(|w| unsafe { w.cmp().bits(next_cnt) });
        systick.ctl().modify(|_, w| w.stie().set_bit());
        systick.s().write(|w| w.cntif().clear_bit());

        // already passed, disarm alarm
        if next_cnt <= self.now_cnt() {
            systick.ctl().modify(|_, w| w.stie().clear_bit());
            systick.s().write(|w| w.cntif().clear_bit());
            return false;
        }

        true
    }
}

impl embassy_time_driver::Driver for Driver {
    fn now(&self) -> u64 {
        let cnt_per_tick = self.cnt_per_tick.load(Ordering::Relaxed) as u64;
        self.now_cnt() / cnt_per_tick
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        let cnt_per_tick = self.cnt_per_tick.load(Ordering::Relaxed) as u64;
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at * cnt_per_tick, waker) {
                let mut next = queue.next_expiration(self.now_cnt());
                while !self.set(cs, next) {
                    next = queue.next_expiration(self.now_cnt());
                }
            }
        })
    }
}

pub fn init(systick: Systick, sys: &Sys, pfic: &Pfic) {
    DRIVER.init(systick, sys);

    pfic.enable(CoreInterrupt::SysTick, Some(Priority::P15));
}

#[riscv_rt::core_interrupt(CoreInterrupt::SysTick)]
fn systick() {
    DRIVER.wake();
}
