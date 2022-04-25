use crate::core::{FixedPointError, FixedPointProblem, IterData, MixerMethods, State};
use miette::Result;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{debug, span, warn, Level};

#[derive(Clone, Serialize, Deserialize)]
/// Data type for the solver of a fixed point problem
pub struct FixedPointSolver<Problem: FixedPointProblem, Mixer> {
    /// method
    mixer: Mixer,

    /// data
    data: Problem::Param,

    /// state
    state: State<Problem>,
}

#[derive(Clone, Serialize, Deserialize)]
/// Data type for the result of a fixed point problem
pub struct FixedPointResult<Problem: FixedPointProblem> {
    /// Result
    param: Problem::Param,

    /// Cost
    cost: Problem::Float,

    /// Terminated
    terminated: bool,

    /// Number of iterations
    iteration_count: u64,
}

impl<Problem: FixedPointProblem> FixedPointResult<Problem> {
    /// Return the parameter from the result object
    pub fn get_param(&self) -> Problem::Param {
        self.param.clone()
    }

    /// Return the cost at the end of the iteration run
    pub fn get_cost(&self) -> Problem::Float {
        self.cost
    }

    /// Is the iteration scheme terminated
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }

    /// The total number of fixed point iterations
    pub fn iteration_count(&self) -> u64 {
        self.iteration_count
    }
}

impl<Problem, Mixer> FixedPointSolver<Problem, Mixer>
where
    Problem: FixedPointProblem,
    // P::Float: FPIntof64,
    Problem::Float: std::fmt::Display,
    Mixer: MixerMethods<Problem>,
{
    /// Create a fixed point solver from a Mixer and a starting parameter
    pub fn new(mixer: Mixer, initial_parameter: Problem::Param) -> Self {
        FixedPointSolver {
            mixer,
            data: initial_parameter.clone(),
            state: State::new(initial_parameter),
        }
    }

    /// Run the fixed point solver
    pub fn run(
        &mut self,
        op: &mut Problem,
    ) -> Result<FixedPointResult<Problem>, FixedPointError<Problem::Float>> {
        let span = span!(Level::TRACE, "starting fixed point solver...");
        let _enter = span.enter();
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
            debug!(iteration = self.state.iter, cost = %self.state.cost);
        }

        // See if we hit the maximum iteration number or not
        dbg!(self.state.iter, self.state.max_iters);
        match self.state.iter < self.state.max_iters {
            true => {
                debug!(
                    iterations = self.state.iter,
                    cost = %self.state.cost,
                    "Fixed point iteration converged"
                );
                Ok(self.generate_result())
            }
            false => {
                warn!("Failed to reach required tolerance");
                Err(FixedPointError::TooManyIterations(self.state.cost))
            }
        }
    }

    /// Generates the result from a converged or non-converged solution
    fn generate_result(&self) -> FixedPointResult<Problem> {
        FixedPointResult {
            param: self.state.get_param(),
            cost: self.state.cost,
            terminated: self.state.terminated(),
            iteration_count: self.state.iter,
        }
    }

    /// Updates the State struct based on a single iteration's output
    fn update(&mut self, output: &IterData<Problem>) {
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
