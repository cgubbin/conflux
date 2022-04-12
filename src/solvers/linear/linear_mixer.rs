/*!
Linear Mixer

This module implements standard linear mixing, possibly with a relaxation parameter
*/

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
/// A simple linear mixer, with controlled relaxation
pub struct LinearMixer<T> {
    /// Relaxation parameter
    beta: T,
    /// Tolerance target
    tol: T,
    /// Maximum iterations
    max_iter: u64,
}

impl<T: FPFloat> std::default::Default for LinearMixer<T> {
    fn default() -> Self {
        LinearMixer::new(T::from_f64(1.).unwrap(), T::from_f64(1e-6).unwrap(), 1000)
    }
}

impl<T: FPFloat> LinearMixer<T> {
    /// Constructor
    pub fn new(beta: T, tol: T, max_iter: u64) -> Self {
        LinearMixer {
            beta,
            tol,
            max_iter,
        }
    }
}

impl<T, Problem> MixerMethods<Problem> for LinearMixer<T>
where
    Problem: FixedPointProblem<Float = T>,
    Problem::Param: FPMul<Problem::Float, Problem::Param>
        + FPAdd<Problem::Param, Problem::Param>
        + FPSub<Problem::Param, Problem::Param>
        + FPNorm<Problem::Float>
        + FPHoldsNaN,
    T: FPFloat,
{
    const NAME: &'static str = "Linear Mixing";

    fn next_iter(
        &mut self,
        op: &mut Problem,
        state: &State<Problem>,
    ) -> Result<IterData<Problem>, FixedPointError<Problem::Float>> {
        let param = state.get_param();
        let output = match op.update(&param) {
            Ok(x) => x,
            Err(_) => return Err(FixedPointError::UpdateFailed),
        };
        let new_param = output
            .mul(&self.beta)
            .add(&param.mul(&(T::from_f64(1.0).unwrap() - self.beta)));

        match new_param.holds_nan() {
            true => return Err(FixedPointError::NumericalDivergence),
            false => (),
        };

        Ok(IterData::new()
            .cost(new_param.sub(&param).norm())
            .param(new_param))
    }

    fn terminate(
        &mut self,
        state: &State<Problem>,
    ) -> Result<TerminationReason, FixedPointError<Problem::Float>> {
        let condition = if state.cost < self.tol {
            TerminationReason::ToleranceBeaten
        } else if state.iter > self.max_iter {
            TerminationReason::HitMaxIterations
        } else {
            TerminationReason::NotTerminated
        };
        Ok(condition)
    }
}
