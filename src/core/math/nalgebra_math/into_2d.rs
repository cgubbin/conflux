use crate::core::math::FPInto2D;
use nalgebra::{DMatrix, DVector, RealField};

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
