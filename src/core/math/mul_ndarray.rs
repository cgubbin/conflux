use crate::core::math::FPMul;
use ndarray::{Array1};

macro_rules! make_mul {
    ($t:ty) => {
        impl FPMul<$t, Array1<$t>> for Array1<$t> {
            #[inline]
            fn mul(&self, other: &$t) -> Array1<$t> {
                self * *other
            }
        }
    };
}

make_mul!(i8);
make_mul!(u8);
make_mul!(i16);
make_mul!(u16);
make_mul!(i32);
make_mul!(u32);
make_mul!(i64);
make_mul!(u64);
make_mul!(f32);
make_mul!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_mul_vec_scalar_ $t>]() {
                    let a = array![1 as $t, 4 as $t, 8 as $t];
                    let b = 2 as $t;
                    let target = array![2 as $t, 8 as $t, 16 as $t];
                    let res = <Array1<$t> as FPMul<$t, Array1<$t>>>::mul(&a, &b);
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