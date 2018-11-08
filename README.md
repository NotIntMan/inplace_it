# Inplace it!

![Version badge](https://img.shields.io/crates/v/inplace_it.svg)
![License badge](https://img.shields.io/crates/l/inplace_it.svg)

Place small arrays on the stack with a low cost!

The only price you should pay for this is the price of choosing
a type based on the size of the requested array! This is just one `match`!

## What?

This crate is created for one purpose: allocating small arrays on the stack.
The simplest way to use it is:

```rust
use inplace_it::inplace_array;

inplace_array(
    150, // size of needed array to allocate
    4096, // limit in bytes allowed to allocate on the stack
          // if the limit is exceeded then Vec<T> will be used
    |index| index * 2, // initializer will be called for every item in the array
    |memory: &mut [usize]| { // and this is consumer of initialized memory
        assert_eq!(memory.len(), 150);
    }
)
```

You can also place copy of some array.

```rust
use inplace_it::inplace_copy_of;

let source = [1, 2, 3, 4, 10];

inplace_copy_of(
    &source, //source for copy
    4096, // limit of allowed stack allocation in bytes
    |memory: &mut [usize]| { // consumer which will use our allocated array
        // Given reference will contains exactly copy of given array.
        assert_eq!(*memory, source);
    }
);
```

You can also place uninitialized array.
This operation is *faster* because of it haven't initializing overhead
but you *should* use it with care.

```rust
use inplace_it::inplace_array_uninitialized;

unsafe {
    inplace_array_uninitialized(
        228, //size of array
        4096, // limit of allowed stack allocation in bytes
        |memory: &mut [usize]| { // consumer which will use our allocated array
            // Unsafely placed array sometimes can be more that you need.
            assert!(memory.len() >= 228);
            // In secret, the size will be equal to the nearest multiple of 32 (upwards, of course).
            assert_eq!(memory.len(), 256);
        }
    );
}
```

## Why?

Because allocation on the stack (i.e. placing variables) is **MUCH FASTER** then usual
allocating in the heap.

## Moar!

You can read the [API reference](https://docs.rs/inplace_it) for more details
or create an [new issue](https://github.com/NotIntMan/inplace_it/issues/new)
to submit a bug, feature request or just ask a question.
