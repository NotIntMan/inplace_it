# Inplace it!

Place small arrays on the stack with a low-cost!

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
        assert!(memory.len() >= 150);
        // sometimes more memory may be placed on the stack than needed
        // but if Vec<T> is used that will never happen
    }
)
```

## Why?

Because allocation on the stack (i.e. placing variables) is **MUCH FASTER** then usual
allocating in the heap.

## Moar!

You can read the [API reference](https://docs.rs/inplace_it) for more details
or create an [new issue](https://github.com/NotIntMan/inplace_it/issues/new)
to submit a bug, feature request or just ask a question.
