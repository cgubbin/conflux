use crate::core::math::FPStack;
use nalgebra::{DMatrix, DVector, Scalar};
use num_traits::Zero;

impl<N> FPStack<DVector<N>, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + Zero + Copy,
{
    #[inline]
    fn stack(&self, other: &DVector<N>) -> DMatrix<N> {
        let trans = self.transpose();
        let values = trans.iter().chain(other.into_iter()).cloned();
        let out: DMatrix<N> =
            DMatrix::from_iterator(self.nrows() + 1, self.ncols(), values).transpose();
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
                fn [<test_stack_nalgebra_ $t>]() {
                    let e: DVector<$t> = DVector::from(vec![1 as $t, 2 as $t, 120 as $t]);
                    let mat: DMatrix<$t> = DMatrix::from_iterator(3, 2, vec![
                        2 as $t, 3 as $t, 4 as $t,
                        3 as $t, 4 as $t, 5 as $t
                    ].into_iter()).transpose();
                    let res = mat.stack(&e);
                    let target: DMatrix<$t> = DMatrix::from_iterator(3, 3, vec![
                        2 as $t, 3 as $t, 4 as $t,
                        3 as $t, 4 as $t, 5 as $t,
                        1 as $t, 2 as $t, 120 as $t
                    ].into_iter()).transpose();
                    println!("{target}");
                    println!("{mat}");
                    println!("{res}");
                    for i in 0..3 {
                        for j in 0..3 {
                            assert!((((res[(i, j)] - target[(i, j)]) as f64).abs()) < std::f64::EPSILON);
                        }
                    }
                }
            }
        };
    }

    // make_test!(isize);
    // make_test!(usize);
    // make_test!(i8);
    // make_test!(u8);
    // make_test!(i16);
    // make_test!(u16);
    // make_test!(i32);
    // make_test!(u32);
    // make_test!(i64);
    // make_test!(u64);
    // make_test!(f32);
    make_test!(f64);
}
