use crate::prelude::*;
use miette::Result;
use ndarray::Array1;

struct TestCase {
    F: Array1<f64>
}

impl TestCase {
    fn new() -> TestCase {
        TestCase {
            F: Array1::zeros(20)
        }
    }
}

impl FixedPointProblem for TestCase {
    type Output = Array1<f64>;

//    fn update(&self) -> Result<Self::Output> {
//        let out = Array1::zeros(20);
//        Ok(out)
//    }
}

#[test]
fn test_init() {
    let cost = TestCase::new();

    let init = Array1::zeros(20);
    Mixer::new(cost, )
}


