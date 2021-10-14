use crate::core::math::FPAdd;
use ndarray::{Array1};

macro_rules! make_add {
    ($t:ty) => {
        impl FPAdd<Array1<$t>, Array1<$t>> for Array1<$t> {
            #[inline]
            fn add(&self, other: &Array1<$t>) -> Array1<$t> {
                self + other
            }
        }
    };
}

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
