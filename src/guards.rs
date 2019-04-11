use core::{
    mem::{ManuallyDrop, uninitialized, forget, replace},
    ops::{Bound, RangeBounds},
};
use crate::FixedArray;

#[derive(Debug)]
pub struct UninitializedMemoryGuard<'a, T: ?Sized> {
    memory: &'a mut T,
}

#[inline]
pub fn inplace<T, R, Consumer: FnOnce(UninitializedMemoryGuard<T>) -> R>(consumer: Consumer) -> R {
    unsafe {
        let mut memory_holder = ManuallyDrop::new(uninitialized::<T>());
        consumer(UninitializedMemoryGuard { memory: &mut *memory_holder })
    }
}

impl<'a, T: ?Sized> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub unsafe fn unwrap(self) -> &'a mut T {
        self.memory
    }

    #[inline]
    pub unsafe fn init<Init: FnOnce(&mut T)>(self, init: Init) -> MemoryGuard<'a, T> {
        init(self.memory);
        MemoryGuard { memory: self.memory }
    }
}

impl<'a, T> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub fn init_with(self, value: T) -> MemoryGuard<'a, T> {
        unsafe {
            self.init(|mem| {
                forget(replace(mem, value))
            })
        }
    }
}

#[inline]
fn slice<T, Range: RangeBounds<usize>>(origin: &mut [T], range: Range) -> &mut [T] {
    let start = match range.start_bound() {
        Bound::Excluded(n) => n.saturating_add(1),
        Bound::Included(n) => *n,
        Bound::Unbounded => 0,
    };
    let end = match range.start_bound() {
        Bound::Excluded(n) => *n,
        Bound::Included(n) => n.saturating_add(1),
        Bound::Unbounded => origin.len(),
    };
    &mut origin[start..end]
}

impl<'a, I> UninitializedMemoryGuard<'a, [I]> {
    #[inline]
    pub fn len(&self) -> usize {
        self.memory.len()
    }

    #[inline]
    pub fn slice<Range: RangeBounds<usize>>(self, range: Range) -> Self {
        Self {
            memory: slice(self.memory, range),
        }
    }

    #[inline]
    pub fn init_array<Init: Fn(usize) -> I>(self, init: Init) -> MemoryGuard<'a, [I]> {
        unsafe {
            self.init(|mem| {
                for (index, item) in mem.into_iter().enumerate() {
                    forget(replace(item, init(index)))
                }
            })
        }
    }

    #[inline]
    pub fn init_copy_of(self, source: &[I]) -> MemoryGuard<'a, [I]>
        where I: Clone
    {
        unsafe {
            self.slice(..source.len()).init(|mem| {
                mem.clone_from_slice(source)
            })
        }
    }
}

impl<'a, T: FixedArray> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub fn len(&self) -> usize {
        T::len()
    }

    #[inline]
    pub fn into_slice_guard(self) -> UninitializedMemoryGuard<'a, [T::Item]> {
        UninitializedMemoryGuard {
            memory: self.memory.as_slice_mut(),
        }
    }

    #[inline]
    pub fn slice<Range: RangeBounds<usize>>(self, range: Range) -> UninitializedMemoryGuard<'a, [T::Item]> {
        self.into_slice_guard().slice(range)
    }

    #[inline]
    pub fn init_array<Init: Fn(usize) -> T::Item>(self, init: Init) -> MemoryGuard<'a, [T::Item]> {
        self.into_slice_guard().init_array(init)
    }

    #[inline]
    pub fn init_copy_of(self, source: &[T::Item]) -> MemoryGuard<'a, [T::Item]>
        where T::Item: Clone
    {
        self.into_slice_guard().init_copy_of(source)
    }
}

#[derive(Debug)]
pub struct MemoryGuard<'a, T: ?Sized> {
    memory: &'a mut T,
}
