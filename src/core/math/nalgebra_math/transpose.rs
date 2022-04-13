use crate::core::math::FPTranspose;

use nalgebra::{DMatrix, DVector, Scalar};

impl<N> FPTranspose<DMatrix<N>> for DMatrix<N>
where
    N: Scalar,
{
    #[inline]
    fn t(&self) -> DMatrix<N> {
        self.transpose()
    }
}

impl<N> FPTranspose<DVector<N>> for DVector<N>
where
    N: Scalar,
{
    #[inline]
    fn t(&self) -> DVector<N> {
        self.to_owned()
    }
}
