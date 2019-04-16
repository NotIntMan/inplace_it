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

macro_rules! impl_fixed_array_for_array {
    ($($length: expr),+) => {
        $(
            impl<T> FixedArray for [T; $length] {
                type Item = T;
                #[inline]
                fn len() -> usize {
                    $length
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

macro_rules! impl_fixed_array_for_array_group_10 {
    ($($length: expr),+) => {
        $(
            impl_fixed_array_for_array!(
                $length, $length + 1, $length + 2, $length + 3, $length + 4,
                $length + 5, $length + 6, $length + 7, $length + 8, $length + 9
            );
        )+
    };
}

macro_rules! impl_fixed_array_for_array_group_100 {
    ($($length: expr),+) => {
        $(
            impl_fixed_array_for_array!(
                $length, $length + 10, $length + 20, $length + 30, $length + 40,
                $length + 50, $length + 60, $length + 70, $length + 80, $length + 90
            );
        )+
    };
}

macro_rules! impl_fixed_array_for_array_group_1_000 {
    ($($length: expr),+) => {
        $(
            impl_fixed_array_for_array_group_100!(
                $length, $length + 100, $length + 200, $length + 300, $length + 400,
                $length + 500, $length + 600, $length + 700, $length + 800, $length + 900
            );
        )+
    };
}

macro_rules! impl_fixed_array_for_array_group_10_000 {
    ($($length: expr),+) => {
        $(
            impl_fixed_array_for_array_group_1_000!(
                $length, $length + 1000, $length + 2000, $length + 3000, $length + 4000,
                $length + 5000, $length + 6000, $length + 7000, $length + 8000, $length + 9000
            );
        )+
    };
}

macro_rules! impl_fixed_array_for_array_group_100_000 {
    ($($length: expr),+) => {
        $(
            impl_fixed_array_for_array_group_10_000!(
                $length, $length + 10000, $length + 20000, $length + 30000, $length + 40000,
                $length + 50000, $length + 60000, $length + 70000, $length + 80000, $length + 90000
            );
        )+
    };
}

macro_rules! impl_fixed_array_for_array_group_1_000_000 {
    ($($length: expr),+) => {
        $(
            impl_fixed_array_for_array_group_100_000!(
                $length, $length + 100000, $length + 200000, $length + 300000, $length + 400000,
                $length + 500000, $length + 600000, $length + 700000, $length + 800000, $length + 900000
            );
        )+
    };
}

#[cfg(target_pointer_width = "8")]
impl_fixed_array_for_array_group_100!(0);

#[cfg(target_pointer_width = "8")]
impl_fixed_array_for_array_group_10!(100, 110);

#[cfg(target_pointer_width = "8")]
impl_fixed_array_for_array!(120, 121, 122, 123, 124, 125, 126, 127);

#[cfg(target_pointer_width = "16")]
impl_fixed_array_for_array_group_10_000!(0, 10000, 20000, 30000, 40000, 50000);

#[cfg(target_pointer_width = "16")]
impl_fixed_array_for_array_group_1_000!(60000, 61000, 62000, 63000, 64000);

#[cfg(target_pointer_width = "16")]
impl_fixed_array_for_array_group_100!(65100, 65200, 65300, 65400);

#[cfg(target_pointer_width = "16")]
impl_fixed_array_for_array_group_10!(65500, 65510, 65520);

#[cfg(target_pointer_width = "16")]
impl_fixed_array_for_array!(65530, 65531, 65532, 65533, 65534, 65535);

#[cfg(target_pointer_width = "32")]
impl_fixed_array_for_array_group_1_000_000!(0);

#[cfg(target_pointer_width = "32")]
impl_fixed_array_for_array_group_10_000!(1000000, 1010000, 1020000, 1030000);

#[cfg(target_pointer_width = "32")]
impl_fixed_array_for_array_group_1_000!(1040000, 1041000, 1042000, 1043000, 1044000, 1045000, 1046000, 1047000);

#[cfg(target_pointer_width = "32")]
impl_fixed_array_for_array_group_100!(1048000, 1048100, 1048200, 1048300, 1048400);

#[cfg(target_pointer_width = "32")]
impl_fixed_array_for_array_group_10!(1048500, 1048510, 1048520, 1048530, 1048540, 1048550, 1048560);

#[cfg(target_pointer_width = "32")]
impl_fixed_array_for_array!(1048570, 1048571, 1048572, 1048573, 1048574, 1048575);

#[cfg(target_pointer_width = "64")]
impl_fixed_array_for_array_group_1_000_000!(0);

#[cfg(target_pointer_width = "64")]
impl_fixed_array_for_array_group_10_000!(1000000, 1010000, 1020000, 1030000);

#[cfg(target_pointer_width = "64")]
impl_fixed_array_for_array_group_1_000!(1040000, 1041000, 1042000, 1043000, 1044000, 1045000, 1046000, 1047000);

#[cfg(target_pointer_width = "64")]
impl_fixed_array_for_array_group_100!(1048000, 1048100, 1048200, 1048300, 1048400);

#[cfg(target_pointer_width = "64")]
impl_fixed_array_for_array_group_10!(1048500, 1048510, 1048520, 1048530, 1048540, 1048550, 1048560);

#[cfg(target_pointer_width = "64")]
impl_fixed_array_for_array!(1048570, 1048571, 1048572, 1048573, 1048574, 1048575);

#[cfg(target_pointer_width = "128")]
impl_fixed_array_for_array_group_1_000_000!(0);

#[cfg(target_pointer_width = "128")]
impl_fixed_array_for_array_group_10_000!(1000000, 1010000, 1020000, 1030000);

#[cfg(target_pointer_width = "128")]
impl_fixed_array_for_array_group_1_000!(1040000, 1041000, 1042000, 1043000, 1044000, 1045000, 1046000, 1047000);

#[cfg(target_pointer_width = "128")]
impl_fixed_array_for_array_group_100!(1048000, 1048100, 1048200, 1048300, 1048400);

#[cfg(target_pointer_width = "128")]
impl_fixed_array_for_array_group_10!(1048500, 1048510, 1048520, 1048530, 1048540, 1048550, 1048560);

#[cfg(target_pointer_width = "128")]
impl_fixed_array_for_array!(1048570, 1048571, 1048572, 1048573, 1048574, 1048575);
