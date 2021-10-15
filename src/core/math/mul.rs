use crate::core::math::FPMul;
use num_complex::Complex;

macro_rules! make_mul {
    ($t:ty) => {
        impl FPMul<$t, $t> for $t {
            #[inline]
            fn mul(&self, other: &$t) -> $t {
                self * other
            }
        }
    };
}

make_mul!(isize);
make_mul!(usize);
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
make_mul!(Complex<isize>);
make_mul!(Complex<usize>);
make_mul!(Complex<i8>);
make_mul!(Complex<u8>);
make_mul!(Complex<i16>);
make_mul!(Complex<u16>);
make_mul!(Complex<i32>);
make_mul!(Complex<u32>);
make_mul!(Complex<i64>);
make_mul!(Complex<u64>);
make_mul!(Complex<f32>);
make_mul!(Complex<f64>);

#[cfg(test)]
mod tests {
    use super::*;
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_mul_ $t>]() {
                    let a = 2 as $t;
                    let b = 21 as $t;
                    let res = <$t as FPMul<$t, $t>>::mul(&a, &b);
                    assert!(((42 as $t - res) as f64).abs() < std::f64::EPSILON);
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