use std::ops::Sub;

use crate::core::math::FPSub;

use nalgebra::{
    base::{allocator::Allocator, dimension::Dim, storage::Storage, Scalar},
    ClosedSub, DefaultAllocator, Matrix, OMatrix,
};

impl<N, R, C, S> FPSub<N, OMatrix<N, R, C>> for Matrix<N, R, C, S>
where
    N: Scalar + Copy + Sub<Output = N>,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn sub(&self, other: &N) -> OMatrix<N, R, C> {
        self.map(|entry| entry - *other)
    }
}

impl<N, R, C, S> FPSub<Matrix<N, R, C, S>, OMatrix<N, R, C>> for N
where
    N: Scalar + Copy + Sub<Output = N>,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn sub(&self, other: &Matrix<N, R, C, S>) -> OMatrix<N, R, C> {
        other.map(|entry| *self - entry)
    }
}

impl<N, R, C, S> FPSub<Matrix<N, R, C, S>, OMatrix<N, R, C>> for Matrix<N, R, C, S>
where
    N: Scalar + ClosedSub,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn sub(&self, other: &Matrix<N, R, C, S>) -> OMatrix<N, R, C> {
        self - other
    }
}
