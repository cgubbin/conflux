use conflux::prelude::*;
use nalgebra::{DMatrix, DVector};
use rand::Rng;

/// Simple test structure
struct TestFunctional {
    x: DVector<f64>,
}

impl TestFunctional {
    /// Generates the new test structure
    fn new(dim: usize) -> TestFunctional {
        let mut rng = rand::thread_rng();
        TestFunctional {
            x: DVector::from_iterator(dim, (0..dim).map(|_| rng.gen())),
        }
    }
}

/// Impl of a FixedPointProblem for the testcase
impl FixedPointProblem for TestFunctional {
    type Output = DVector<f64>;
    type Param = DVector<f64>;
    type Float = f64;
    type Square = DMatrix<f64>;

    fn update(
        &mut self,
        values: &Self::Param,
    ) -> Result<Self::Param, FixedPointError<Self::Float>> {
        let out = self
            .x
            .iter()
            .zip(values.iter())
            .map(|(x, val)| (val + x).sqrt());
        let out = DVector::from_iterator(self.x.len(), out);
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

        let init: DVector<f64> = DVector::from_element(dimension, 1_f64);
        let mut solver = FixedPointSolver::new(mixer, init);

        let result = solver.run(&mut cost);
        assert!(result.is_ok());
        let result = result.unwrap();
        approx::assert_relative_eq!(result.get_cost(), 0_f64);
    }

    #[test]
    fn test_stable_anderson_nalgebra() {
        let dimension = 10;
        let mut cost = TestFunctional::new(dimension);
        let init: DVector<f64> = DVector::from_element(dimension, 1_f64);
        let mixer = Type1AndersonMixer::new(dimension, std::f64::EPSILON, 1000);
        let mut solver = FixedPointSolver::new(mixer, init);
        let result = solver.run(&mut cost);
        assert!(result.is_ok());
        let result = result.unwrap();
        approx::assert_relative_eq!(result.get_cost(), 0_f64);
    }
}
