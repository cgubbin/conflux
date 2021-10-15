use crate::core::math::{FPEye};
use num::{One, Zero};

impl<T> FPEye for ndarray::Array2<T>
where
    T: Clone + One + Zero
{
    #[inline]
    fn eye(dim: usize) -> ndarray::Array2<T> {
        ndarray::Array2::eye(dim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array2};
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_eye_ $t>]() {
                    let e: Array2<$t> = <Array2<$t> as FPEye>::eye(3);
                    let res = array![
                        [1 as $t, 0 as $t, 0 as $t],
                        [0 as $t, 1 as $t, 0 as $t],
                        [0 as $t, 0 as $t, 1 as $t]
                    ];
                    for i in 0..3 {
                        for j in 0..3 {
                            assert!((((res[(i, j)] - e[(i, j)]) as f64).abs()) < std::f64::EPSILON);
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