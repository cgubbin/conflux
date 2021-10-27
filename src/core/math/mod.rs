/**
This module contains overloaded mathematics operators to allow the mixing algorithms to
operate with generic math libraries
*/
mod add;
mod add_ndarray;
mod div;
mod div_ndarray;
mod dot_ndarray;
mod empty_ndarray;
mod eye_ndarray;
mod holds_nan_ndarray;
mod into_2d_ndarray;
mod into_f64;
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
pub use crate::core::math::holds_nan_ndarray::*;
pub use crate::core::math::into_2d_ndarray::*;
pub use crate::core::math::into_f64::*;
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

/// Stacks the vector 'X' under 'self'
pub trait FPStack<X> {
    /// Stacks the vector 'X' under 'self'
    fn stack(&self, other: &X) -> Self;
}

/// Checks whether 'self' is empty
pub trait FPEmpty {
    /// Checks whether 'self' is empty
    fn is_empty(&self) -> bool;
}

/// Creates an identity matrix of dimension 'x'
pub trait FPEye {
    /// Creates an identity matrix of dimension 'x'
    fn eye(x: usize) -> Self;
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

/// Converts self to f64 for tracing
pub trait FPIntof64 {
    /// Converts self into f64 for tracing
    fn cast_f64(&self) -> f64;
}

/// Converts the row vector 'self' into a 2D array
/// Converts the row vector 'self' into a 2D array
pub trait FPInto2D<X> {
    /// Converts the row vector 'self' into a 2D array
    fn into_2d(self) -> X;
}

/// Sub 'X' from 'self'
pub trait FPSub<X, Y> {
    /// Sub 'X' from 'self'
    fn sub(&self, other: &X) -> Y;
}

/// Dot product of 'self' with 'X'
pub trait FPDot<X, Y> {
    /// Dot product of 'self' with 'X'
    fn dot(&self, other: &X) -> Y;
}

/// Transpose of 'self'
pub trait FPTranspose {
    /// Generate the transpose of self
    fn t(&self) -> Self;
}

/// L2 norm of self
pub trait FPNorm<X> {
    /// L2 norm of self
    fn norm(&self) -> X;
}

/// Checks whether the quantity holds any nan
pub trait FPHoldsNaN {
    /// L2 norm of self
    fn holds_nan(&self) -> bool;
}
/// Generates a zero
pub trait FPZeros {
    /// Generates a zero
    fn zeros() -> Self;
}

/// Generates an array of zeros with dim 'x'
pub trait FPFromZeros {
    /// Generates an array of zeros with dim 'x'
    fn zeros(x: usize) -> Self;
}

/// Generates an array of zeros of dim of 'self'
pub trait FPZerosLike {
    /// Generates an array of zeros of dim of 'self'
    fn zeros_like(&self) -> Self;
}
