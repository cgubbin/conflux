use crate::core::FixedPointProblem;
use num::traits::float::Float;
use serde::{Deserialize, Serialize};
use instant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State<O: FixedPointProblem> {
    /// Current parameter vector
    pub param: O::Output,
    /// Previous parameter vector
    pub prev_param: O::Output,
    /// Current best parameter vector
    pub best_param: O::Output,
    /// Previous best parameter vector
    pub prev_best_param: O::Output,
    /// Current cost function
    pub cost: O::Float,
    /// Previous cost
    pub prev_cost: O::Float,
    /// Current best cost
    pub best_cost: O::Float,
    /// Previous best cost
    pub prev_best_cost: O::Float,
    /// Current iteration
    pub iter: u64,
    /// Iteration number of last best cost
    pub last_best_iter: u64,
    /// Maximum number of iterations
    pub max_iters: u64,
    /// Time required as yet
    pub time: Option<instant::Duration>,
    /// Termination reason
    pub termination_reason: TerminationReason,
}

impl <O: FixedPointProblem> State<O> {
    pub fn new(param: O::Output) -> Self {
        State {
            param: param.clone(),
            prev_param: param.clone(),
            best_param: param.clone(),
            prev_best_param: param.clone(),
            cost: O::Float::infinity(),
            prev_cost: O::Float::infinity(),
            best_cost: O::Float::infinity(),
            prev_best_cost: O::Float::infinity(),
            iter: 0,
            last_best_iter: 0,
            max_iters: std::u64::MAX,
            time: Some(instant::Duration::new(0, 0)),
            termination_reason: TerminationReason::NotTerminated,
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TerminationReason {
    NotTerminated
}

