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
//! use inplace_it::inplace_array;
//!
//! inplace_array(
//!     150, // size of needed array to allocate
//!     4096, // limit in bytes allowed to allocate on the stack
//!           // if the limit is exceeded then Vec<T> will be used
//!     |index| index * 2, // initializer will be called for every item in the array
//!     |memory: &mut [usize]| { // and this is consumer of initialized memory
//!         assert_eq!(memory.len(), 150);
//!     }
//! )
//! ```
//!
//! More details you can find in [inplace_array](fn.inplace_array.html) description.
//!
//! You can also place copy of some array.
//!
//! ```rust
//! use inplace_it::inplace_copy_of;
//!
//! let source = [1, 2, 3, 4, 10];
//!
//! inplace_copy_of(
//!     &source, //source for copy
//!     4096, // limit of allowed stack allocation in bytes
//!     |memory: &mut [usize]| { // consumer which will use our allocated array
//!         // Given reference will contains exactly copy of given array.
//!         assert_eq!(*memory, source);
//!     }
//! );
//! ```
//!
//! More details you can find in [inplace_copy_of](fn.inplace_copy_of.html) description.
//!
//! You can also place uninitialized array.
//! This operation is *faster* because of it haven't initializing overhead
//! but you *should* use it with care.
//!
//! ```rust
//! use inplace_it::inplace_array_uninitialized;
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
//! }
//! ```
//!
//! More details you can find in [inplace_array_uninitialized](fn.inplace_array_uninitialized.html) description.
//!
//! ## Why?
//!
//! Because allocation on the stack (i.e. placing variables) is **MUCH FASTER** then usual
//! allocating in the heap.
//!

/// Places uninitialized memory for the `T` type on the stack
/// and passes the reference to it into the `consumer` closure.
///
/// This function is being used for placing fixed-size arrays
/// on the stack in cases when the `type` of array (`[T; $size]` generically)
/// is selects in runtime.
///
/// `R` type is used to pass the result of the `consumer` back when it return
/// control back.
/// `consumer`'s result will be returned.
///
/// # Example
///
/// ```rust
/// use inplace_it::inplace;
/// unsafe {
///     inplace(|memory: &mut [u8; 12]| {
///         for i in 0u8..12 {
///             memory[i as usize] = i;
///         }
///         let mut sum = 0;
///         for i in 0..12 {
///             sum += memory[i];
///         }
///         assert_eq!(sum, 66);
///     });
/// }
/// ```
///
/// # Safety
///
/// Because of some purposes we don't want to initialize the memory allocated
/// on the stack so we use `core::mem::uninitialized` which is unsafe
/// so `inplace` is unsafe too.
#[inline]
pub unsafe fn inplace<T, R, Consumer: Fn(&mut T) -> R>(consumer: Consumer) -> R {
    let mut memory = ::core::mem::uninitialized::<T>();
    consumer(&mut memory)
}

/// This trait is a extended copy of unstable
/// [core::array::FixedSizeArray](core::array::FixedSizeArray).
///
/// This is not a perfect solution. Inheritance from `AsRef<[T]> + AsMut<[T]>` would be preferable.
/// But we until cannot implement `std` traits for `std` types so that inheritance limits us
/// and we cannot use `[T; n]` where `n > 32`.
pub trait FixedArray {
    type Item;
    fn len() -> usize;
    fn as_slice(&self) -> &[Self::Item];
    fn as_slice_mut(&mut self) -> &mut [Self::Item];
}

/// `alloc_array` is used when `inplace_array` realize that the size of requested array of `T`
/// is too large and should be replaced in the heap.
///
/// It allocates a vector with `size` elements and fills it up with help of `init` closure
/// and then pass a reference to a slice of the vector into the `consumer` closure.
/// `consumer`'s result will be returned.
#[inline]
pub unsafe fn alloc_array<T, R, Consumer: Fn(&mut [T]) -> R>(size: usize, consumer: Consumer) -> R {
    let mut memory = Vec::with_capacity(size);
    memory.set_len(size);
    consumer(&mut *memory)
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
/// Also this function is **FAST** because it haven't initializing overhead. Really.
///
/// # Examples
///
/// ```rust
/// use inplace_it::inplace_array_uninitialized;
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
/// }
/// ```
#[inline]
pub unsafe fn inplace_array_uninitialized<
    T,
    R,
    Consumer: Fn(&mut [T]) -> R,
>(size: usize, limit: usize, consumer: Consumer) -> R {
    macro_rules! inplace {
        ($size: expr) => {
            inplace(|memory: &mut [T; $size]| consumer(&mut *memory))
        };
    }
    macro_rules! safe_inplace {
        ($size: expr) => {
            if ::core::mem::size_of::<[T; $size]>() <= limit {
                inplace!($size)
            } else {
                alloc_array::<T, R, Consumer>(size, consumer)
            }
        };
    }
    match size {
        0 => inplace!(0),
        1 => safe_inplace!(1),
        2 => safe_inplace!(2),
        3 => safe_inplace!(3),
        4 => safe_inplace!(4),
        5 => safe_inplace!(5),
        6 => safe_inplace!(6),
        7 => safe_inplace!(7),
        8 => safe_inplace!(8),
        9 => safe_inplace!(9),
        10 => safe_inplace!(10),
        11 => safe_inplace!(11),
        12 => safe_inplace!(12),
        13 => safe_inplace!(13),
        14 => safe_inplace!(14),
        15 => safe_inplace!(15),
        16 => safe_inplace!(16),
        17 => safe_inplace!(17),
        18 => safe_inplace!(18),
        19 => safe_inplace!(19),
        20 => safe_inplace!(20),
        21 => safe_inplace!(21),
        22 => safe_inplace!(22),
        23 => safe_inplace!(23),
        24 => safe_inplace!(24),
        25 => safe_inplace!(25),
        26 => safe_inplace!(26),
        27 => safe_inplace!(27),
        28 => safe_inplace!(28),
        29 => safe_inplace!(29),
        30 => safe_inplace!(30),
        31 => safe_inplace!(31),
        32 => safe_inplace!(32),
        33..=64 => safe_inplace!(64),
        65..=96 => safe_inplace!(96),
        97..=128 => safe_inplace!(128),
        129..=160 => safe_inplace!(160),
        161..=192 => safe_inplace!(192),
        193..=224 => safe_inplace!(224),
        225..=256 => safe_inplace!(256),
        257..=288 => safe_inplace!(288),
        289..=320 => safe_inplace!(320),
        321..=352 => safe_inplace!(352),
        353..=384 => safe_inplace!(384),
        385..=416 => safe_inplace!(416),
        417..=448 => safe_inplace!(448),
        449..=480 => safe_inplace!(480),
        481..=512 => safe_inplace!(512),
        513..=544 => safe_inplace!(544),
        545..=576 => safe_inplace!(576),
        577..=608 => safe_inplace!(608),
        609..=640 => safe_inplace!(640),
        641..=672 => safe_inplace!(672),
        673..=704 => safe_inplace!(704),
        705..=736 => safe_inplace!(736),
        737..=768 => safe_inplace!(768),
        769..=800 => safe_inplace!(800),
        801..=832 => safe_inplace!(832),
        833..=864 => safe_inplace!(864),
        865..=896 => safe_inplace!(896),
        897..=928 => safe_inplace!(928),
        929..=960 => safe_inplace!(960),
        961..=992 => safe_inplace!(992),
        993..=1024 => safe_inplace!(1024),
        1025..=1056 => safe_inplace!(1056),
        1057..=1088 => safe_inplace!(1088),
        1089..=1120 => safe_inplace!(1120),
        1121..=1152 => safe_inplace!(1152),
        1153..=1184 => safe_inplace!(1184),
        1185..=1216 => safe_inplace!(1216),
        1217..=1248 => safe_inplace!(1248),
        1249..=1280 => safe_inplace!(1280),
        1281..=1312 => safe_inplace!(1312),
        1313..=1344 => safe_inplace!(1344),
        1345..=1376 => safe_inplace!(1376),
        1377..=1408 => safe_inplace!(1408),
        1409..=1440 => safe_inplace!(1440),
        1441..=1472 => safe_inplace!(1472),
        1473..=1504 => safe_inplace!(1504),
        1505..=1536 => safe_inplace!(1536),
        1537..=1568 => safe_inplace!(1568),
        1569..=1600 => safe_inplace!(1600),
        1601..=1632 => safe_inplace!(1632),
        1633..=1664 => safe_inplace!(1664),
        1665..=1696 => safe_inplace!(1696),
        1697..=1728 => safe_inplace!(1728),
        1729..=1760 => safe_inplace!(1760),
        1761..=1792 => safe_inplace!(1792),
        1793..=1824 => safe_inplace!(1824),
        1825..=1856 => safe_inplace!(1856),
        1857..=1888 => safe_inplace!(1888),
        1889..=1920 => safe_inplace!(1920),
        1921..=1952 => safe_inplace!(1952),
        1953..=1984 => safe_inplace!(1984),
        1985..=2016 => safe_inplace!(2016),
        2017..=2048 => safe_inplace!(2048),
        2049..=2080 => safe_inplace!(2080),
        2081..=2112 => safe_inplace!(2112),
        2113..=2144 => safe_inplace!(2144),
        2145..=2176 => safe_inplace!(2176),
        2177..=2208 => safe_inplace!(2208),
        2209..=2240 => safe_inplace!(2240),
        2241..=2272 => safe_inplace!(2272),
        2273..=2304 => safe_inplace!(2304),
        2305..=2336 => safe_inplace!(2336),
        2337..=2368 => safe_inplace!(2368),
        2369..=2400 => safe_inplace!(2400),
        2401..=2432 => safe_inplace!(2432),
        2433..=2464 => safe_inplace!(2464),
        2465..=2496 => safe_inplace!(2496),
        2497..=2528 => safe_inplace!(2528),
        2529..=2560 => safe_inplace!(2560),
        2561..=2592 => safe_inplace!(2592),
        2593..=2624 => safe_inplace!(2624),
        2625..=2656 => safe_inplace!(2656),
        2657..=2688 => safe_inplace!(2688),
        2689..=2720 => safe_inplace!(2720),
        2721..=2752 => safe_inplace!(2752),
        2753..=2784 => safe_inplace!(2784),
        2785..=2816 => safe_inplace!(2816),
        2817..=2848 => safe_inplace!(2848),
        2849..=2880 => safe_inplace!(2880),
        2881..=2912 => safe_inplace!(2912),
        2913..=2944 => safe_inplace!(2944),
        2945..=2976 => safe_inplace!(2976),
        2977..=3008 => safe_inplace!(3008),
        3009..=3040 => safe_inplace!(3040),
        3041..=3072 => safe_inplace!(3072),
        3073..=3104 => safe_inplace!(3104),
        3105..=3136 => safe_inplace!(3136),
        3137..=3168 => safe_inplace!(3168),
        3169..=3200 => safe_inplace!(3200),
        3201..=3232 => safe_inplace!(3232),
        3233..=3264 => safe_inplace!(3264),
        3265..=3296 => safe_inplace!(3296),
        3297..=3328 => safe_inplace!(3328),
        3329..=3360 => safe_inplace!(3360),
        3361..=3392 => safe_inplace!(3392),
        3393..=3424 => safe_inplace!(3424),
        3425..=3456 => safe_inplace!(3456),
        3457..=3488 => safe_inplace!(3488),
        3489..=3520 => safe_inplace!(3520),
        3521..=3552 => safe_inplace!(3552),
        3553..=3584 => safe_inplace!(3584),
        3585..=3616 => safe_inplace!(3616),
        3617..=3648 => safe_inplace!(3648),
        3649..=3680 => safe_inplace!(3680),
        3681..=3712 => safe_inplace!(3712),
        3713..=3744 => safe_inplace!(3744),
        3745..=3776 => safe_inplace!(3776),
        3777..=3808 => safe_inplace!(3808),
        3809..=3840 => safe_inplace!(3840),
        3841..=3872 => safe_inplace!(3872),
        3873..=3904 => safe_inplace!(3904),
        3905..=3936 => safe_inplace!(3936),
        3937..=3968 => safe_inplace!(3968),
        3969..=4000 => safe_inplace!(4000),
        4001..=4032 => safe_inplace!(4032),
        4033..=4064 => safe_inplace!(4064),
        4065..=4096 => safe_inplace!(4096),
        n => alloc_array(n, consumer),
    }
}

/// `inplace_array` trying to place an array of `T` on the stack, then initialize it using the
/// `init` closure and finally pass the reference to it into the `consumer` closure.
/// `size` argument sets the requested size of an array.
/// `consumer`'s result will be returned.
///
/// If the result of array of `T` is more than `limit` (or it's size is more than 4096)
/// then the vector will be allocated in the heap and will be initialized and passed as a
/// reference instead of stack-based fixed-size array.
///
/// It's shrink placed/allocated memory by `inplace_array_uninitialized` to requested size
/// so you don't need to worry about extra memory, just use it.
///
/// # Examples
///
/// ```rust
/// use inplace_it::inplace_array;
///
/// for i in (0..500).step_by(25) {
///     inplace_array(
///         i, //size of array
///         1024, // limit of allowed stack allocation in bytes
///         |index| index, // initializer which will be called for every array item,
///         |memory: &mut [usize]| { // consumer which will use our allocated array
///             assert_eq!(memory.len(), i);
///         }
///     );
/// }
/// ```
#[inline]
pub fn inplace_array<
    T,
    R,
    Init: Fn(usize) -> T,
    Consumer: Fn(&mut [T]) -> R,
>(size: usize, limit: usize, init: Init, consumer: Consumer) -> R {
    unsafe {
        inplace_array_uninitialized(size, limit, |memory: &mut [T]| {
            let memory = &mut memory[..size];
            for (index, item) in memory.into_iter().enumerate() {
                *item = init(index);
            }
            consumer(memory)
        })
    }
}

/// `inplace_copy_of` trying to place an array of `T` on the stack, then initialize it by
/// copying from the `source` slice and finally pass the reference to it into the `consumer` closure.
/// Length of `source` argument sets the requested size of an array.
/// `consumer`'s result will be returned.
///
/// If the result of array of `T` is more than `limit` (or it's size is more than 4096)
/// then the vector will be allocated in the heap and will be initialized and passed as a
/// reference instead of stack-based fixed-size array.
///
/// It's shrink placed/allocated memory by `inplace_array_uninitialized` to requested size
/// so you don't need to worry about extra memory, just use it.
///
/// # Examples
///
/// ```rust
/// use inplace_it::{ inplace_array, inplace_copy_of };
///
/// for i in (0..500).step_by(25) {
///     // we use `inplace_array` for generating inputs
///     inplace_array(i, 1024, |index| index, |memory: &mut [usize]| {
///         assert_eq!(memory.len(), i);
///         inplace_copy_of(
///             memory, // source which will be used for setting size and initializing an array
///             1024, // limit of allowed stack allocation in bytes,
///             |memory_copy: &mut [usize]| {
///                 assert_eq!(memory, memory_copy);
///             },
///         );
///     });
/// }
/// ```
#[inline]
pub fn inplace_copy_of<
    T: Clone,
    R,
    Consumer: Fn(&mut [T]) -> R,
>(source: &[T], limit: usize, consumer: Consumer) -> R {
    let source_length = source.len();
    unsafe {
        inplace_array_uninitialized(source_length, limit, |memory: &mut [T]| {
            let memory = &mut memory[..source_length];
            memory.clone_from_slice(source);
            consumer(memory)
        })
    }
}

macro_rules! impl_fixed_array_for_array {
    ($($x: expr),+) => {
        $(
            impl<T> FixedArray for [T; $x] {
                type Item = T;
                #[inline]
                fn len() -> usize {
                    $x
                }
                #[inline]
                fn as_slice(&self) -> &[Self::Item] {
                    self
                }
                #[inline]
                fn as_slice_mut(&mut self) -> &mut [Self::Item] {
                    self
                }
            }
        )+
    };
}

impl_fixed_array_for_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480,
    512, 544, 576, 608, 640, 672, 704, 736, 768, 800, 832, 864, 896, 928, 960, 992, 1024, 1056,
    1088, 1120, 1152, 1184, 1216, 1248, 1280, 1312, 1344, 1376, 1408, 1440, 1472, 1504, 1536, 1568,
    1600, 1632, 1664, 1696, 1728, 1760, 1792, 1824, 1856, 1888, 1920, 1952, 1984, 2016, 2048, 2080,
    2112, 2144, 2176, 2208, 2240, 2272, 2304, 2336, 2368, 2400, 2432, 2464, 2496, 2528, 2560, 2592,
    2624, 2656, 2688, 2720, 2752, 2784, 2816, 2848, 2880, 2912, 2944, 2976, 3008, 3040, 3072, 3104,
    3136, 3168, 3200, 3232, 3264, 3296, 3328, 3360, 3392, 3424, 3456, 3488, 3520, 3552, 3584, 3616,
    3648, 3680, 3712, 3744, 3776, 3808, 3840, 3872, 3904, 3936, 3968, 4000, 4032, 4064, 4096
);
