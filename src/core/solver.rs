use crate::core::{FixedPointProblem, IterData, Mixer, State};
use miette::Result;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize)]
/// Data type for the solver of a fixed point problem
pub struct FixedPointSolver<P: FixedPointProblem, M> {
    /// method
    mixer: M,

    /// data
    data: P::Param,

    /// state
    state: State<P>,
}

#[derive(Clone, Serialize, Deserialize)]
/// Data type for the result of a fixed point problem
pub struct FixedPointResult<P: FixedPointProblem> {
    /// Result
    param: P::Param,

    /// Cost
    cost: P::Float,

    /// Terminated
    terminated: bool,
}

impl<P: FixedPointProblem> FixedPointResult<P> {
    /// Return the parameter from the result object
    pub fn get_param(&self) -> P::Param {
        self.param.clone()
    }
}

impl<P, M> FixedPointSolver<P, M>
where
    P: FixedPointProblem,
    M: Mixer<P>,
{
    /// Create a fixed point solver from a Mixer and a starting parameter
    pub fn new(mixer: M, initial_parameter: P::Param) -> Self {
        FixedPointSolver {
            mixer,
            data: initial_parameter.clone(),
            state: State::new(initial_parameter),
        }
    }

    /// Run the fixed point solver
    pub fn run(&mut self, op: &P) -> Result<FixedPointResult<P>> {
        let running = Arc::new(AtomicBool::new(true));

        while running.load(Ordering::SeqCst) {
            if !self.state.terminated() {
                self.state
                    .termination_reason(self.mixer.terminate(&self.state).unwrap())
            }

            if self.state.terminated() {
                break;
            }

            let output = self.mixer.next_iter(op, &self.state).unwrap();
            self.update(&output);
            println!("Iter: {}, Cost: {}", self.state.iter, self.state.cost);
        }

        Ok(self.generate_result())
    }

    /// Generates the result from a converged or non-converged solution
    fn generate_result(&self) -> FixedPointResult<P> {
        FixedPointResult {
            param: self.state.get_param(),
            cost: self.state.cost,
            terminated: self.state.terminated(),
        }
    }

    /// Updates the State struct based on a single iteration's output
    fn update(&mut self, output: &IterData<P>) {
        self.state.prev_param = self.state.param.clone();
        self.state.param = output.get_param().unwrap();
        self.state.prev_cost = self.state.cost;
        self.state.cost = output.get_cost().unwrap();

        if self.state.cost < self.state.best_cost {
            self.state.prev_best_cost = self.state.best_cost;
            self.state.prev_best_param = self.state.best_param.clone();
            self.state.best_cost = self.state.cost;
            self.state.best_param = self.state.param.clone();
        }

        self.state.iter += 1;
    }
}
