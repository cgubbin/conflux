use crate::core::math::{FPZeros, FPZerosLike, FPFromZeros};
use num::Zero;

impl<T> FPZerosLike for ndarray::Array1<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros_like(&self) -> ndarray::Array1<T> {
        ndarray::Array1::zeros(self.raw_dim())
    }
}

impl<T> FPZerosLike for ndarray::Array2<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros_like(&self) -> ndarray::Array2<T> {
        ndarray::Array2::zeros(self.raw_dim())
    }
}

impl<T> FPFromZeros for ndarray::Array1<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros(dim: usize) -> ndarray::Array1<T> {
        ndarray::Array1::zeros(dim)
    }
}

impl<T> FPFromZeros for ndarray::Array2<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros(dim: usize) -> ndarray::Array2<T> {
        ndarray::Array2::zeros((dim, dim))
    }

    // #[inline]
    // fn zero() -> ndarray::Array2<T> {
    //     ndarray::Array2::zeros((0, 0))
    // }
}
