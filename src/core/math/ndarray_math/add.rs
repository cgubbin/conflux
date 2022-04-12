use crate::core::math::FPAdd;
use ndarray::Array1;
use num_traits::Float;
use std::ops::AddAssign;

impl<T> FPAdd<Array1<T>, Array1<T>> for Array1<T>
where
    T: Float + AddAssign,
{
    #[inline]
    fn add(&self, other: &Array1<T>) -> Array1<T> {
        self + other
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
                fn [<test_add_vec_vec_ $t>]() {
                    let a = array![1 as $t, 4 as $t, 8 as $t];
                    let b = array![41 as $t, 38 as $t, 34 as $t];
                    let target = array![42 as $t, 42 as $t, 42 as $t];
                    let res = <Array1<$t> as FPAdd<Array1<$t>, Array1<$t>>>::add(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_vec_vec_panic_ $t>]() {
                    let a = array![1 as $t, 4 as $t];
                    let b = array![41 as $t, 38 as $t, 34 as $t];
                    <Array1<$t> as FPAdd<Array1<$t>, Array1<$t>>>::add(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_vec_vec_panic_2_ $t>]() {
                    let a = array![];
                    let b = array![41 as $t, 38 as $t, 34 as $t];
                    <Array1<$t> as FPAdd<Array1<$t>, Array1<$t>>>::add(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_vec_vec_panic_3_ $t>]() {
                    let a = array![41 as $t, 38 as $t, 34 as $t];
                    let b = array![];
                    <Array1<$t> as FPAdd<Array1<$t>, Array1<$t>>>::add(&a, &b);
                }
            }
        };
    }

    make_test!(f32);
    make_test!(f64);
}
