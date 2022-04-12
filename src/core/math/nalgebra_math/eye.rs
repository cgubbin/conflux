use crate::core::math::FPEye;

use num_traits::{One, Zero};

use nalgebra::{DMatrix, Scalar};

impl<N> FPEye for DMatrix<N>
where
    N: Scalar + Zero + One,
{
    #[inline]
    fn eye(dim: usize) -> DMatrix<N> {
        Self::identity(dim, dim)
    }
}
