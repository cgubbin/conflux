use crate::core::math::FPSub;
use num_complex::Complex;

macro_rules! make_sub {
    ($t:ty) => {
        impl FPSub<$t, $t> for $t {
            #[inline]
            fn sub(&self, other: &$t) -> $t {
                self - other
            }
        }
    };
}

make_sub!(isize);
make_sub!(usize);
make_sub!(i8);
make_sub!(u8);
make_sub!(i16);
make_sub!(u16);
make_sub!(i32);
make_sub!(u32);
make_sub!(i64);
make_sub!(u64);
make_sub!(f32);
make_sub!(f64);
make_sub!(Complex<isize>);
make_sub!(Complex<usize>);
make_sub!(Complex<i8>);
make_sub!(Complex<u8>);
make_sub!(Complex<i16>);
make_sub!(Complex<u16>);
make_sub!(Complex<i32>);
make_sub!(Complex<u32>);
make_sub!(Complex<i64>);
make_sub!(Complex<u64>);
make_sub!(Complex<f32>);
make_sub!(Complex<f64>);
