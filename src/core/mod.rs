/*!
Node fixed point iteration tools core module

This crate holds the core functionality of the library.
*/

/// Error Handling
mod errors;
mod math;
mod solver;
mod state;

use num::traits::{Float, FloatConst, FromPrimitive, ToPrimitive};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Display};

pub use errors::*;
pub use math::*;
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
    type Output: Clone + Debug; // + Serialize; // + DeserializeOwned;
    /// The type of the input required by the fixed point iteration
    type Param: Clone + Debug; // + Serialize; // + DeserializeOwned;
    /// Floating point precision
    type Float: FPFloat + Debug;
    /// Type for square matrices
    type Square: Clone + Debug; // + Serialize; // + DeserializeOwned;

    /// Carries out the full self-consistent iteration for the problem
    fn update(
        &mut self,
        _values: &Self::Param,
    ) -> Result<Self::Param, FixedPointError<Self::Float>> {
        Err(FixedPointError::UnimplementedOperation)
    }
}

/// This trait defines the mixer operation. All mixers implement the trait
pub trait MixerMethods<Problem: FixedPointProblem> {
    //: Serialize {
    /// The name of the mixing algorithm
    const NAME: &'static str = "UNDEFINED";

    /// Defines a single iteration of the mixing operation
    fn next_iter(
        &mut self,
        op: &mut Problem,
        state: &State<Problem>,
    ) -> Result<IterData<Problem>, FixedPointError<Problem::Float>>;

    /// Checks whether termination conditions are satisfied
    fn terminate(
        &mut self,
        state: &State<Problem>,
    ) -> Result<TerminationReason, FixedPointError<Problem::Float>>;
}
