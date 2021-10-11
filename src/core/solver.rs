use crate::core::{FixedPointProblem, Mixer, State};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct FixedPointSolver<O: FixedPointProblem, M> {
    /// method
    mixer: M,

    /// data
    data: O::Output,

    /// state
    state: State<O>,
}

impl<O, M> FixedPointSolver<O, M>
where
    O: FixedPointProblem,
    M: Mixer<O>,
{
    pub fn new(pr: O, mixer: M, initial_parameter: O::Output) -> Self {
        FixedPointSolver {
            mixer,
            data: initial_parameter.clone(),
            state: State::new(initial_parameter),
        }
    }
}
