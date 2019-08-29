use std::cell::Cell;
use inplace_it::*;

struct DropCounter {
    count: Cell<usize>,
}

impl DropCounter {
    fn with_current<F: FnOnce(&DropCounter) -> R, R>(f: F) -> R {
        thread_local!(
            static COUNTER: DropCounter = DropCounter {count: Cell::new(0)};
        );
        COUNTER.with(f)
    }

    #[inline]
    fn get() -> usize {
        DropCounter::with_current(|c| c.count.get())
    }

    #[inline]
    fn inc() {
        DropCounter::with_current(|c| c.count.set(c.count.get() + 1));
    }

    #[inline]
    fn clear() {
        DropCounter::with_current(|c| c.count.set(0));
    }
}

struct DropCounterTrigger(u8 /* One byte to avoid zero-sized types optimizations */);

impl DropCounterTrigger {
    fn new() -> Self {
        Self(228)
    }
}

impl Drop for DropCounterTrigger {
    #[inline]
    fn drop(&mut self) {
        DropCounter::inc();
    }
}


#[test]
fn inplace_should_correctly_drop_values() {
    DropCounter::clear();
    inplace(|guard| {
        guard.init(DropCounterTrigger::new());
    });
    assert_eq!(DropCounter::get(), 1);
}

#[test]
fn inplace_array_should_correctly_drop_values() {
    for i in (0..4096).step_by(8) {
        DropCounter::clear();
        inplace_or_alloc_array(i, |guard| {
            guard.init_slice(|_| DropCounterTrigger::new());
        });
        assert_eq!(DropCounter::get(), i);
    }
}

#[test]
fn alloc_array_should_correctly_drop_values() {
    for i in (0..4096).step_by(8) {
        DropCounter::clear();
        alloc_array(i, |guard| {
            guard.init_slice(|_| DropCounterTrigger::new());
        });
        assert_eq!(DropCounter::get(), i);
    }
}
