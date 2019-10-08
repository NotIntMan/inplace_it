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
mod fixed_array;
mod alloc_array;

pub use fixed_array::*;
pub use alloc_array::*;
