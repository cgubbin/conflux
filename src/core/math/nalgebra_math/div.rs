use crate::core::math::FPDiv;
use nalgebra::{ClosedDiv, DMatrix, DVector, Scalar};

impl<N> FPDiv<N, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + ClosedDiv + Copy,
{
    #[inline]
    fn div(&self, other: &N) -> DMatrix<N> {
        self / *other
    }
}

impl<N> FPDiv<DMatrix<N>, DMatrix<N>> for N
where
    N: Scalar + ClosedDiv + Copy,
{
    #[inline]
    fn div(&self, other: &DMatrix<N>) -> DMatrix<N> {
        other.map(|entry| *self / entry)
    }
}

impl<N> FPDiv<DMatrix<N>, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + ClosedDiv,
{
    #[inline]
    fn div(&self, other: &DMatrix<N>) -> DMatrix<N> {
        self.component_div(other)
    }
}

impl<N> FPDiv<N, DVector<N>> for DVector<N>
where
    N: Scalar + ClosedDiv + Copy,
{
    #[inline]
    fn div(&self, other: &N) -> DVector<N> {
        self / *other
    }
}

impl<N> FPDiv<DVector<N>, DVector<N>> for N
where
    N: Scalar + ClosedDiv + Copy,
{
    #[inline]
    fn div(&self, other: &DVector<N>) -> DVector<N> {
        other.map(|entry| *self / entry)
    }
}

impl<N> FPDiv<DVector<N>, DVector<N>> for DVector<N>
where
    N: Scalar + ClosedDiv,
{
    #[inline]
    fn div(&self, other: &DVector<N>) -> DVector<N> {
        self.component_div(other)
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
                fn [<test_div_vec_scalar_ $t>]() {
                    let a = DVector::from(vec![4 as $t, 16 as $t, 8 as $t]);
                    let b = 2 as $t;
                    let target = DVector::from(vec![2 as $t, 8 as $t, 4 as $t]);
                    let res = <DVector<$t> as FPDiv<$t, DVector<$t>>>::div(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_scalar_vec_ $t>]() {
                    let a = DVector::from(vec![2 as $t, 4 as $t, 8 as $t]);
                    let b = 32 as $t;
                    let target = DVector::from(vec![16 as $t, 8 as $t, 4 as $t]);
                    let res = <$t as FPDiv<DVector<$t>, DVector<$t>>>::div(&b, &a);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_vec_vec_ $t>]() {
                    let a = DVector::from(vec![4 as $t, 9 as $t, 8 as $t]);
                    let b = DVector::from(vec![2 as $t, 3 as $t, 4 as $t]);
                    let target = DVector::from(vec![2 as $t, 3 as $t, 2 as $t]);
                    let res = <DVector<$t> as FPDiv<DVector<$t>, DVector<$t>>>::div(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_vec_vec_panic_ $t>]() {
                    let a = DVector::from_vec(vec![1 as $t, 4 as $t]);
                    let b = DVector::from_vec(vec![41 as $t, 38 as $t, 34 as $t]);
                    <DVector<$t> as FPDiv<DVector<$t>, DVector<$t>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_vec_vec_panic_2_ $t>]() {
                    let a = DVector::from_vec(vec![]);
                    let b = DVector::from_vec(vec![41 as $t, 38 as $t, 34 as $t]);
                    <DVector<$t> as FPDiv<DVector<$t>, DVector<$t>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_vec_vec_panic_3_ $t>]() {
                    let a = DVector::from_vec(vec![41 as $t, 38 as $t, 34 as $t]);
                    let b = DVector::from_vec(vec![]);
                    <DVector<$t> as FPDiv<DVector<$t>, DVector<$t>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                fn [<test_div_mat_mat_ $t>]() {
                    let a = DMatrix::from_iterator(2, 3, vec![
                        4 as $t, 12 as $t, 8 as $t,
                        9 as $t, 20 as $t, 45 as $t
                    ].into_iter());
                    let b = DMatrix::from_iterator(2, 3, vec![
                        2 as $t, 3 as $t, 4 as $t,
                        3 as $t, 4 as $t, 5 as $t
                    ].into_iter());
                    let target = DMatrix::from_iterator(2, 3, vec![
                        2 as $t, 4 as $t, 2 as $t,
                        3 as $t, 5 as $t, 9 as $t
                    ].into_iter());
                    let res = <DMatrix<$t> as FPDiv<DMatrix<$t>, DMatrix<$t>>>::div(&a, &b);
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
                fn [<test_div_mat_mat_panic_2_ $t>]() {
                    let a = DMatrix::from_vec(2, 3, vec![
                        1 as $t, 4 as $t, 8 as $t,
                        2 as $t, 5 as $t, 9 as $t
                    ]);
                    let b = DMatrix::from_vec(1, 2, vec![
                        41 as $t, 38 as $t,
                    ]);
                    <DMatrix<$t> as FPDiv<DMatrix<$t>, DMatrix<$t>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_mat_mat_panic_3_ $t>]() {
                    let a = DMatrix::from_vec(2, 3, vec![
                        1 as $t, 4 as $t, 8 as $t,
                        2 as $t, 5 as $t, 9 as $t
                    ]);
                    let b = DMatrix::from_vec(0, 0, vec![]);
                    <DMatrix<$t> as FPDiv<DMatrix<$t>, DMatrix<$t>>>::div(&a, &b);
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
