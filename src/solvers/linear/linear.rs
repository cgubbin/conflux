/*!
Linear Mixer

This module implements standard linear mixing, possibly with a relaxation parameter
*/

use crate::prelude::*;
use miette::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct LinearMixer<F> {
    /// Relaxation parameter
    beta: F,
    /// Tolerance target
    tol: F,
    /// Maximum iterations
    max_iter: u64,
}

impl<F: FPFloat> std::default::Default for LinearMixer<F> {
    fn default() -> Self {
        LinearMixer::new(
            F::from_f64(1.).unwrap(),
            F::from_f64(1e-6).unwrap(),
            1000
            )
    }
}


impl<F: FPFloat> LinearMixer<F> {
    /// Constructor
    pub fn new(beta: F, tol: F, max_iter: u64) -> Self {
        LinearMixer {
            beta,
            tol,
            max_iter,
        }
    }
}

impl<P, F>  Mixer<P> for LinearMixer<F>
where
    P: FixedPointProblem<Float = F>,
    P::Param: FPMul<P::Float, P::Param>
        + FPAdd<P::Param, P::Param>
        + FPSub<P::Param, P::Param>
        + FPNorm<P::Float>,
    F: FPFloat,
{

    const NAME: &'static str = "Linear Mixing";

    fn next_iter(
        &mut self,
        op: &P,
        state: &State<P>,
    ) -> Result<IterData<P>> {
        let param = state.get_param();
        let output = op.update(&param).expect("Failed to update");
        let new_param = 
            output.mul(&self.beta)
            .add(&param.mul(&(F::from_f64(1.0).unwrap() - self.beta)));
        Ok(IterData::new()
            .cost(new_param.sub(&param).norm())
            .param(new_param)
        )
    }

    fn terminate(
        &mut self,
        state: &State<P>
    ) -> Result<TerminationReason> {
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
