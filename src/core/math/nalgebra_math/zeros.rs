use crate::core::math::{FPFromZeros, FPZerosLike};

use num_traits::Zero;

use nalgebra::{DMatrix, DVector, Scalar};

impl<N> FPZerosLike for DVector<N>
where
    N: Scalar + Zero,
{
    #[inline]
    fn zeros_like(&self) -> DVector<N> {
        Self::zeros(self.nrows())
    }
}

impl<N> FPZerosLike for DMatrix<N>
where
    N: Scalar + Zero,
{
    #[inline]
    fn zeros_like(&self) -> DMatrix<N> {
        Self::zeros(self.nrows(), self.ncols())
    }
}

impl<N> FPFromZeros for DVector<N>
where
    N: Scalar + Zero,
{
    #[inline]
    fn zeros(dim: usize) -> DVector<N> {
        Self::zeros(dim)
    }
}

impl<N> FPFromZeros for DMatrix<N>
where
    N: Scalar + Zero,
{
    #[inline]
    fn zeros(dim: usize) -> DMatrix<N> {
        Self::zeros(dim, dim)
    }
}
