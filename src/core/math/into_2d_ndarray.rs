use crate::core::math::FPInto2D;
use ndarray::{Array1, Array2};

impl<T> FPInto2D<Array2<T>> for Array1<T>
where
    T: Clone,
{
    #[inline]
    fn into_2d(self) -> Array2<T> {
        self.clone().into_shape((1, self.len())).unwrap()
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
                fn [<test_into_2d_ $t>]() {
                    let e: Array1<$t> = array![1 as $t, 2 as $t, 120 as $t];
                    let e_2d = e.clone().into_2d();
                    for i in 0..3 {
                        assert!((((e_2d[(0, i)] - e[i]) as f64).abs()) < std::f64::EPSILON);
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
