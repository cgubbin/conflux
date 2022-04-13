use crate::core::math::FPDot;

use num_traits::{One, Zero};

use nalgebra::{ClosedAdd, ClosedMul, DMatrix, DVector, Scalar};

impl<N> FPDot<DMatrix<N>, DMatrix<N>> for N
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul + Copy,
{
    #[inline]
    fn dot(&self, other: &DMatrix<N>) -> DMatrix<N> {
        other * *self
    }
}

impl<N> FPDot<N, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul + Copy,
{
    #[inline]
    fn dot(&self, other: &N) -> DMatrix<N> {
        self * *other
    }
}

impl<N> FPDot<DVector<N>, DVector<N>> for N
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul + Copy,
{
    #[inline]
    fn dot(&self, other: &DVector<N>) -> DVector<N> {
        other * *self
    }
}

impl<N> FPDot<DVector<N>, N> for DVector<N>
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul + Copy,
{
    #[inline]
    fn dot(&self, other: &DVector<N>) -> N {
        self.dot(other)
    }
}

impl<N> FPDot<N, DVector<N>> for DVector<N>
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul + Copy,
{
    #[inline]
    fn dot(&self, other: &N) -> DVector<N> {
        self * *other
    }
}

impl<N> FPDot<DMatrix<N>, DVector<N>> for DVector<N>
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul,
{
    #[inline]
    fn dot(&self, other: &DMatrix<N>) -> DVector<N> {
        let promoted =
            nalgebra::DMatrix::from_iterator(1, self.iter().len(), self.into_iter().cloned());
        let out = promoted * other;
        assert_eq!(out.shape().0, 1);
        assert_eq!(out.shape().1, other.shape().1);
        nalgebra::DVector::from_iterator(other.shape().1, out.into_iter().cloned())
    }
}

impl<N> FPDot<DMatrix<N>, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul,
{
    #[inline]
    fn dot(&self, other: &DMatrix<N>) -> DMatrix<N> {
        self * other
    }
}

impl<N> FPDot<DVector<N>, DVector<N>> for DMatrix<N>
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul + Copy,
{
    #[inline]
    fn dot(&self, other: &DVector<N>) -> DVector<N> {
        self * other
    }
}

impl<N> FPDot<DVector<N>, DMatrix<N>> for DVector<N>
where
    N: Scalar + Zero + One + ClosedAdd + ClosedMul + Copy,
{
    #[inline]
    fn dot(&self, other: &DVector<N>) -> DMatrix<N> {
        let mut out = DMatrix::zeros(self.len(), self.len());
        for (idx, mut row) in out.row_iter_mut().enumerate() {
            for (jdx, val) in row.iter_mut().enumerate() {
                *val = self[jdx] * other[idx];
            }
        }
        out
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
                fn [<test_vec_vec_ $t>]() {
                    let a = DVector::from(vec![1 as $t, 2 as $t, 3 as $t]);
                    let b = DVector::from(vec![4 as $t, 5 as $t, 6 as $t]);
                    let res: $t = <DVector<$t> as FPDot<DVector<$t>, $t>>::dot(&a, &b);
                    assert!((((res - 32 as $t) as f64).abs()) < std::f64::EPSILON);
                }
            }

            item! {
                #[test]
                fn [<test_vec_scalar_ $t>]() {
                    let a = DVector::from(vec![1 as $t, 2 as $t, 3 as $t]);
                    let b = 2 as $t;
                    let product: DVector<$t> =
                        <DVector<$t> as FPDot<$t, DVector<$t>>>::dot(&a, &b);
                    let res = DVector::from(vec![2 as $t, 4 as $t, 6 as $t]);
                    for i in 0..3 {
                        assert!((((res[i] - product[i]) as f64).abs()) < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_scalar_vec_ $t>]() {
                    let a = DVector::from(vec![1 as $t, 2 as $t, 3 as $t]);
                    let b = 2 as $t;
                    let product: DVector<$t> =
                        <$t as FPDot<DVector<$t>, DVector<$t>>>::dot(&b, &a);
                    let res = DVector::from(vec![2 as $t, 4 as $t, 6 as $t]);
                    for i in 0..3 {
                        assert!((((res[i] - product[i]) as f64).abs()) < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_mat_vec_ $t>]() {
                    let a = DVector::from(vec![1 as $t, 2 as $t, 3 as $t]);
                    let b = DVector::from(vec![4 as $t, 5 as $t, 6 as $t]);
                    let res = DMatrix::from_iterator(3, 3, vec![
                        4 as $t, 5 as $t, 6 as $t,
                        8 as $t, 10 as $t, 12 as $t,
                        12 as $t, 15 as $t, 18 as $t
                    ].into_iter());
                    let product: DMatrix<$t> =
                        <DVector<$t> as FPDot<DVector<$t>, DMatrix<$t>>>::dot(&a, &b);
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
                    let a = DMatrix::from_iterator(3, 3, vec![
                        1 as $t, 2 as $t, 3 as $t,
                        4 as $t, 5 as $t, 6 as $t,
                        7 as $t, 8 as $t, 9 as $t
                    ].into_iter()).transpose(); // Transpose as DMatrix fill from iter column by column
                    let b = DVector::from(vec![1 as $t, 2 as $t, 3 as $t]);
                    let res = DVector::from(vec![14 as $t, 32 as $t, 50 as $t]);
                    let product: DVector<$t> =
                        <DMatrix<$t> as FPDot<DVector<$t>, DVector<$t>>>::dot(&a, &b);
                    for i in 0..3 {
                        assert!((((res[i] - product[i]) as f64).abs()) < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_mat_mat_ $t>]() {
                    let a = DMatrix::from_iterator(3, 3, vec![
                        1 as $t, 2 as $t, 3 as $t,
                        4 as $t, 5 as $t, 6 as $t,
                        3 as $t, 2 as $t, 1 as $t
                    ].into_iter()).transpose();
                    let b = DMatrix::from_iterator(3, 3, vec![
                        3 as $t, 2 as $t, 1 as $t,
                        6 as $t, 5 as $t, 4 as $t,
                        2 as $t, 4 as $t, 3 as $t
                    ].into_iter()).transpose();
                    let res = DMatrix::from_iterator(3, 3, vec![
                        21 as $t, 24 as $t, 18 as $t,
                        54 as $t, 57 as $t, 42 as $t,
                        23 as $t, 20 as $t, 14 as $t
                    ].into_iter()).transpose();
                    let product: DMatrix<$t> =
                        <DMatrix<$t> as FPDot<DMatrix<$t>, DMatrix<$t>>>::dot(&a, &b);
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
                    let a = DMatrix::from_iterator(3, 3, vec![
                        1 as $t, 2 as $t, 3 as $t,
                        4 as $t, 5 as $t, 6 as $t,
                        3 as $t, 2 as $t, 1 as $t
                    ].into_iter());
                    let res = DMatrix::from_iterator(3, 3, vec![
                        2 as $t, 4 as $t, 6 as $t,
                        8 as $t, 10 as $t, 12 as $t,
                        6 as $t, 4 as $t, 2 as $t
                    ].into_iter());
                    let product: DMatrix<$t> =
                        <DMatrix<$t> as FPDot<$t, DMatrix<$t>>>::dot(&a, &(2 as $t));
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
                    let a = DMatrix::from_iterator(3, 3, vec![
                        1 as $t, 2 as $t, 3 as $t,
                        4 as $t, 5 as $t, 6 as $t,
                        3 as $t, 2 as $t, 1 as $t
                    ].into_iter());
                    let res = DMatrix::from_iterator(3, 3, vec![
                        2 as $t, 4 as $t, 6 as $t,
                        8 as $t, 10 as $t, 12 as $t,
                        6 as $t, 4 as $t, 2 as $t
                    ].into_iter());
                    let product: DMatrix<$t> =
                        <$t as FPDot<DMatrix<$t>, DMatrix<$t>>>::dot(&(2 as $t), &a);
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
