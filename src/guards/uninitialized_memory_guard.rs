use core::mem::MaybeUninit;
use crate::guards::MemoryGuard;

pub struct UninitializedMemoryGuard<'a, T> {
    memory: &'a mut MaybeUninit<T>,
}

impl<'a, T> UninitializedMemoryGuard<'a, T> {
    #[inline]
    pub unsafe fn new(memory: &'a mut MaybeUninit<T>) -> Self {
        Self { memory }
    }

    #[inline]
    pub unsafe fn unwrap(self) -> &'a mut MaybeUninit<T> {
        self.memory
    }

    #[inline]
    pub fn init(self, value: T) -> MemoryGuard<'a, T> {
        unsafe {
            MemoryGuard::new(self.memory, value)
        }
    }
}
