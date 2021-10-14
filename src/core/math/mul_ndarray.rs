use crate::core::math::FPMul;
use ndarray::{Array1};

macro_rules! make_mul {
    ($t:ty) => {
        impl FPMul<$t, Array1<$t>> for Array1<$t> {
            #[inline]
            fn mul(&self, other: &$t) -> Array1<$t> {
                self * *other
            }
        }
    };
}

make_mul!(i8);
make_mul!(u8);
make_mul!(i16);
make_mul!(u16);
make_mul!(i32);
make_mul!(u32);
make_mul!(i64);
make_mul!(u64);
make_mul!(f32);
make_mul!(f64);
