/*!
Type 1 Anderson Mixer

Reference: https://stanford.edu/~boyd/papers/pdf/scs_2.0_v_global.pdf
*/

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Clone, Deserialize, Serialize)]
/// Type 1 Anderson Mixer with stabilisation
pub struct Type1AndersonMixer<T, Problem: FixedPointProblem> {
    dim: usize,
    tol: T,
    regularisation: T,
    safeguard_factor: T,
    iter: u64,
    m: u64,
    max_iter: u64,
    beta: T,
    epsilon: T,
    tau: T,
    memory: u64,
    theta_bar: T,

    /// Internal data
    h_matrix: Problem::Square,
    hv1: Problem::Square,
    hv2: Problem::Square,
    sk_hat: Problem::Param,
    s: Problem::Param,
    fx0: Problem::Param,
    x0: Problem::Param,
    y0: Problem::Param,
    g0: Problem::Param,
    s0: Problem::Param,
    fx1: Problem::Param,
    x1: Problem::Param,
    g1: Problem::Param,
    g_prev: Problem::Param,
    s1: Problem::Param,
    s0_hat: Problem::Param,
    s_history: Problem::Square,
    s_hat_memory: Problem::Square,
    y_tilde: Problem::Param,
    ubar: T,
    n_anderson: u64,
}

impl<T: FPFloat, Problem: FixedPointProblem<Float = T>> std::default::Default
    for Type1AndersonMixer<T, Problem>
where
    Problem::Param: FPFromZeros
        + FPSub<Problem::Param, Problem::Param>
        + FPNorm<Problem::Float>
        + FPMul<Problem::Float, Problem::Param>
        + FPAdd<Problem::Param, Problem::Param>
        + FPDiv<Problem::Float, Problem::Param>
        + FPTranspose<Problem::Param>
        + FPDot<Problem::Param, Problem::Square>
        + FPDot<Problem::Square, Problem::Param>
        + FPDot<Problem::Param, Problem::Float>
        + FPInto2D<Problem::Square>,
    Problem::Square: FPEye
        + FPFromZeros
        + FPEmpty
        + FPTranspose<Problem::Square>
        + FPDot<Problem::Square, Problem::Square>
        + FPDot<Problem::Param, Problem::Param>
        + FPStack<Problem::Param, Problem::Square>,
{
    fn default() -> Self {
        Type1AndersonMixer::new(10, T::from_f64(1e-6).unwrap(), 1000)
    }
}

impl<T: FPFloat, Problem: FixedPointProblem<Float = T>> Type1AndersonMixer<T, Problem>
where
    Problem::Param: FPFromZeros
        + FPSub<Problem::Param, Problem::Param>
        + FPNorm<Problem::Float>
        + FPMul<Problem::Float, Problem::Param>
        + FPAdd<Problem::Param, Problem::Param>
        + FPDiv<Problem::Float, Problem::Param>
        + FPTranspose<Problem::Param>
        + FPDot<Problem::Square, Problem::Param>
        + FPDot<Problem::Param, Problem::Float>
        + FPInto2D<Problem::Square>,
    Problem::Square: FPEye
        + FPFromZeros
        + FPEmpty
        + FPTranspose<Problem::Square>
        + FPDot<Problem::Square, Problem::Square>
        + FPDot<Problem::Param, Problem::Param>
        + FPStack<Problem::Param, Problem::Square>,
{
    /// Generate a new Anderson Mixer with default parameters
    pub fn new(dimension: usize, tolerance: T, max_iter: u64) -> Self {
        Type1AndersonMixer {
            dim: dimension,
            tol: tolerance,
            regularisation: T::from_f64(1.).unwrap(),
            safeguard_factor: T::from_f64(1e6).unwrap(),
            iter: 0,
            m: 0,
            max_iter,
            beta: T::from_f64(1.).unwrap(),
            memory: 5,
            theta_bar: T::from_f64(1e-2).unwrap(),
            epsilon: T::from_f64(1e-6).unwrap(),
            tau: T::from_f64(1e-3).unwrap(),
            h_matrix: Problem::Square::eye(dimension),
            hv1: Problem::Square::zeros(0),
            hv2: Problem::Square::zeros(0),
            sk_hat: Problem::Param::zeros(dimension),
            s: Problem::Param::zeros(dimension),
            fx0: Problem::Param::zeros(dimension),
            x0: Problem::Param::zeros(dimension),
            y0: Problem::Param::zeros(dimension),
            g0: Problem::Param::zeros(dimension),
            s0: Problem::Param::zeros(dimension),
            fx1: Problem::Param::zeros(dimension),
            x1: Problem::Param::zeros(dimension),
            g1: Problem::Param::zeros(dimension),
            g_prev: Problem::Param::zeros(dimension),
            s1: Problem::Param::zeros(dimension),
            s0_hat: Problem::Param::zeros(dimension),
            s_history: Problem::Square::zeros(0),
            s_hat_memory: Problem::Square::zeros(0),
            y_tilde: Problem::Param::zeros(dimension),
            ubar: T::from_f64(0.).unwrap(),
            n_anderson: 0,
        }
    }

    /// Factory method to set the regularisation parameter
    pub fn regularisation(mut self, regularisation: T) -> Self {
        self.regularisation = regularisation;
        self
    }

    /// Factory method to set the safeguard factor
    pub fn safeguard_factor(mut self, safeguard_factor: T) -> Self {
        self.safeguard_factor = safeguard_factor;
        self
    }

    /// Factory method to set the relaxation beta
    pub fn beta(mut self, beta: T) -> Self {
        self.beta = beta;
        self
    }

    /// Factory method to set the memory size for the mixer
    pub fn memory(mut self, memory: u64) -> Self {
        self.memory = memory;
        self
    }

    /// Helper method to initialise for the first iteration
    fn init(
        &mut self,
        op: &mut Problem,
        state: &State<Problem>,
    ) -> Result<(), FixedPointError<Problem::Float>> {
        self.fx0 = match op.update(&state.get_param()) {
            Ok(x) => x,
            Err(_) => return Err(FixedPointError::UpdateFailed),
        };
        self.x0 = state.param.clone();
        self.g0 = self.x0.sub(&self.fx0);
        self.ubar = self.g0.norm();
        self.n_anderson = 0;
        self.iter = 0;
        self.m = 0;
        Ok(())
    }

    /// Helper function for common matrix operation
    fn h_aa(&self, input: &Problem::Param) -> Problem::Param {
        if self.hv1.is_empty() | self.hv2.is_empty() {
            input.clone()
        } else {
            input.add(&self.hv1.t().dot(&self.hv2.dot(input)))
        }
    }

    /// Safeguarding step
    fn safeguard(&mut self, op: &mut Problem) -> Result<(), FixedPointError<Problem::Float>> {
        let ubar0 = self.g0.norm();
        let factor = self.ubar
            * self.safeguard_factor
            * T::from_u64(self.n_anderson + 1)
                .unwrap()
                .powf(-T::from_i64(1).unwrap() - self.epsilon);
        if (self.iter == 0) | (ubar0 <= factor) {
            debug!(iteration = self.iter, "Taking Anderson Step");
            self.n_anderson += 1;
            self.x0 = self.x1.clone();
            self.fx0 = self.fx1.clone();
        } else {
            debug!(iteration = self.iter, "Taking Linear Step");
            self.x1 = self
                .fx0
                .mul(&self.beta)
                .add(&self.x0.mul(&(T::from_f64(1.).unwrap() - self.beta)));
            self.x0 = self.x1.clone();
            self.fx0 = match op.update(&self.x0) {
                Ok(x) => x,
                Err(_) => return Err(FixedPointError::UpdateFailed),
            }
        }
        Ok(())
    }

    /// Powell regularisation step
    fn regularise(&mut self) {
        if self.m <= self.memory {
            self.s0_hat = if !self.s_hat_memory.is_empty() {
                self.s0
                    .sub(&(self.s_hat_memory.dot(&self.s0).t().dot(&self.s_hat_memory)).t())
            } else {
                debug!(iteration = self.iter, "No orthogonalisation");
                self.s0.clone()
            };
            if self.s0_hat.norm() < self.tau * self.s0.norm() {
                self.s0_hat = self.s0.clone();
                self.m = 1;
                self.s_hat_memory = Problem::Square::zeros(0);
                self.hv1 = Problem::Square::zeros(0);
                self.hv2 = Problem::Square::zeros(0);
            }
        } else {
            // memory exceeds
            self.s0_hat = self.s0.clone();
            self.m = 1;
            self.s_hat_memory = Problem::Square::zeros(0);
            self.hv1 = Problem::Square::zeros(0);
            self.hv2 = Problem::Square::zeros(0);
        }
        let norm_s0_hat = self.s0_hat.norm();
        let new_shat_row = self.s0_hat.div(&norm_s0_hat);
        self.s_hat_memory = if self.s_hat_memory.is_empty() {
            new_shat_row.into_2d()
        } else {
            self.s_hat_memory.stack(&new_shat_row)
        };

        let gamma = self
            .s0_hat
            .t()
            .dot(&self.h_aa(&self.y0))
            .div(norm_s0_hat.powi(2));

        let theta = if gamma.abs() >= self.theta_bar {
            T::from_f64(1.).unwrap()
        } else {
            T::from_f64(1.).unwrap()
                - gamma.signum() * self.theta_bar.div(T::from_f64(1.).unwrap() - gamma)
        };

        self.y_tilde = self
            .y0
            .mul(&theta)
            .sub(&self.g_prev.mul(&(T::from_f64(1.).unwrap() - theta)));

        let h_y_tilde = self.h_aa(&self.y_tilde);
        let hvec1 = self.s0.sub(&h_y_tilde);
        let hvec2 = self.h_aa(&self.s).div(&self.s0_hat.dot(&h_y_tilde));

        self.hv1 = if self.hv1.is_empty() {
            hvec1.into_2d()
        } else {
            self.hv1.stack(&hvec1)
        };

        self.hv2 = if self.hv2.is_empty() {
            hvec2.into_2d()
        } else {
            self.hv2.stack(&hvec2)
        };
    }
}

impl<T, Problem> MixerMethods<Problem> for Type1AndersonMixer<T, Problem>
where
    Problem::Param: FPFromZeros
        + FPSub<Problem::Param, Problem::Param>
        + FPNorm<Problem::Float>
        + FPMul<Problem::Float, Problem::Param>
        + FPAdd<Problem::Param, Problem::Param>
        + FPDiv<Problem::Float, Problem::Param>
        + FPTranspose<Problem::Param>
        + FPDot<Problem::Square, Problem::Param>
        + FPDot<Problem::Param, Problem::Float>
        + FPInto2D<Problem::Square>
        + std::fmt::Debug
        + FPHoldsNaN,
    Problem::Square: FPEye
        + FPFromZeros
        + FPEmpty
        + FPTranspose<Problem::Square>
        + FPDot<Problem::Square, Problem::Square>
        + FPDot<Problem::Param, Problem::Param>
        + FPStack<Problem::Param, Problem::Square>,
    Problem: FixedPointProblem<Float = T>,
    T: FPFloat,
{
    const NAME: &'static str = "Type-I Anderson Mixing";

    fn next_iter(
        &mut self,
        op: &mut Problem,
        state: &State<Problem>,
    ) -> Result<IterData<Problem>, FixedPointError<Problem::Float>> {
        if self.iter == 0 {
            match self.init(op, state) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        self.m += 1;
        if self.iter == 0 {
            self.x1 = self.fx0.clone();
        } else {
            self.x1 = self.x0.sub(&self.h_aa(&self.g0));
        }

        self.s0 = self.x1.sub(&self.x0);
        self.fx1 = match op.update(&self.x1) {
            Ok(x) => x,
            Err(_) => return Err(FixedPointError::UpdateFailed),
        };
        self.g1 = self.x1.sub(&self.fx1);
        self.y0 = self.g1.sub(&self.g0);

        match self.safeguard(op) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        // Storing for Powell regularisation step
        self.g_prev = self.g0.clone();
        self.g0 = self.x0.sub(&self.fx0);

        self.regularise();
        let res = self.fx1.sub(&self.x1);
        self.iter += 1;

        match self.x1.holds_nan() {
            true => return Err(FixedPointError::NumericalDivergence),
            false => (),
        };

        Ok(IterData::new().cost(res.norm()).param(self.x1.clone()))
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
