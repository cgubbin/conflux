use crate::core::math::{FPZeros, FPZerosLike, FPFromZeros};
use num::Zero;

impl<T> FPZerosLike for ndarray::Array1<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros_like(&self) -> ndarray::Array1<T> {
        ndarray::Array1::zeros(self.raw_dim())
    }
}

impl<T> FPZerosLike for ndarray::Array2<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros_like(&self) -> ndarray::Array2<T> {
        ndarray::Array2::zeros(self.raw_dim())
    }
}

impl<T> FPFromZeros for ndarray::Array1<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros(dim: usize) -> ndarray::Array1<T> {
        ndarray::Array1::zeros(dim)
    }
}

impl<T> FPFromZeros for ndarray::Array2<T>
where
    T: Zero + FPZeros + Clone,
{
    #[inline]
    fn zeros(dim: usize) -> ndarray::Array2<T> {
        ndarray::Array2::zeros((dim, dim))
    }
}

mod tests {
    use super::*;
    use paste::item;
    use ndarray::{array, Array1, Array2};

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_zeros_like_ $t>]() {
                    let t: Array1<$t> = array![];
                    let a = t.zeros_like();
                    assert_eq!(t, a);
                }
            }

            item! {
                #[test]
                fn [<test_zeros_like_2_ $t>]() {
                    let a = (array![42 as $t, 42 as $t, 42 as $t, 42 as $t]).zeros_like();
                    for i in 0..4 {
                        assert!(((0 as $t - a[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_2d_zeros_like_ $t>]() {
                    let t: Array2<$t> = Array2::zeros((0, 0));
                    let a = t.zeros_like();
                    assert_eq!(t, a);
                }
            }

            item! {
                #[test]
                fn [<test_2d_zeros_like_2_ $t>]() {
                    let a = (array![[42 as $t, 42 as $t], [42 as $t, 42 as $t]]).zeros_like();
                    for i in 0..2 {
                        for j in 0..2 {
                            assert!(((0 as $t - a[(i, j)]) as f64).abs() < std::f64::EPSILON);
                        }
                    }
                }
            }
        };
    }

    make_test!(isize);
    make_test!(usize);
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