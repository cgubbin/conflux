use crate::core::math::{FPFromZeros, FPZerosLike};

use num_traits::Zero;

use nalgebra::{
    base::{allocator::Allocator, dimension::Dim},
    DefaultAllocator, OMatrix, Scalar,
};

impl<N, R, C> FPZerosLike for OMatrix<N, R, C>
where
    N: Scalar + Zero,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn zeros_like(&self) -> OMatrix<N, R, C> {
        Self::zeros_generic(R::from_usize(self.nrows()), C::from_usize(self.ncols()))
    }
}

impl<N, R, C> FPFromZeros for OMatrix<N, R, C>
where
    N: Scalar + Zero,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn zeros(dim: usize) -> OMatrix<N, R, C> {
        Self::zeros_generic(R::from_usize(dim), C::from_usize(dim))
    }
}
