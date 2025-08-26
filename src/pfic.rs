use crate::{
    interrupt::{CoreInterrupt, Priority},
    Pfic,
};
use riscv::{InterruptNumber, PriorityNumber};

pub trait PficExt {
    fn enable(&self, interrupt: CoreInterrupt);
    fn disable(&self, interrupt: CoreInterrupt);
    fn is_enabled(&self, interrupt: CoreInterrupt) -> bool;

    fn pend(&self, interrupt: CoreInterrupt);
    fn unpend(&self, interrupt: CoreInterrupt);
    fn is_pending(&self, interrupt: CoreInterrupt) -> bool;

    fn is_active(&self, interrupt: CoreInterrupt) -> bool;

    fn set_priority(&self, interrupt: CoreInterrupt, priority: Priority);
    fn get_priority(&self, interrupt: CoreInterrupt) -> Priority;
}

impl PficExt for Pfic {
    fn enable(&self, interrupt: CoreInterrupt) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.ienr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn disable(&self, interrupt: CoreInterrupt) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.irer1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn is_enabled(&self, interrupt: CoreInterrupt) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.isr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn pend(&self, interrupt: CoreInterrupt) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.ipsr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn unpend(&self, interrupt: CoreInterrupt) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.iprr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn is_pending(&self, interrupt: CoreInterrupt) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.ipr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn is_active(&self, interrupt: CoreInterrupt) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        unsafe { self.iactr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn set_priority(&self, interrupt: CoreInterrupt, priority: Priority) {
        let off = interrupt.number();
        unsafe {
            self.iprior0()
                .as_ptr()
                .add(off)
                .write_volatile(priority.number() as _)
        }
    }

    fn get_priority(&self, interrupt: CoreInterrupt) -> Priority {
        let off = interrupt.number();
        Priority::from_number(unsafe { self.iprior0().as_ptr().add(off).read_volatile() } as _)
            .unwrap()
    }
}
