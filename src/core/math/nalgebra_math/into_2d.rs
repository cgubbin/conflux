use crate::core::math::FPInto2D;
use nalgebra::RealField;
use nalgebra::{dimension::Dim, Matrix, Storage};

impl<N, R, C, S> FPInto2D<Matrix<N, R, C, S>> for Matrix<N, R, C, S>
where
    N: Clone,
    R: Dim,
    C: Dim,
    S: Storage<N, R, C>,
{
    #[inline]
    fn into_2d(self) -> Matrix<N, R, C, S> {
        self
    }
}

use nalgebra::{DMatrix, DVector};

impl<N> FPInto2D<DMatrix<N>> for DVector<N>
where
    N: RealField + Copy,
{
    #[inline]
    fn into_2d(self) -> DMatrix<N> {
        let n_points = self.iter().count();
        let iter = self.iter().map(|x| *x);
        DMatrix::from_iterator(1, n_points, iter)
    }
}
