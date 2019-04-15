use core::{
    mem::{ManuallyDrop, uninitialized, forget, replace, drop},
    ops::{Bound, Deref, DerefMut, RangeBounds},
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
        consumer(UninitializedMemoryGuard::new(&mut *memory_holder))
    }
}

#[inline]
pub fn alloc_array<T, R, Consumer: FnOnce(UninitializedMemoryGuard<[T]>) -> R>(size: usize, consumer: Consumer) -> R {
    unsafe {
        let mut memory_holder = Vec::with_capacity(size);
        memory_holder.set_len(size);
        let result = consumer(UninitializedMemoryGuard::new(&mut *memory_holder));
        memory_holder.set_len(0);
        result
    }
}

impl<'a, T: ?Sized> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub(crate) unsafe fn new(memory: &'a mut T) -> Self {
        Self { memory }
    }

    #[inline]
    pub unsafe fn unwrap(self) -> &'a mut T {
        self.memory
    }

    #[inline]
    pub fn borrow<'b: 'a>(&'b mut self) -> UninitializedMemoryGuard<'b, T> {
        unsafe { UninitializedMemoryGuard::new(self.memory) }
    }
}

impl<'a, T> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub fn init(self, value: T) -> MemoryGuard<'a, T> {
        forget(replace(self.memory, value));
        MemoryGuard { memory: self.memory }
    }
}

impl<'a, I> UninitializedMemoryGuard<'a, [I]> {
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
    pub fn init_slice<Init: Fn(usize) -> I>(self, init: Init) -> SliceMemoryGuard<'a, I> {
        for (index, item) in self.memory.into_iter().enumerate() {
            unsafe {
                forget(replace(item, init(index)))
            }
        }
        SliceMemoryGuard { memory: self.memory }
    }

    #[inline]
    pub fn init_copy_of(self, source: &[I]) -> SliceMemoryGuard<'a, I>
        where I: Clone
    {
        unsafe {
            self.slice(..source.len()).init_slice(|index| { source[index].clone() })
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
        unsafe { UninitializedMemoryGuard::new(self.memory.as_slice_mut()) }
    }

    #[inline]
    pub fn slice<Range: RangeBounds<usize>>(self, range: Range) -> UninitializedMemoryGuard<'a, [T::Item]> {
        self.into_slice_guard().slice(range)
    }

    #[inline]
    pub fn init_slice<Init: Fn(usize) -> T::Item>(self, init: Init) -> SliceMemoryGuard<'a, T::Item> {
        self.into_slice_guard().init_slice(init)
    }

    #[inline]
    pub fn init_copy_of(self, source: &[T::Item]) -> SliceMemoryGuard<'a, T::Item>
        where T::Item: Clone
    {
        self.into_slice_guard().init_copy_of(source)
    }
}

#[derive(Debug)]
pub struct MemoryGuard<'a, T> {
    memory: &'a mut T,
}

impl<'a, T> Deref for MemoryGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.memory
    }
}

impl<'a, T> DerefMut for MemoryGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.memory
    }
}

impl<'a, T> Drop for MemoryGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        drop(unsafe {
            replace(self.memory, uninitialized())
        });
    }
}

#[derive(Debug)]
pub struct SliceMemoryGuard<'a, T> {
    memory: &'a mut [T],
}

impl<'a, T> Deref for SliceMemoryGuard<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.memory
    }
}

impl<'a, T> DerefMut for SliceMemoryGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.memory
    }
}

impl<'a, T> Drop for SliceMemoryGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        for item in self.memory.into_iter() {
            drop(unsafe {
                replace(item, uninitialized())
            });
        }
    }
}
