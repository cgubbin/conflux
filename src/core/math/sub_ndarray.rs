use crate::core::math::FPSub;
use ndarray::{Array1};

macro_rules! make_sub {
    ($t:ty) => {
        impl FPSub<Array1<$t>, Array1<$t>> for Array1<$t> {
            #[inline]
            fn sub(&self, other: &Array1<$t>) -> Array1<$t> {
                self - other
            }
        }
    };
}

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
