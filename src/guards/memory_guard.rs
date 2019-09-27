use core::{
    ops::{Deref, DerefMut},
    mem::{MaybeUninit, transmute},
    ptr::{drop_in_place, write},
};

pub struct MemoryGuard<'a, T> {
    memory: &'a mut MaybeUninit<T>,
}

impl<'a, T> MemoryGuard<'a, T> {
    #[inline]
    pub unsafe fn new(memory: &'a mut MaybeUninit<T>, value: T) -> Self {
        write(memory.as_mut_ptr(), value);
        MemoryGuard { memory }
    }
}

impl<'a, T> Deref for MemoryGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<&MaybeUninit<T>, &T>(&self.memory) }
    }
}

impl<'a, T> DerefMut for MemoryGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute::<&mut MaybeUninit<T>, &mut T>(&mut self.memory) }
    }
}

impl<'a, T> Drop for MemoryGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { drop_in_place(self.memory.as_mut_ptr()); }
    }
}
