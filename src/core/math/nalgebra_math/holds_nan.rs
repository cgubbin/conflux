use crate::core::math::FPHoldsNaN;
use nalgebra::{allocator::Allocator, dimension::Dim, DefaultAllocator, OMatrix};
use num_traits::Float;

impl<N, R, C> FPHoldsNaN for OMatrix<N, R, C>
where
    N: Float,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn holds_nan(&self) -> bool {
        self.iter().any(|x| x.is_nan())
    }
}
