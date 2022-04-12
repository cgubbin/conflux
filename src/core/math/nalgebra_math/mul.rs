use crate::core::math::FPMul;

use nalgebra::{base::Scalar, ClosedMul, DMatrix, DVector};

impl<N> FPMul<N, DVector<N>> for DVector<N>
where
    N: Scalar + Copy + ClosedMul,
{
    #[inline]
    fn mul(&self, other: &N) -> DVector<N> {
        self * *other
    }
}

impl<N> FPMul<N, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + Copy + ClosedMul,
{
    #[inline]
    fn mul(&self, other: &N) -> DMatrix<N> {
        self * *other
    }
}

impl<N> FPMul<DMatrix<N>, DMatrix<N>> for N
where
    N: Scalar + Copy + ClosedMul,
{
    #[inline]
    fn mul(&self, other: &DMatrix<N>) -> DMatrix<N> {
        other * *self
    }
}

impl<N> FPMul<DVector<N>, DVector<N>> for N
where
    N: Scalar + Copy + ClosedMul,
{
    #[inline]
    fn mul(&self, other: &DVector<N>) -> DVector<N> {
        other * *self
    }
}

impl<N> FPMul<DMatrix<N>, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + ClosedMul,
{
    #[inline]
    fn mul(&self, other: &DMatrix<N>) -> DMatrix<N> {
        self.component_mul(other)
    }
}
