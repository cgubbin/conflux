use crate::core::math::FPDot;
use ndarray::{Array1, Array2};

macro_rules! make_dot {
    ($t:ty) => {
        impl FPDot<Array1<$t>, $t> for Array1<$t> {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> $t {
                ndarray::Array1::dot(self, other)
            }
        }

        impl FPDot<$t, Array1<$t>> for Array1<$t> {
            #[inline]
            fn dot(&self, other: &$t) -> Array1<$t> {
                *other * self
            }
        }

        impl<'a> FPDot<Array1<$t>, Array1<$t>> for $t {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> Array1<$t> {
                other * *self
            }
        }

        impl FPDot<Array2<$t>, Array1<$t>> for Array1<$t> {
            #[inline]
            fn dot(&self, other: &Array2<$t>) -> Array1<$t> {
                self.dot(other)
            }
        }

        impl FPDot<Array1<$t>, Array2<$t>> for Array1<$t> {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> Array2<$t> {
                let mut out = Array2::zeros((self.len(), other.len()));
                for i in 0..self.len() {
                    for j in 0..other.len() {
                        out[(i, j)] = self[i] * other[j];
                    }
                }
                out
            }
        }

        impl FPDot<Array1<$t>, Array1<$t>> for Array2<$t> {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> Array1<$t> {
                ndarray::Array2::dot(self, other)
            }
        }

        impl FPDot<Array2<$t>, Array2<$t>> for Array2<$t> {
            #[inline]
            fn dot(&self, other: &Array2<$t>) -> Array2<$t> {
                ndarray::Array2::dot(self, other)
            }
        }

        impl FPDot<$t, Array2<$t>> for Array2<$t> {
            #[inline]
            fn dot(&self, other: &$t) -> Array2<$t> {
                *other * self
            }
        }

        impl<'a> FPDot<Array2<$t>, Array2<$t>> for $t {
            #[inline]
            fn dot(&self, other: &Array2<$t>) -> Array2<$t> {
                other * *self
            }
        }
    };
}

make_dot!(i8);
make_dot!(u8);
make_dot!(i16);
make_dot!(u16);
make_dot!(i32);
make_dot!(u32);
make_dot!(i64);
make_dot!(u64);
make_dot!(f32);
make_dot!(f64);

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

    make_test!(i8);
    make_test!(u8);
    make_test!(i16);
    make_test!(u16);
    make_test!(i32);
    make_test!(u32);
    make_test!(i64);
    make_test!(u64);
    make_test!(f32);
    make_test!(f64);
}
