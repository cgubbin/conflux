use crate::core::math::FPDiv;
use num_complex::Complex;

macro_rules! make_div {
    ($t:ty) => {
        impl FPDiv<$t, $t> for $t {
            #[inline]
            fn div(&self, other: &$t) -> $t {
                self / other
            }
        }
    };
}

make_div!(isize);
make_div!(usize);
make_div!(i8);
make_div!(u8);
make_div!(i16);
make_div!(u16);
make_div!(i32);
make_div!(u32);
make_div!(i64);
make_div!(u64);
make_div!(f32);
make_div!(f64);
make_div!(Complex<isize>);
make_div!(Complex<usize>);
make_div!(Complex<i8>);
make_div!(Complex<u8>);
make_div!(Complex<i16>);
make_div!(Complex<u16>);
make_div!(Complex<i32>);
make_div!(Complex<u32>);
make_div!(Complex<i64>);
make_div!(Complex<u64>);
make_div!(Complex<f32>);
make_div!(Complex<f64>);
