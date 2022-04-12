use crate::core::math::FPEmpty;

use nalgebra::{allocator::Allocator, dimension::Dim, DefaultAllocator, OMatrix};

impl<N, R, C> FPEmpty for OMatrix<N, R, C>
where
    R: Dim,
    C: Dim,
    DefaultAllocator: Allocator<N, R, C>,
{
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
