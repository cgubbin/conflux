use crate::core::math::FPHoldsNaN;
use ndarray::Array1;

macro_rules! make_holds_nan_float {
    ($t:ty) => {
        impl FPHoldsNaN for Array1<$t> {
            #[inline]
            fn holds_nan(&self) -> bool {
                self.iter().fold(0, |x, y| x + y.is_nan() as i64) > 0
            }
        }
    };
}

make_holds_nan_float!(f32);
make_holds_nan_float!(f64);
