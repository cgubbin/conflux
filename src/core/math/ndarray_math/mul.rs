use crate::core::math::FPMul;
use ndarray::{Array1, ScalarOperand};
use num_traits::Float;

impl<T: Float + ScalarOperand + Copy> FPMul<T, Array1<T>> for Array1<T> {
    #[inline]
    fn mul(&self, other: &T) -> Array1<T> {
        self * *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_mul_vec_scalar_ $t>]() {
                    let a = array![1 as $t, 4 as $t, 8 as $t];
                    let b = 2 as $t;
                    let target = array![2 as $t, 8 as $t, 16 as $t];
                    let res = <Array1<$t> as FPMul<$t, Array1<$t>>>::mul(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }
        };
    }

    make_test!(f32);
    make_test!(f64);
}
