use crate::core::math::FPEmpty;

use nalgebra::{DMatrix, DVector};

impl<N> FPEmpty for DVector<N> {
    fn is_empty(&self) -> bool {
        (self.nrows() == 0) & (self.ncols() == 0)
    }
}

impl<N> FPEmpty for DMatrix<N> {
    fn is_empty(&self) -> bool {
        (self.nrows() == 0) & (self.ncols() == 0)
    }
}
