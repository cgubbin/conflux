use crate::core::math::FPDiv;
use ndarray::Array1;

macro_rules! make_div {
    ($t:ty) => {
        impl FPDiv<$t, Array1<$t>> for Array1<$t> {
            #[inline]
            fn div(&self, other: &$t) -> Array1<$t> {
                self / *other
            }
        }
    };
}

make_div!(i8);
make_div!(u8);
make_div!(i16);
make_div!(u16);
make_div!(i32);
make_div!(u32);
make_div!(i64);
make_div!(u64);
make_div!(f32);
make_div!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_div_vec_scalar_ $t>]() {
                    let a = array![4 as $t, 16 as $t, 8 as $t];
                    let b = 2 as $t;
                    let target = array![2 as $t, 8 as $t, 4 as $t];
                    let res = <Array1<$t> as FPDiv<$t, Array1<$t>>>::div(&a, &b);
                    for i in 0..3 {
                        assert!(((target[i] - res[i]) as f64).abs() < std::f64::EPSILON);
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
