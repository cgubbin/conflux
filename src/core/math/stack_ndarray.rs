use crate::core::math::{FPInto2D, FPStack};
use ndarray::{Array1, Array2};

impl<T> FPStack<Array1<T>> for Array2<T>
where
    T: Clone
    {
        #[inline]
        fn stack(&self, other: &Array1<T>) -> Array2<T> {
            ndarray::concatenate(
                ndarray::Axis(0),
                &[self.view(), other.into_2d().view()]
            ).unwrap()
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
                fn [<test_stack_ $t>]() {
                    let e: Array1<$t> = array![1 as $t, 2 as $t, 120 as $t];
                    let mat: Array2<$t> = array![
                        [2 as $t, 3 as $t, 4 as $t],
                        [3 as $t, 4 as $t, 5 as $t]
                    ]; 
                    let res = mat.stack(&e);
                    let target: Array2<$t> = array![
                        [2 as $t, 3 as $t, 4 as $t],
                        [3 as $t, 4 as $t, 5 as $t],
                        [1 as $t, 2 as $t, 120 as $t]
                    ]; 
                    for i in 0..3 {
                        for j in 0..3 {
                            assert!((((res[(i, j)] - target[(i, j)]) as f64).abs()) < std::f64::EPSILON);
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