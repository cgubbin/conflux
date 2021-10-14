use crate::core::math::{FPZeros, FPZerosLike};
use num_complex::Complex;

macro_rules! make_zero {
    ($t:ty) => {
        impl FPZeros for $t {
            #[allow(clippy::cast_lossless)]
            #[inline]
            fn zeros() -> $t {
                0 as $t
            }
        }
        impl FPZerosLike for $t {
            #[allow(clippy::cast_lossless)]
            #[inline]
            fn zeros_like(&self) -> $t {
                0 as $t
            }
        }
    };
}

macro_rules! make_complex_zero {
    ($t:ty) => {
        impl FPZeros for Complex<$t> {
            #[allow(clippy::cast_lossless)]
            #[inline]
            fn zeros() -> Complex<$t> {
                Complex::new(0 as $t, 0 as $t)
            }
        }
        impl FPZerosLike for Complex<$t> {
            #[allow(clippy::cast_lossless)]
            #[inline]
            fn zeros_like(&self) -> Complex<$t> {
                Complex::new(0 as $t, 0 as $t)
            }
        }
    };
}

make_zero!(f32);
make_zero!(f64);
make_zero!(i8);
make_zero!(i16);
make_zero!(i32);
make_zero!(i64);
make_zero!(u8);
make_zero!(u16);
make_zero!(u32);
make_zero!(u64);
make_zero!(isize);
make_zero!(usize);
make_complex_zero!(f32);
make_complex_zero!(f64);
make_complex_zero!(i8);
make_complex_zero!(i16);
make_complex_zero!(i32);
make_complex_zero!(i64);
make_complex_zero!(u8);
make_complex_zero!(u16);
make_complex_zero!(u32);
make_complex_zero!(u64);
make_complex_zero!(isize);
make_complex_zero!(usize);