use crate::core::math::FPMul;

use nalgebra::{
    base::{
        allocator::{Allocator, SameShapeAllocator},
        constraint::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint},
        dimension::Dim,
        storage::Storage,
        MatrixSum, Scalar,
    },
    ClosedMul, DefaultAllocator, Matrix, OMatrix,
};

impl<N, R, C, S> FPMul<N, OMatrix<N, R, C>> for Matrix<N, R, C, S>
where
    N: Scalar + Copy + ClosedMul,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn mul(&self, other: &N) -> OMatrix<N, R, C> {
        self * *other
    }
}

impl<N, R, C, S> FPMul<Matrix<N, R, C, S>, OMatrix<N, R, C>> for N
where
    N: Scalar + Copy + ClosedMul,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn mul(&self, other: &Matrix<N, R, C, S>) -> OMatrix<N, R, C> {
        other * *self
    }
}

impl<N, R1, R2, C1, C2, SA, SB> FPMul<Matrix<N, R2, C2, SB>, MatrixSum<N, R1, C1, R2, C2>>
    for Matrix<N, R1, C1, SA>
where
    N: Scalar + ClosedMul,
    R1: Dim,
    R2: Dim,
    C1: Dim,
    C2: Dim,
    SA: Storage<N, R1, C1>,
    SB: Storage<N, R2, C2>,
    DefaultAllocator: SameShapeAllocator<N, R1, C1, R2, C2>,
    ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2>,
{
    #[inline]
    fn mul(&self, other: &Matrix<N, R2, C2, SB>) -> MatrixSum<N, R1, C1, R2, C2> {
        self.component_mul(other)
    }
}
