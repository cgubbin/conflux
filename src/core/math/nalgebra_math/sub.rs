use std::ops::Sub;

use crate::core::math::FPSub;

use nalgebra::{base::Scalar, ClosedSub, DMatrix, DVector};

impl<N> FPSub<N, DVector<N>> for DVector<N>
where
    N: Scalar + Copy + Sub<Output = N>,
{
    #[inline]
    fn sub(&self, other: &N) -> DVector<N> {
        self.map(|entry| entry - *other)
    }
}

impl<N> FPSub<N, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + Copy + Sub<Output = N>,
{
    #[inline]
    fn sub(&self, other: &N) -> DMatrix<N> {
        self.map(|entry| entry - *other)
    }
}

impl<N> FPSub<DVector<N>, DVector<N>> for N
where
    N: Scalar + Copy + Sub<Output = N>,
{
    #[inline]
    fn sub(&self, other: &DVector<N>) -> DVector<N> {
        other.map(|entry| *self - entry)
    }
}

impl<N> FPSub<DMatrix<N>, DMatrix<N>> for N
where
    N: Scalar + Copy + Sub<Output = N>,
{
    #[inline]
    fn sub(&self, other: &DMatrix<N>) -> DMatrix<N> {
        other.map(|entry| *self - entry)
    }
}

impl<N> FPSub<DVector<N>, DVector<N>> for DVector<N>
where
    N: Scalar + ClosedSub,
{
    #[inline]
    fn sub(&self, other: &DVector<N>) -> DVector<N> {
        self - other
    }
}

impl<N> FPSub<DMatrix<N>, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + ClosedSub,
{
    #[inline]
    fn sub(&self, other: &DMatrix<N>) -> DMatrix<N> {
        self - other
    }
}
