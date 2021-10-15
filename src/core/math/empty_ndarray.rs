use crate::core::math::FPEmpty;

impl<T> FPEmpty for ndarray::Array1<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> FPEmpty for ndarray::Array2<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
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
                fn [<test_empty_vec_ $t>]() {
                    let vec: ndarray::Array1<$t> = array![];
                    assert!(vec.is_empty());
                }
            }

            item! {
                #[test]
                fn [<test_empty_mat_ $t>]() {
                    let vec: ndarray::Array2<$t> = ndarray::Array2::zeros((0, 0));
                    assert!(vec.is_empty());
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