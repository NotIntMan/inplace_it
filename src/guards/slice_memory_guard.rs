use core::{
    ops::{Deref, DerefMut},
    mem::{MaybeUninit, transmute},
    ptr::{drop_in_place, write},
};

pub struct SliceMemoryGuard<'a, T> {
    memory: &'a mut [MaybeUninit<T>],
}

impl<'a, T> SliceMemoryGuard<'a, T> {
    #[inline]
    pub unsafe fn new(memory: &'a mut [MaybeUninit<T>], mut init: impl FnMut(usize) -> T) -> Self {
        for (index, item) in memory.into_iter().enumerate() {
            write(item.as_mut_ptr(), init(index));
        }
        SliceMemoryGuard { memory }
    }
}

impl<'a, T> Deref for SliceMemoryGuard<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<&[MaybeUninit<T>], &[T]>(&self.memory) }
    }
}

impl<'a, T> DerefMut for SliceMemoryGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute::<&mut [MaybeUninit<T>], &mut [T]>(&mut self.memory) }
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
