use crate::core::math::FPSub;
use ndarray::{Array1, ScalarOperand};
use num_traits::Float;
use std::ops::SubAssign;

impl<T: Float + ScalarOperand + SubAssign> FPSub<Array1<T>, Array1<T>> for Array1<T> {
    #[inline]
    fn sub(&self, other: &Array1<T>) -> Array1<T> {
        self - other
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
                fn [<test_sub_vec_vec_ $t>]() {
                    let a = array![41 as $t, 38 as $t, 34 as $t];
                    let b = array![1 as $t, 4 as $t, 8 as $t];
                    let target = array![40 as $t, 34 as $t, 26 as $t];
                    let res = <Array1<$t> as FPSub<Array1<$t>, Array1<$t>>>::sub(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_sub_vec_vec_panic_ $t>]() {
                    let a = array![41 as $t, 38 as $t, 34 as $t];
                    let b = array![1 as $t, 4 as $t];
                    <Array1<$t> as FPSub<Array1<$t>, Array1<$t>>>::sub(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_sub_vec_vec_panic_2_ $t>]() {
                    let a = array![];
                    let b = array![41 as $t, 38 as $t, 34 as $t];
                    <Array1<$t> as FPSub<Array1<$t>, Array1<$t>>>::sub(&a, &b);
                }
            }
        };
    }

    make_test!(f32);
    make_test!(f64);
}
