use crate::core::FixedPointProblem;
use instant;
use num::traits::float::Float;
use paste::item;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Struct to hold the iteration state
pub struct State<O: FixedPointProblem> {
    /// Current parameter vector
    pub param: O::Param,
    /// Previous parameter vector
    pub prev_param: O::Param,
    /// Current best parameter vector
    pub best_param: O::Param,
    /// Previous best parameter vector
    pub prev_best_param: O::Param,
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

// macro_rules! setter {
//     ($name:ident, $type:ty, $doc:tt) => {
//         #[doc=$doc]
//         pub fn $name(&mut self, $name: $type) -> &mut Self {
//             self.$name = $name;
//             self
//         }
//     };
// }

macro_rules! getter {
    ($name:ident, $type:ty, $doc:tt) => {
        item! {
            #[doc=$doc]
            pub fn [<get_ $name>](&self) -> $type {
                self.$name.clone()
            }
        }
    };
}

macro_rules! ogetter {
    ($name:ident, $type:ty, $doc:tt) => {
        item! {
            #[doc=$doc]
            pub fn [<get_ $name>](&self) -> Option<$type> {
                self.$name.clone()
            }
        }
    };
}

impl<O: FixedPointProblem> State<O> {
    /// Generate a new initial State
    pub fn new(param: O::Param) -> Self {
        State {
            param: param.clone(),
            prev_param: param.clone(),
            best_param: param.clone(),
            prev_best_param: param,
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

    /// Verify whether the solution has terminated
    pub fn terminated(&self) -> bool {
        match self.termination_reason {
            TerminationReason::NotTerminated => false,
            TerminationReason::ToleranceBeaten => true,
            TerminationReason::HitMaxIterations => true,
        }
    }

    /// Set the termination reason
    pub fn termination_reason(&mut self, reason: TerminationReason) {
        self.termination_reason = reason;
    }

    getter!(param, O::Param, "Returns current parameter vector");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Enum for termination conditions
pub enum TerminationReason {
    /// The iteration is in progress
    NotTerminated,
    /// The iteration has converged
    ToleranceBeaten,
    /// The maximum iterations have been reached
    HitMaxIterations,
}

#[derive(Clone, Debug, Default)]
/// Struct for the output of a mixing operation
pub struct IterData<P: FixedPointProblem> {
    /// The parameter at the current step
    param: Option<P::Param>,
    /// The associated cost ||f(x) - x||
    cost: Option<P::Float>,
}

impl<P: FixedPointProblem> IterData<P> {
    /// Creates a new iterdata struct
    pub fn new() -> Self {
        IterData {
            param: None,
            cost: None,
        }
    }

    /// Factory method to set 'param' field
    pub fn param(mut self, param: P::Param) -> Self {
        self.param = Some(param);
        self
    }

    /// Factory method to set 'cost' field
    pub fn cost(mut self, cost: P::Float) -> Self {
        self.cost = Some(cost);
        self
    }

    ogetter!(param, P::Param, "Returns current parameter vector");
    ogetter!(cost, P::Float, "Returns current cost");
}
