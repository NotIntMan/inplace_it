use core::{
    mem::MaybeUninit,
    ops::{
        RangeBounds,
        Bound,
    },
};
use crate::guards::SliceMemoryGuard;

pub struct UninitializedSliceMemoryGuard<'a, T> {
    memory: &'a mut [MaybeUninit<T>],
}

impl<'a, I> UninitializedSliceMemoryGuard<'a, I> {
    #[inline]
    pub unsafe fn new(memory: &'a mut [MaybeUninit<I>]) -> Self {
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
        let end = match range.end_bound() {
            Bound::Excluded(n) => *n,
            Bound::Included(n) => n.saturating_add(1),
            Bound::Unbounded => self.memory.len(),
        };
        Self {
            memory: &mut self.memory[start..end],
        }
    }

    #[inline]
    pub fn init(self, init: impl FnMut(usize) -> I) -> SliceMemoryGuard<'a, I> {
        unsafe {
            SliceMemoryGuard::new(self.memory, init)
        }
    }

    #[inline]
    pub fn init_copy_of(self, source: &[I]) -> SliceMemoryGuard<'a, I>
        where I: Clone
    {
        self.slice(..source.len()).init(|index| { source[index].clone() })
    }
}
