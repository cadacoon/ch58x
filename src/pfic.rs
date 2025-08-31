use crate::{
    interrupt::{CoreInterrupt, Priority},
    Pfic,
};
use riscv::{InterruptNumber, PriorityNumber};

pub trait PficExt {
    fn enable(&self, interrupt: CoreInterrupt, priority: Option<Priority>);
    fn disable(&self, interrupt: CoreInterrupt);
    fn is_enabled(&self, interrupt: CoreInterrupt) -> bool;

    fn pend(&self, interrupt: CoreInterrupt);
    fn unpend(&self, interrupt: CoreInterrupt);
    fn is_pending(&self, interrupt: CoreInterrupt) -> bool;

    fn is_active(&self, interrupt: CoreInterrupt) -> bool;
}

impl PficExt for Pfic {
    fn enable(&self, interrupt: CoreInterrupt, priority: Option<Priority>) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid
        unsafe { self.ienr1().as_ptr().add(off).write_volatile(1 << bit) }
        if let Some(priority) = priority {
            // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid same as
            // with priorities
            unsafe {
                self.iprior0()
                    .as_ptr()
                    .add(interrupt.number())
                    .write_volatile(priority.number() as _)
            }
        }
    }

    fn disable(&self, interrupt: CoreInterrupt) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid
        unsafe { self.irer1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn is_enabled(&self, interrupt: CoreInterrupt) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid
        unsafe { self.isr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn pend(&self, interrupt: CoreInterrupt) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid
        unsafe { self.ipsr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn unpend(&self, interrupt: CoreInterrupt) {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid
        unsafe { self.iprr1().as_ptr().add(off).write_volatile(1 << bit) }
    }

    fn is_pending(&self, interrupt: CoreInterrupt) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid
        unsafe { self.ipr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }

    fn is_active(&self, interrupt: CoreInterrupt) -> bool {
        let off = interrupt.number() / 32;
        let bit = interrupt.number() % 32;
        // SAFETY: no arbitrary interrupt numbers, and all interrupts are valid
        unsafe { self.iactr1().as_ptr().add(off).read_volatile() & (1 << bit) != 0 }
    }
}
