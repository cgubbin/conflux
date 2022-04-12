use crate::core::math::FPEye;

use num_traits::{One, Zero};

use nalgebra::{
    base::{allocator::Allocator, dimension::Dim},
    DefaultAllocator, OMatrix, Scalar,
};

impl<N, R, C> FPEye for OMatrix<N, R, C>
where
    N: Scalar + Zero + One,
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    fn eye(dim: usize) -> OMatrix<N, R, C> {
        Self::identity_generic(R::from_usize(dim), C::from_usize(dim))
    }
}
