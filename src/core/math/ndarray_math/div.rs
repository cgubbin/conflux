use crate::core::math::FPDiv;
use ndarray::{Array1, ScalarOperand};
use num_traits::Float;
use std::ops::DivAssign;

impl<T> FPDiv<T, Array1<T>> for Array1<T>
where
    T: Float + ScalarOperand + DivAssign,
{
    #[inline]
    fn div(&self, other: &T) -> Array1<T> {
        self / *other
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
                fn [<test_div_vec_scalar_ $t>]() {
                    let a = array![4 as $t, 16 as $t, 8 as $t];
                    let b = 2 as $t;
                    let target = array![2 as $t, 8 as $t, 4 as $t];
                    let res = <Array1<$t> as FPDiv<$t, Array1<$t>>>::div(&a, &b);
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
