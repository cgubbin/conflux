use crate::core::math::FPHoldsNaN;
use ndarray::Array1;
use num_traits::Float;

impl<T: Float> FPHoldsNaN for Array1<T> {
    #[inline]
    fn holds_nan(&self) -> bool {
        self.iter().any(|&x| x.is_nan())
    }
}
