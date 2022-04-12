use conflux::prelude::*;
use ndarray::{Array1, Array2};
use rand::Rng;

/// Simple test structure
struct TestFunctional {
    x: Array1<f64>,
}

impl TestFunctional {
    /// Generates the new test structure
    fn new(dimension: usize) -> TestFunctional {
        let mut rng = rand::thread_rng();
        TestFunctional {
            x: Array1::from_iter((0..dimension).map(|_| rng.gen())),
        }
    }
}

/// Impl of a FixedPointProblem for the testcase
impl FixedPointProblem for TestFunctional {
    type Output = Array1<f64>;
    type Param = Array1<f64>;
    type Float = f64;
    type Square = Array2<f64>;

    fn update(
        &mut self,
        values: &Self::Param,
    ) -> Result<Self::Param, FixedPointError<Self::Float>> {
        let out = self
            .x
            .iter()
            .zip(values.iter())
            .map(|(x, val)| (val + x).sqrt())
            .collect::<Array1<f64>>();
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use conflux::solvers::{anderson::Type1AndersonMixer, linear::LinearMixer};

    #[test]
    fn test_linear() {
        let dimension = 10;
        let mut cost = TestFunctional::new(dimension);
        let mixer: LinearMixer<f64> = LinearMixer::new(0.1, std::f64::EPSILON, 1000);

        let init: Array1<f64> = Array1::ones(dimension);
        let mut solver = FixedPointSolver::new(mixer, init);

        let result = solver.run(&mut cost);
        assert!(result.is_ok());
        let result = result.unwrap();
        dbg!(result.get_cost());
        dbg!(result.get_param());
        dbg!(result.is_terminated());
        dbg!(result.iteration_count());
    }

    #[test]
    fn test_stable_anderson() {
        let dimension = 10;
        let mut cost = TestFunctional::new(dimension);
        let init: Array1<f64> = Array1::ones(dimension);
        let mixer = Type1AndersonMixer::new(dimension, std::f64::EPSILON, 1000);

        let mut solver = FixedPointSolver::new(mixer, init);

        let result = solver.run(&mut cost);
        assert!(result.is_ok());
        let result = result.unwrap();
        dbg!(result.get_cost());
        dbg!(result.get_param());
        dbg!(result.is_terminated());
        dbg!(result.iteration_count());
    }
}
