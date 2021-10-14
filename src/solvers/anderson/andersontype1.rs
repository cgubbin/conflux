/*!
 Type 1 Anderson Mixer

 Reference: 
 */


use crate::prelude::*;
use miette::Result;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Type1AndersonMixer<F, P: FixedPointProblem> {
    dim: usize,
    tol: F,
    regularisation: F,
    relaxation: F,
    safeguard_factor: F,
    iter: u64,
    m: u64,
    max_iter: u64,
    beta: F,
    epsilon: F,
    tau: F,
    memory: u64,
    theta_bar: F,

    /// Internal data
    h_matrix: P::Square,
    hv1: P::Square,
    hv2: P::Square,
    sk_hat: P::Param,
    s: P::Param,
    fx0: P::Param,
    x0: P::Param,
    y0: P::Param,
    g0: P::Param,
    s0: P::Param,
    fx1: P::Param,
    x1: P::Param,
    g1: P::Param,
    g_prev: P::Param,
    s1: P::Param,
    s0_hat: P::Param,
    s_history: P::Square,
    s_hat_memory: P::Square,
    y_tilde: P::Param,
    Ubar: F,
    n_anderson: u64,
}

impl<F: FPFloat, P: FixedPointProblem<Float = F>> std::default::Default for Type1AndersonMixer<F, P>
where
    P::Param: FPFromZeros
        + FPSub<P::Param, P::Param>
        + FPNorm<P::Float>
        + FPMul<P::Float, P::Param>
        + FPAdd<P::Param, P::Param>
        + FPDiv<P::Float, P::Param>
        + FPTranspose
        + FPDot<P::Param, P::Square> + FPDot<P::Square, P::Param> + FPDot<P::Param, P::Float>
        + FPInto2D<P::Square>,
    P::Square: FPEye
     + FPFromZeros 
     + FPEmpty
     + FPTranspose
     + FPDot<P::Square, P::Square> + FPDot<P::Param, P::Param>
     + FPStack<P::Param>,
{
    fn default() -> Self {
        Type1AndersonMixer::new(10, F::from_f64(1e-6).unwrap(), 1000)
    }
}

impl<F: FPFloat, P: FixedPointProblem<Float = F>> Type1AndersonMixer<F, P>
where
    P::Param: FPFromZeros
        + FPSub<P::Param, P::Param>
        + FPNorm<P::Float>
        + FPMul<P::Float, P::Param>
        + FPAdd<P::Param, P::Param>
        + FPDiv<P::Float, P::Param>
        + FPTranspose
        + FPDot<P::Square, P::Param> + FPDot<P::Param, P::Float>
        + FPInto2D<P::Square>,
    P::Square: FPEye
        + FPFromZeros
        + FPEmpty
        + FPTranspose
        + FPDot<P::Square, P::Square> + FPDot<P::Param, P::Param>
        + FPStack<P::Param>,
{
    pub fn new(dimension: usize, tolerance: F, max_iter: u64) -> Self {
        Type1AndersonMixer {
            dim: dimension,
            tol: tolerance,
            regularisation: F::from_f64(1.).unwrap(),
            relaxation: F::from_f64(1e-6).unwrap(),
            safeguard_factor: F::from_f64(0.1).unwrap(),
            iter: 0,
            m: 0,
            max_iter: max_iter,
            beta: F::from_f64(1.).unwrap(),
            memory: 5,
            theta_bar: F::from_f64(1.).unwrap(),
            epsilon: F::from_f64(1e-4).unwrap(),
            tau: F::from_f64(1e-4).unwrap(),
            h_matrix: P::Square::eye(dimension),
            hv1: P::Square::zeros(0),
            hv2: P::Square::zeros(0),
            sk_hat: P::Param::zeros(dimension),
            s: P::Param::zeros(dimension),
            fx0: P::Param::zeros(dimension),
            x0: P::Param::zeros(dimension),
            y0: P::Param::zeros(dimension),
            g0: P::Param::zeros(dimension),
            s0: P::Param::zeros(dimension),
            fx1: P::Param::zeros(dimension),
            x1: P::Param::zeros(dimension),
            g1: P::Param::zeros(dimension),
            g_prev: P::Param::zeros(dimension),
            s1: P::Param::zeros(dimension),
            s0_hat: P::Param::zeros(dimension),
            s_history: P::Square::zeros(0),
            s_hat_memory: P::Square::zeros(0),
            y_tilde: P::Param::zeros(dimension),
            Ubar: F::from_f64(0.).unwrap(),
            n_anderson: 0,
        }
    }

    pub fn regularisation(mut self, regularisation: F) -> Self {
        self.regularisation = regularisation;
        self
    }

    pub fn relaxation(mut self, relaxation: F) -> Self {
        self.relaxation = relaxation;
        self
    }

    pub fn safeguard_factor(mut self, safeguard_factor: F) -> Self {
        self.safeguard_factor = safeguard_factor;
        self
    }

    pub fn beta(mut self, beta: F) -> Self {
        self.beta = beta;
        self
    }

    pub fn memory(mut self, memory: u64) -> Self {
        self.memory = memory;
        self
    }

    fn init(&mut self, op: &P, state: &State<P>) {
        self.fx0 = op.update(&state.get_param()).expect("Failed to update");
        self.x0 = state.param.clone();
        self.g0 = self.x0.sub(&self.fx0);
        self.Ubar = self.g0.norm();
        self.n_anderson = 0;
        self.iter = 0;
        self.m = 0;
    }

    fn h_aa(&self, input: &P::Param) -> P::Param {
        if self.hv1.is_empty() | self.hv2.is_empty() {
            input.clone()
        } else {
            let tmp = self.hv1.dot(input);
            let test = self.hv1.t().dot(&self.hv2);
            let testb = test.dot(input);
            let tmp = input
                .add(&self.hv1.t().dot(&self.hv2.dot(&self.g0)));
            tmp.clone()
        }
    }

    fn safeguard(&mut self, op: &P) {
        let Ubar0 = self.g0.norm();
        let factor = F::from_u64(self.n_anderson + 1).unwrap().powf(self.epsilon);
        if (self.iter == 1) | (Ubar0 <= self.Ubar * self.safeguard_factor * factor) {
            self.n_anderson += 1;
            self.x0 = self.x1.clone();
            self.fx0 = self.fx1.clone();
        } else {
            self.x1 = self.x0.mul(&self.beta)
                .add(&self.fx0.mul(&(F::from_f64(1.).unwrap() - self.beta)));
            self.x0 = self.x1.clone();
            self.fx0 = op.update(&self.x0).expect("Failed to update");
        }
    }

    fn regularise(&mut self) {
        if self.m <= self.memory {
            self.s0_hat = if !self.s_hat_memory.is_empty() {
                self.s0
                    .sub(&(self.s_hat_memory.dot(&self.s0).t().dot(&self.s_hat_memory)).t())

            } else {
                println!("iter={}, no orthogonalisation", self.iter);
                self.s0.clone()
            };
            if self.s0_hat.norm() < self.tau * self.s0.norm() {
                self.s0_hat = self.s0.clone();
                self.m = 1;
                self.s_hat_memory = P::Square::zeros(0);
                self.hv1 = P::Square::zeros(0);
                self.hv2 = P::Square::zeros(0); 
            }
        } else {
            // memory exceeds
            self.s0_hat = self.s0.clone();
            self.m = 1;
            self.s_hat_memory = P::Square::zeros(0);
            self.hv1 = P::Square::zeros(0);
            self.hv2 = P::Square::zeros(0); 
        }
        let norm_s0_hat = self.s0_hat.norm();
        let new_shat_row = self.s0_hat.div(&norm_s0_hat);
        self.s_hat_memory = if self.s_hat_memory.is_empty() {
            new_shat_row.into_2d()
        } else {
            self.s_hat_memory.stack(&new_shat_row)
        };

        let gamma = self.s0_hat
            .t()
            .dot(&self.h_aa(&self.y0))
            .div(norm_s0_hat.powi(2));

        let theta = if gamma.abs() >= self.theta_bar {
            F::from_f64(1.).unwrap()
        } else {
            F::from_f64(1.).unwrap() - gamma.signum() * self.theta_bar
            .div(F::from_f64(1.).unwrap() - gamma)
        };

        self.y_tilde = self.y0.mul(&theta) 
            .sub(&self.g_prev.mul(&(F::from_f64(1.).unwrap() - theta)));
        
        let h_y_tilde = self.h_aa(&self.y_tilde);
        let hvec1 = self.s0.sub(&h_y_tilde);
        let hvec2 = self.h_aa(&self.s)
            .div(&self.s0_hat.dot(&h_y_tilde));
        
        self.hv1 = if self.hv1.is_empty() {
            hvec1.into_2d()
        } else {
            self.hv1
                .stack(&hvec1)
        };

        self.hv2 = if self.hv2.is_empty() {
            hvec2.into_2d()
        } else {
            self.hv2
                .stack(&hvec2)
        };
    }
}

impl<P, F> Mixer<P> for Type1AndersonMixer<F, P> 
where
    P::Param: FPFromZeros
        + FPSub<P::Param, P::Param>
        + FPNorm<P::Float>
        + FPMul<P::Float, P::Param>
        + FPAdd<P::Param, P::Param>
        + FPDiv<P::Float, P::Param>
        + FPTranspose
        + FPDot<P::Square, P::Param> + FPDot<P::Param, P::Float>
        + FPInto2D<P::Square>,
    P::Square: FPEye
        + FPFromZeros
        + FPEmpty
        + FPTranspose
        + FPDot<P::Square, P::Square> + FPDot<P::Param, P::Param>
        + FPStack<P::Param>,
    P: FixedPointProblem<Float = F>,
    F: FPFloat,
{
    
    const NAME: &'static str = "Type-I Anderson Mixing";

    fn next_iter(
        &mut self,
        op: &P,
        state: &State<P>,
    ) -> Result<IterData<P>> {
        if self.iter == 0 {self.init(op, state);}
        
        self.m += 1;
        if self.iter == 0 {
            self.x1 = self.fx0.clone();
        } else {
            self.x1 = self.x0.sub(&self.h_aa(&self.g0));
        }

        self.s0 = self.x1.sub(&self.x0);
        self.fx1 = op.update(&self.x1).expect("Failed to update");
        self.g1 = self.x1.sub(&self.fx1);
        self.y0 = self.g1.sub(&self.g0);

        self.safeguard(op);

        // Storing for Powell regularisation step
        self.g_prev = self.g0.clone();
        self.g0 = self.x0.sub(&self.fx0);

        self.regularise();
        let res = op.update(&self.x1).unwrap().sub(&self.x1);
        
        Ok(IterData::new()
            .cost(res.norm())
            .param(res)
        )
    }

    fn terminate(&mut self, state: &State<P>) -> Result<TerminationReason> {
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