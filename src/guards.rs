use core::{
    mem::{MaybeUninit},
    ops::{Bound, Deref, DerefMut, RangeBounds},
};
use crate::fixed_array::FixedArray;
use std::ptr::drop_in_place;
use std::intrinsics::transmute;

pub struct UninitializedMemoryGuard<'a, T> {
    memory: &'a mut MaybeUninit<T>,
}

impl<'a, T> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub(crate) fn new(memory: &'a mut MaybeUninit<T>) -> Self {
        Self { memory }
    }

    #[inline]
    pub unsafe fn unwrap(self) -> &'a mut MaybeUninit<T> {
        self.memory
    }

    #[inline]
    pub fn borrow<'b: 'a>(&'b mut self) -> UninitializedMemoryGuard<'b, T> {
        unsafe { UninitializedMemoryGuard::new(self.memory) }
    }

    #[inline]
    pub fn init(self, value: T) -> MemoryGuard<'a, T> {
        unsafe { *self.memory.as_mut_ptr() = value };
        MemoryGuard { memory: self.memory }
    }
}

pub struct UninitializedSliceMemoryGuard<'a, T> {
    memory: &'a mut [MaybeUninit<T>],
}

impl<'a, I> UninitializedSliceMemoryGuard<'a, I> {
    #[inline]
    pub(crate) fn new(memory: &'a mut [MaybeUninit<I>]) -> Self {
        Self { memory }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.memory.len()
    }

    #[inline]
    pub fn slice<Range: RangeBounds<usize>>(self, range: Range) -> Self {
        let start = match range.start_bound() {
            Bound::Excluded(n) => n.saturating_add(1),
            Bound::Included(n) => *n,
            Bound::Unbounded => 0,
        };
        let end = match range.start_bound() {
            Bound::Excluded(n) => *n,
            Bound::Included(n) => n.saturating_add(1),
            Bound::Unbounded => self.memory.len(),
        };
        Self {
            memory: &mut self.memory[start..end],
        }
    }

    #[inline]
    pub fn init<Init: Fn(usize) -> I>(self, init: Init) -> SliceMemoryGuard<'a, I> {
        for (index, item) in self.memory.into_iter().enumerate() {
            unsafe { *item.as_mut_ptr() = init(index); }
        }
        SliceMemoryGuard { memory: self.memory }
    }

    #[inline]
    pub fn init_copy_of(self, source: &[I]) -> SliceMemoryGuard<'a, I>
        where I: Clone
    {
        self.slice(..source.len()).init(|index| { source[index].clone() })
    }
}

impl<'a, T: FixedArray> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub fn len(&self) -> usize {
        T::len()
    }

    #[inline]
    pub fn into_slice_guard(self) -> UninitializedSliceMemoryGuard<'a, T::Item> {
        unsafe { UninitializedSliceMemoryGuard::new(transmute(self.memory)) }
    }

    #[inline]
    pub fn slice<Range: RangeBounds<usize>>(self, range: Range) -> UninitializedSliceMemoryGuard<'a, T::Item> {
        self.into_slice_guard().slice(range)
    }

    #[inline]
    pub fn init_slice<Init: Fn(usize) -> T::Item>(self, init: Init) -> SliceMemoryGuard<'a, T::Item> {
        self.into_slice_guard().init(init)
    }

    #[inline]
    pub fn init_copy_of(self, source: &[T::Item]) -> SliceMemoryGuard<'a, T::Item>
        where T::Item: Clone
    {
        self.into_slice_guard().init_copy_of(source)
    }
}

pub struct MemoryGuard<'a, T> {
    memory: &'a mut MaybeUninit<T>,
}

impl<'a, T> Deref for MemoryGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.memory) }
    }
}

impl<'a, T> DerefMut for MemoryGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.memory) }
    }
}

impl<'a, T> Drop for MemoryGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { drop_in_place(self.memory.as_mut_ptr()); }
    }
}

pub struct SliceMemoryGuard<'a, T> {
    memory: &'a mut [MaybeUninit<T>],
}

impl<'a, T> Deref for SliceMemoryGuard<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.memory) }
    }
}

impl<'a, T> DerefMut for SliceMemoryGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.memory) }
    }
}

impl<'a, T> Drop for SliceMemoryGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        for item in self.memory.into_iter() {
            unsafe { drop_in_place(item.as_mut_ptr()); }
        }
    }
}
