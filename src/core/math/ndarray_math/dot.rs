use crate::core::math::FPDot;
use ndarray::{Array1, Array2, ScalarOperand};
use num_traits::Float;

impl<T: Float + ScalarOperand + Copy> FPDot<Array1<T>, T> for Array1<T> {
    #[inline]
    fn dot(&self, other: &Array1<T>) -> T {
        ndarray::Array1::dot(self, other)
    }
}

impl<T: Float + ScalarOperand + Copy> FPDot<T, Array1<T>> for Array1<T> {
    #[inline]
    fn dot(&self, other: &T) -> Array1<T> {
        self * *other
    }
}

impl<'a, T: Float + ScalarOperand + Copy> FPDot<Array1<T>, Array1<T>> for T {
    #[inline]
    fn dot(&self, other: &Array1<T>) -> Array1<T> {
        other * *self
    }
}

impl<T: Float + ScalarOperand + Copy> FPDot<Array2<T>, Array1<T>> for Array1<T> {
    #[inline]
    fn dot(&self, other: &Array2<T>) -> Array1<T> {
        self.dot(other)
    }
}

impl<T: Float + Copy> FPDot<Array1<T>, Array2<T>> for Array1<T> {
    #[inline]
    fn dot(&self, other: &Array1<T>) -> Array2<T> {
        let mut out = Array2::zeros((self.len(), other.len()));
        for i in 0..self.len() {
            for j in 0..other.len() {
                out[(i, j)] = self[i] * other[j];
            }
        }
        out
    }
}

impl<T: Float + ScalarOperand + Copy> FPDot<Array1<T>, Array1<T>> for Array2<T> {
    #[inline]
    fn dot(&self, other: &Array1<T>) -> Array1<T> {
        ndarray::Array2::dot(self, other)
    }
}

impl<T: Float + ScalarOperand + Copy> FPDot<Array2<T>, Array2<T>> for Array2<T> {
    #[inline]
    fn dot(&self, other: &Array2<T>) -> Array2<T> {
        ndarray::Array2::dot(self, other)
    }
}

impl<T: Float + ScalarOperand + Copy> FPDot<T, Array2<T>> for Array2<T> {
    #[inline]
    fn dot(&self, other: &T) -> Array2<T> {
        self * *other
    }
}

impl<'a, T: Float + ScalarOperand + Copy> FPDot<Array2<T>, Array2<T>> for T {
    #[inline]
    fn dot(&self, other: &Array2<T>) -> Array2<T> {
        other * *self
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
                fn [<test_vec_vec_ $t>]() {
                    let a = array![1 as $t, 2 as $t, 3 as $t];
                    let b = array![4 as $t, 5 as $t, 6 as $t];
                    let res: $t = <Array1<$t> as FPDot<Array1<$t>, $t>>::dot(&a, &b);
                    assert!((((res - 32 as $t) as f64).abs()) < std::f64::EPSILON);
                }
            }

            item! {
                #[test]
                fn [<test_vec_scalar_ $t>]() {
                    let a = array![1 as $t, 2 as $t, 3 as $t];
                    let b = 2 as $t;
                    let product: Array1<$t> =
                        <Array1<$t> as FPDot<$t, Array1<$t>>>::dot(&a, &b);
                    let res = array![2 as $t, 4 as $t, 6 as $t];
                    for i in 0..3 {
                        assert!((((res[i] - product[i]) as f64).abs()) < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_scalar_vec_ $t>]() {
                    let a = array![1 as $t, 2 as $t, 3 as $t];
                    let b = 2 as $t;
                    let product: Array1<$t> =
                        <$t as FPDot<Array1<$t>, Array1<$t>>>::dot(&b, &a);
                    let res = array![2 as $t, 4 as $t, 6 as $t];
                    for i in 0..3 {
                        assert!((((res[i] - product[i]) as f64).abs()) < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_mat_vec_ $t>]() {
                    let a = array![1 as $t, 2 as $t, 3 as $t];
                    let b = array![4 as $t, 5 as $t, 6 as $t];
                    let res = array![
                        [4 as $t, 5 as $t, 6 as $t],
                        [8 as $t, 10 as $t, 12 as $t],
                        [12 as $t, 15 as $t, 18 as $t]
                    ];
                    let product: Array2<$t> =
                        <Array1<$t> as FPDot<Array1<$t>, Array2<$t>>>::dot(&a, &b);
                    for i in 0..3 {
                        for j in 0..3 {
                            assert!((((res[(i, j)] - product[(i, j)]) as f64).abs()) < std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                fn [<test_mat_vec_2_ $t>]() {
                    let a = array![
                        [1 as $t, 2 as $t, 3 as $t],
                        [4 as $t, 5 as $t, 6 as $t],
                        [7 as $t, 8 as $t, 9 as $t]
                    ];
                    let b = array![1 as $t, 2 as $t, 3 as $t];
                    let res = array![14 as $t, 32 as $t, 50 as $t];
                    let product: Array1<$t> =
                        <Array2<$t> as FPDot<Array1<$t>, Array1<$t>>>::dot(&a, &b);
                    for i in 0..3 {
                        assert!((((res[i] - product[i]) as f64).abs()) < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_mat_mat_ $t>]() {
                    let a = array![
                        [1 as $t, 2 as $t, 3 as $t],
                        [4 as $t, 5 as $t, 6 as $t],
                        [3 as $t, 2 as $t, 1 as $t]
                    ];
                    let b = array![
                        [3 as $t, 2 as $t, 1 as $t],
                        [6 as $t, 5 as $t, 4 as $t],
                        [2 as $t, 4 as $t, 3 as $t]
                    ];
                    let res = array![
                        [21 as $t, 24 as $t, 18 as $t],
                        [54 as $t, 57 as $t, 42 as $t],
                        [23 as $t, 20 as $t, 14 as $t]
                    ];
                    let product: Array2<$t> =
                        <Array2<$t> as FPDot<Array2<$t>, Array2<$t>>>::dot(&a, &b);
                    for i in 0..3 {
                        for j in 0..3 {
                            assert!((((res[(i, j)] - product[(i, j)]) as f64).abs()) < std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                fn [<test_mat_primitive_ $t>]() {
                    let a = array![
                        [1 as $t, 2 as $t, 3 as $t],
                        [4 as $t, 5 as $t, 6 as $t],
                        [3 as $t, 2 as $t, 1 as $t]
                    ];
                    let res = array![
                        [2 as $t, 4 as $t, 6 as $t],
                        [8 as $t, 10 as $t, 12 as $t],
                        [6 as $t, 4 as $t, 2 as $t]
                    ];
                    let product: Array2<$t> =
                        <Array2<$t> as FPDot<$t, Array2<$t>>>::dot(&a, &(2 as $t));
                    for i in 0..3 {
                        for j in 0..3 {
                            assert!((((res[(i, j)] - product[(i, j)]) as f64).abs()) < std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                fn [<test_primitive_mat_ $t>]() {
                    let a = array![
                        [1 as $t, 2 as $t, 3 as $t],
                        [4 as $t, 5 as $t, 6 as $t],
                        [3 as $t, 2 as $t, 1 as $t]
                    ];
                    let res = array![
                        [2 as $t, 4 as $t, 6 as $t],
                        [8 as $t, 10 as $t, 12 as $t],
                        [6 as $t, 4 as $t, 2 as $t]
                    ];
                    let product: Array2<$t> =
                        <$t as FPDot<Array2<$t>, Array2<$t>>>::dot(&(2 as $t), &a);
                    for i in 0..3 {
                        for j in 0..3 {
                            assert!((((res[(i, j)] - product[(i, j)]) as f64).abs()) < std::f64::EPSILON);
                        }
                    }
                }
            }
        };
    }

    make_test!(f32);
    make_test!(f64);
}
