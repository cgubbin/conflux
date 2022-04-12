use crate::core::math::FPTranspose;

impl<T> FPTranspose<ndarray::Array2<T>> for ndarray::Array2<T>
where
    T: Clone,
{
    #[inline]
    fn t(&self) -> ndarray::Array2<T> {
        self.t().to_owned()
    }
}

impl<T> FPTranspose<ndarray::Array1<T>> for ndarray::Array1<T>
where
    T: Clone,
{
    #[inline]
    fn t(&self) -> ndarray::Array1<T> {
        self.t().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_transpose_ $t>]() {
                    let a = array![1 as $t, 4 as $t];
                    let target = array![1 as $t, 4 as $t];
                    let res = a.t();
                    for i in 0..2 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_transpose_2d_1_ $t>]() {
                    let a = array![
                        [1 as $t, 4 as $t],
                        [8 as $t, 7 as $t]
                    ];
                    let target = array![
                        [1 as $t, 8 as $t],
                        [4 as $t, 7 as $t]
                    ];
                    let res = a.t();
                    for i in 0..2 {
                        for j in 0..2 {
                            assert!(((target[(i, j)] - res[(i, j)]) as f64).abs() < std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                fn [<test_transpose_2d_2_ $t>]() {
                    let a = array![
                        [1 as $t, 4 as $t],
                        [8 as $t, 7 as $t],
                        [3 as $t, 6 as $t]
                    ];
                    let target = array![
                        [1 as $t, 8 as $t, 3 as $t],
                        [4 as $t, 7 as $t, 6 as $t]
                    ];
                    let res = a.t();
                    for i in 0..2 {
                        for j in 0..3 {
                            assert!(((target[(i, j)] - res[(i, j)]) as f64).abs() < std::f64::EPSILON);
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
