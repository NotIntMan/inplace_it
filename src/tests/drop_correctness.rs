use core::cell::Cell;
use crate::*;

struct DropCounter {
    count: Cell<usize>,
}

struct DropCounterItem<'a> {
    counter: &'a DropCounter,
}

impl DropCounter {
    #[inline]
    fn new() -> Self {
        DropCounter {
            count: Cell::new(0),
        }
    }

    #[inline]
    fn get(&self) -> usize {
        self.count.get()
    }

    #[inline]
    fn inc(&self) {
        self.count.set(
            self.count.get() + 1
        )
    }

    #[inline]
    fn make_item(&self) -> DropCounterItem {
        DropCounterItem {
            counter: self,
        }
    }
}

impl Drop for DropCounterItem<'_> {
    #[inline]
    fn drop(&mut self) {
        self.counter.inc()
    }
}


#[test]
fn inplace_should_correctly_drop_values() {
    let counter = DropCounter::new();
    inplace(|guard| {
        guard.init(counter.make_item());
    });
    assert_eq!(counter.get(), 1);
}

#[test]
fn inplace_array_should_correctly_drop_values() {
    for i in (0..4096).step_by(32) {
        let counter = DropCounter::new();
        inplace_array(i, |guard| {
            guard.init_slice(|_| counter.make_item());
        });
        assert_eq!(counter.get(), i);
    }
}

#[test]
fn alloc_array_should_correctly_drop_values() {
    for i in (0..4096).step_by(128) {
        let counter = DropCounter::new();
        alloc_array(i, |guard| {
            guard.init_slice(|_| counter.make_item());
        });
        assert_eq!(counter.get(), i);
    }
}
