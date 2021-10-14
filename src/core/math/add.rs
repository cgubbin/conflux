use crate::core::math::FPAdd;
use num_complex::Complex;

macro_rules! make_add {
    ($t:ty) => {
        impl FPAdd<$t, $t> for $t {
            #[inline]
            fn add(&self, other: &$t) -> $t {
                self + other
            }
        }
    };
}
make_add!(isize);
make_add!(usize);
make_add!(i8);
make_add!(u8);
make_add!(i16);
make_add!(u16);
make_add!(i32);
make_add!(u32);
make_add!(i64);
make_add!(u64);
make_add!(f32);
make_add!(f64);
make_add!(Complex<isize>);
make_add!(Complex<usize>);
make_add!(Complex<i8>);
make_add!(Complex<u8>);
make_add!(Complex<i16>);
make_add!(Complex<u16>);
make_add!(Complex<i32>);
make_add!(Complex<u32>);
make_add!(Complex<i64>);
make_add!(Complex<u64>);
make_add!(Complex<f32>);
make_add!(Complex<f64>);
