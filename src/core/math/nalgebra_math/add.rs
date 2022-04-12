use crate::core::math::FPAdd;
use nalgebra::{ClosedAdd, DMatrix, DVector, Scalar};

impl<N> FPAdd<N, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + ClosedAdd + Copy,
{
    #[inline]
    fn add(&self, other: &N) -> DMatrix<N> {
        self.add_scalar(*other)
    }
}

impl<N> FPAdd<DMatrix<N>, DMatrix<N>> for N
where
    N: Scalar + ClosedAdd + Copy,
{
    #[inline]
    fn add(&self, other: &DMatrix<N>) -> DMatrix<N> {
        other.add_scalar(*self)
    }
}

impl<N> FPAdd<DMatrix<N>, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + ClosedAdd,
{
    #[inline]
    fn add(&self, other: &DMatrix<N>) -> DMatrix<N> {
        self + other
    }
}

impl<N> FPAdd<N, DVector<N>> for DVector<N>
where
    N: Scalar + ClosedAdd + Copy,
{
    #[inline]
    fn add(&self, other: &N) -> DVector<N> {
        self.add_scalar(*other)
    }
}

impl<N> FPAdd<DVector<N>, DVector<N>> for N
where
    N: Scalar + ClosedAdd + Copy,
{
    #[inline]
    fn add(&self, other: &DVector<N>) -> DVector<N> {
        other.add_scalar(*self)
    }
}

impl<N> FPAdd<DVector<N>, DVector<N>> for DVector<N>
where
    N: Scalar + ClosedAdd,
{
    #[inline]
    fn add(&self, other: &DVector<N>) -> DVector<N> {
        self + other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_add_vec_scalar_ $t>]() {
                    let a = DVector::from(vec![1 as $t, 4 as $t, 8 as $t]);
                    let b = 34 as $t;
                    let target = DVector::from(vec![35 as $t, 38 as $t, 42 as $t]);
                    let res = <DVector<$t> as FPAdd<$t, DVector<$t>>>::add(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_add_scalar_vec_ $t>]() {
                    let a = DVector::from(vec![1 as $t, 4 as $t, 8 as $t]);
                    let b = 34 as $t;
                    let target = DVector::from(vec![35 as $t, 38 as $t, 42 as $t]);
                    let res = <$t as FPAdd<DVector<$t>, DVector<$t>>>::add(&b, &a);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_add_vec_vec_ $t>]() {
                    let a = DVector::from(vec![1 as $t, 4 as $t, 8 as $t]);
                    let b = DVector::from(vec![41 as $t, 38 as $t, 34 as $t]);
                    let target = DVector::from(vec![42 as $t, 42 as $t, 42 as $t]);
                    let res = <DVector<$t> as FPAdd<DVector<$t>, DVector<$t>>>::add(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_vec_vec_panic_ $t>]() {
                    let a = DVector::from_vec(vec![1 as $t, 4 as $t]);
                    let b = DVector::from_vec(vec![41 as $t, 38 as $t, 34 as $t]);
                    <DVector<$t> as FPAdd<DVector<$t>, DVector<$t>>>::add(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_vec_vec_panic_2_ $t>]() {
                    let a = DVector::from_vec(vec![]);
                    let b = DVector::from_vec(vec![41 as $t, 38 as $t, 34 as $t]);
                    <DVector<$t> as FPAdd<DVector<$t>, DVector<$t>>>::add(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_vec_vec_panic_3_ $t>]() {
                    let a = DVector::from_vec(vec![41 as $t, 38 as $t, 34 as $t]);
                    let b = DVector::from_vec(vec![]);
                    <DVector<$t> as FPAdd<DVector<$t>, DVector<$t>>>::add(&a, &b);
                }
            }

            item! {
                #[test]
                fn [<test_add_mat_mat_ $t>]() {
                    let a = DMatrix::from_iterator(2, 3, vec![1 as $t, 4 as $t, 8 as $t,
                        2 as $t, 5 as $t, 9 as $t].into_iter()
                    );
                    let b = DMatrix::from_iterator(2, 3, vec![41 as $t, 38 as $t, 34 as $t,
                        40 as $t, 37 as $t, 33 as $t
                    ].into_iter());
                    let target = DMatrix::from_iterator(2, 3, vec![
                        42 as $t, 42 as $t, 42 as $t,
                        42 as $t, 42 as $t, 42 as $t
                    ]);
                    let res = <DMatrix<$t> as FPAdd<DMatrix<$t>, DMatrix<$t>>>::add(&a, &b);
                    for i in 0..3 {
                        for j in 0..2 {
                        assert!(((target[(j, i)] - res[(j, i)]) as f64).abs() < std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                fn [<test_add_mat_scalar_ $t>]() {
                    let a = DMatrix::from_iterator(2, 3, vec![
                        1 as $t, 4 as $t, 8 as $t,
                        2 as $t, 5 as $t, 9 as $t
                    ].into_iter());
                    let b = 2 as $t;
                    let target = DMatrix::from_iterator(2, 3, vec![
                        3 as $t, 6 as $t, 10 as $t,
                        4 as $t, 7 as $t, 11 as $t
                    ].into_iter());
                    let res = <DMatrix<$t> as FPAdd<$t, DMatrix<$t>>>::add(&a, &b);
                    for i in 0..3 {
                        for j in 0..2 {
                        assert!(((target[(j, i)] - res[(j, i)]) as f64).abs() < std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_mat_mat_panic_2_ $t>]() {
                    let a = DMatrix::from_vec(2, 3, vec![
                        1 as $t, 4 as $t, 8 as $t,
                        2 as $t, 5 as $t, 9 as $t
                    ]);
                    let b = DMatrix::from_vec(1, 2, vec![
                        41 as $t, 38 as $t,
                    ]);
                    <DMatrix<$t> as FPAdd<DMatrix<$t>, DMatrix<$t>>>::add(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_add_mat_mat_panic_3_ $t>]() {
                    let a = DMatrix::from_vec(2, 3, vec![
                        1 as $t, 4 as $t, 8 as $t,
                        2 as $t, 5 as $t, 9 as $t
                    ]);
                    let b = DMatrix::from_vec(0, 0, vec![]);
                    <DMatrix<$t> as FPAdd<DMatrix<$t>, DMatrix<$t>>>::add(&a, &b);
                }
            }
        };
    }

    make_test!(f32);
    make_test!(f64);
}
