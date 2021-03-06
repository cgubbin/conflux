/**
Tests for running algorithms
*/
use crate::prelude::*;
use miette::Result;
use ndarray::{Array1, Array2};

/// Simple test structure
struct TestCase {
    _f: Array1<f64>,
}

impl TestCase {
    /// Generates the new test structure
    fn new() -> TestCase {
        TestCase {
            _f: Array1::zeros(2),
        }
    }
}

/// Impl of a FixedPointProblem for the testcase
impl FixedPointProblem for TestCase {
    type Output = Array1<f64>;
    type Param = Array1<f64>;
    type Float = f64;
    type Square = Array2<f64>;

    fn update(&mut self, values: &Self::Param) -> Result<Self::Param> {
        let mut out = Array1::zeros(6);
        out[0] = values[0].sin() * values[0].powi(2) + 0.05;
        out[1] = values[1].cos() * values[1].powi(2) + 0.05;
        out[2] = values[0].sin() * values[0].powi(2) + 0.09;
        out[3] = values[1].cos() * values[1].powi(2) + 0.1;
        out[4] = values[0].sin() * values[0].powi(2) + 0.25;
        out[5] = values[1].cos() * values[1].powi(2) + 0.35;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{anderson::Type1AndersonMixer, linear::LinearMixer};

    #[test]
    fn test_linear() {
        let mut cost = TestCase::new();
        let mixer: LinearMixer<f64> = LinearMixer::new(0.1, std::f64::EPSILON, 1000);

        let init: Array1<f64> = Array1::ones(6);
        let mut solver = FixedPointSolver::new(mixer, init);

        let result = solver.run(&mut cost).unwrap();
        println!("{}", result.get_param());
    }

    #[test]
    fn test_stable_anderson() {
        let mut cost = TestCase::new();
        let init: Array1<f64> = Array1::ones(6);
        let mixer = Type1AndersonMixer::new(init.len(), std::f64::EPSILON, 1000);

        let mut solver = FixedPointSolver::new(mixer, init);

        let result = solver.run(&mut cost).unwrap();
        println!("{}", result.get_param());
    }
}
