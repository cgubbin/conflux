//! Node fixed point iteration tools core module
//!
//! This crate holds the core functionality of the library.

/// Error Handling
mod errors;
mod solver;
mod state;

use miette::Result;
use num::traits::{Float, FloatConst, FromPrimitive, ToPrimitive};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Display};

pub use errors::*;
pub use solver::*;
pub use state::*;

/// Trait alias for simplification of common trait bounds
pub trait FPFloat:
    Float + FloatConst + FromPrimitive + ToPrimitive + Debug + Display + Serialize + DeserializeOwned
{
}
impl<I> FPFloat for I where
    I: Float
        + FloatConst
        + FromPrimitive
        + ToPrimitive
        + Debug
        + Display
        + Serialize
        + DeserializeOwned
{
}

/// This trait needs to be implemented for the problem to be solved
///
/// The update method should carry out a full self-consistent iteration
pub trait FixedPointProblem {
    /// The type of the output returned by the fixed point iteration
    type Output: Clone + Serialize + DeserializeOwned;
    /// Floating point precision
    type Float: FPFloat;

    /// Carries out the full self-consistent iteration for the problem
    fn update(&self) -> Result<Self::Output> {
        Err(
            FixedPointError::UnimplementedOperation
        )?
    }
}


/// This trait defines the mixer operation. All mixers implement the trait
pub trait Mixer<O: FixedPointProblem>: Serialize {
    /// The name of the mixing algorithm
    const NAME: &'static str = "UNDEFINED";

    /// Defines a single iteration of the mixing operation
    fn next_iter(
        &mut self
    ) -> Result<O::Output>;

    /// Initializes the mixing algorithm
    fn init(
        &mut self
    ) -> Result<O::Output>;

    /// Checks whether termination conditions are satisfied
    fn terminate(
        &mut self
    ) -> Result<O::Output>;
}
