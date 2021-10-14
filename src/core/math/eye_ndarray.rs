use crate::core::math::{FPEye};
use num::{One, Zero};

impl<T> FPEye for ndarray::Array2<T>
where
    T: Clone + One + Zero
{
    #[inline]
    fn eye(dim: usize) -> ndarray::Array2<T> {
        ndarray::Array2::eye(dim)
    }
}
