use crate::core::math::FPDiv;
use ndarray::{Array1};

macro_rules! make_div {
    ($t:ty) => {
        impl FPDiv<$t, Array1<$t>> for Array1<$t> {
            #[inline]
            fn div(&self, other: &$t) -> Array1<$t> {
                self / *other
            }
        }
    };
}

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
