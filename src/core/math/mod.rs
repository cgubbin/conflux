mod add;
mod add_ndarray;
mod div;
mod div_ndarray;
mod dot_ndarray;
mod empty_ndarray;
mod eye_ndarray;
mod into_2d_ndarray;
mod mul;
mod mul_ndarray;
mod norm;
mod norm_ndarray;
mod stack_ndarray;
mod sub;
mod sub_ndarray;
mod transpose_ndarray;
mod zeros;
mod zeros_ndarray;

pub use crate::core::math::add::*;
pub use crate::core::math::add_ndarray::*;
pub use crate::core::math::div::*;
pub use crate::core::math::div_ndarray::*;
pub use crate::core::math::dot_ndarray::*;
pub use crate::core::math::empty_ndarray::*;
pub use crate::core::math::eye_ndarray::*;
pub use crate::core::math::into_2d_ndarray::*;
pub use crate::core::math::mul::*;
pub use crate::core::math::mul_ndarray::*;
pub use crate::core::math::norm::*;
pub use crate::core::math::norm_ndarray::*;
pub use crate::core::math::stack_ndarray::*;
pub use crate::core::math::sub::*;
pub use crate::core::math::sub_ndarray::*;
pub use crate::core::math::transpose_ndarray::*;
pub use crate::core::math::zeros::*;
pub use crate::core::math::zeros_ndarray::*;


/// Add 'X' to 'self'
pub trait FPAdd<X, Y> {
    /// Add 'X' to 'self'
    fn add(&self, other: &X) -> Y;
}

pub trait FPStack<X> {
    fn stack(&self, other: &X) -> Self;
}

pub trait FPEmpty {
    fn is_empty(&self) -> bool;
}
pub trait FPEye {
    fn eye(X: usize) -> Self;
}

/// Divide self by 'X'
pub trait FPDiv<X, Y> {
    /// Divide self by 'X'
    fn div(&self, other: &X) -> Y;
}

/// Multiply (pointwise) 'X' with 'self'
pub trait FPMul<X, Y> {
    /// Multiply (pointwise) 'X' with 'self'
    fn mul(&self, other: &X) -> Y;
}

pub trait FPInto2D<X> {
    fn into_2d(&self) -> X;
}

/// Sub 'X' from 'self'
pub trait FPSub<X, Y> {
    /// Sub 'X' from 'self'
    fn sub(&self, other: &X) -> Y;
}

pub trait FPDot<X, Y> {
    fn dot(&self, other: &X) -> Y;
}
pub trait FPTranspose {
    /// Generate the transpose of self
    fn t(&self) -> Self;
}

/// L2 norm of self
pub trait FPNorm<X> {
    /// L2 norm of self
    fn norm(&self) -> X;
}

pub trait FPZeros {
    fn zeros() -> Self;
}

pub trait FPFromZeros {
    fn zeros(X: usize) -> Self;
}

pub trait FPZerosLike {
    fn zeros_like(&self) -> Self;
}