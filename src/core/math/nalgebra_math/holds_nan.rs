use crate::core::math::FPHoldsNaN;
use nalgebra::{DMatrix, DVector};
use num_traits::Float;

impl<N> FPHoldsNaN for DMatrix<N>
where
    N: Float,
{
    #[inline]
    fn holds_nan(&self) -> bool {
        self.iter().any(|x| x.is_nan())
    }
}

impl<N> FPHoldsNaN for DVector<N>
where
    N: Float,
{
    #[inline]
    fn holds_nan(&self) -> bool {
        self.iter().any(|x| x.is_nan())
    }
}
