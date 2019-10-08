//! # Inplace it!
//!
//! Place small arrays on the stack with a low cost!
//!
//! The only price you should pay for this is the price of choosing
//! a type based on the size of the requested array! This is just one `match`!
//!
//! ## What?
//!
//! This crate is created for one purpose: allocating small arrays on the stack.
//! The simplest way to use it is:
//!
//! ```rust
//! /*use inplace_it::{inplace_array, guards::UninitializedMemoryGuard};
//!
//! inplace_array(
//!     150, // size of needed array to allocate
//!     |guard: UninitializedMemoryGuard<[usize]>| { // and this is consumer of initialized memory
//!         assert_eq!(160, guard.len());
//!     }
//! )*/
//! ```
//!
//! More details you can find in [inplace_array](fn.inplace_array.html) description.
//!
//! You can also place some other data.
//!
//! ```rust
//! /*use inplace_it::{inplace, guards::{UninitializedMemoryGuard, MemoryGuard}};
//!
//! let source = [1, 2, 3, 4, 10];
//!
//! inplace(
//!     |guard: UninitializedMemoryGuard<(usize, usize)>| { // consumer which will use our allocated array
//!         let initialized_guard: MemoryGuard<(usize, usize)> = guard.init((1, 2));
//!         assert_eq!((1, 2), *initialized_guard);
//!     }
//! );*/
//! ```
//!
//! More details you can find in [inplace_copy_of](fn.inplace_copy_of.html) description.
//!
//! You can also place uninitialized array.
//! This operation is *faster* because of it haven't initializing overhead
//! but you *should* use it with care.
//!
//! ```rust
//! /*use inplace_it::inplace_array_uninitialized;
//!
//! unsafe {
//!     inplace_array_uninitialized(
//!         228, //size of array
//!         4096, // limit of allowed stack allocation in bytes
//!         |memory: &mut [usize]| { // consumer which will use our allocated array
//!             // Unsafely placed array sometimes can be more that you need.
//!             assert!(memory.len() >= 228);
//!             // In secret, the size will be equal to the nearest multiple of 32 (upwards, of course).
//!             assert_eq!(memory.len(), 256);
//!         }
//!     );
//! }*/
//! ```
//!
//! More details you can find in [inplace_array_uninitialized](fn.inplace_array_uninitialized.html) description.
//!
//! ## Why?
//!
//! Because allocation on the stack (i.e. placing variables) is **MUCH FASTER** then usual
//! allocating in the heap.
//!

pub mod guards;
pub mod fixed_array;

use crate::guards::{UninitializedMemoryGuard, UninitializedSliceMemoryGuard};
use std::{
    mem::MaybeUninit,
    intrinsics::transmute
};
use crate::fixed_array::try_inplace_array;

/// `alloc_array` is used when `inplace_array` realize that the size of requested array of `T`
/// is too large and should be replaced in the heap.
///
/// It allocates a vector with `size` elements and fills it up with help of `init` closure
/// and then pass a reference to a slice of the vector into the `consumer` closure.
/// `consumer`'s result will be returned.
#[inline]
pub fn alloc_array<T, R, Consumer: FnOnce(UninitializedSliceMemoryGuard<T>) -> R>(size: usize, consumer: Consumer) -> R {
    unsafe {
        let mut memory_holder = Vec::<T>::with_capacity(size);
        memory_holder.set_len(size);
        let result = consumer(UninitializedSliceMemoryGuard::new(
            transmute::<&mut [T], &mut [MaybeUninit<T>]>(&mut *memory_holder)
        ));
        memory_holder.set_len(0);
        result
    }
}

/// `inplace_array_uninitialized` is unsafe API which is being used by `inplace_array` and
/// `inplace_copy_of` internally.
///  It's trying to place an array of `T` on the stack and pass the reference to it into the
/// `consumer` closure.
/// `size` argument sets the requested size of an array.
/// `consumer`'s result will be returned.
///
/// If the result of array of `T` is more than `limit` (or it's size is more than 4096)
/// then the vector will be allocated in the heap and will be passed as a
/// reference instead of stack-based fixed-size array.
///
/// Sometimes size of allocated array might be more than requested. For sizes larger than 32,
/// the following formula is used: `roundUp(size/32)*32`. This is a simplification that used
/// for keeping code short, simple and able to optimize.
/// For example, for requested 50 item `[T; 64]` will be allocated.
/// For 120 items - `[T; 128]` and so on.
///
/// Note that rounding size up is working for fixed-sized arrays only. If function decides to
/// allocate a vector then its size will be equal to requested.
///
/// # Safety
///
/// It uses `core::mem::uninitialized` under the hood so placed memory is not initialized
/// and it is not safe to use this directly. You it with care, please.
///
/// Also `inplace_array_uninitialized` **DO NOT** `drop` inplaced memory.
///
/// But this function is **FAST** because it haven't initializing overhead. Really.
///
/// # Examples
///
/// ```rust
/// /*use inplace_it::inplace_array_uninitialized;
///
/// // For sizes <= 32 will be allocated exactly same size array
///
/// for i in 1..32 {
///     unsafe {
///         inplace_array_uninitialized(
///             i, //size of array
///             1024, // limit of allowed stack allocation in bytes
///             |memory: &mut [usize]| { // consumer which will use our allocated array
///                 assert_eq!(memory.len(), i);
///             }
///         );
///     }
/// }
///
/// // For sizes > 32 an array may contains a little more items
///
/// for i in (50..500).step_by(50) {
///     unsafe {
///         inplace_array_uninitialized(
///             i, //size of array
///             2048, // limit of allowed stack allocation in bytes
///             |memory: &mut [u16]| { // consumer which will use our allocated array
///                 let mut j = i / 32;
///                 if (i % 32) != 0 {
///                     j += 1;
///                 }
///                 j *= 32;
///                 assert_eq!(memory.len(), j);
///             }
///         );
///     }
/// }
///
/// // But if size of fixed-size array more than limit then vector of exact size will be allocated
///
/// for i in (50..500).step_by(50) {
///     unsafe {
///         inplace_array_uninitialized(
///             i, //size of array
///             0, // limit of allowed stack allocation in bytes
///             |memory: &mut [usize]| { // consumer which will use our allocated array
///                 assert_eq!(memory.len(), i);
///             }
///         );
///     }
/// }*/
/// ```
#[inline]
pub fn inplace_or_alloc_array<T, R, Consumer>(size: usize, consumer: Consumer) -> R
    where Consumer: FnOnce(UninitializedSliceMemoryGuard<T>) -> R
{
    match try_inplace_array(size, consumer) {
        Ok(result) => result,
        Err(consumer) => alloc_array(size, consumer),
    }
}
