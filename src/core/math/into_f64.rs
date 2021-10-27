use crate::core::math::FPIntof64;

macro_rules! make_into_f64 {
    ($t:ty) => {
        impl FPIntof64 for $t {
            #[inline]
            fn cast_f64(&self) -> f64 {
                *self as f64
            }
        }
    };
}

make_into_f64!(isize);
make_into_f64!(usize);
make_into_f64!(i8);
make_into_f64!(i16);
make_into_f64!(i32);
make_into_f64!(i64);
make_into_f64!(u8);
make_into_f64!(u16);
make_into_f64!(u32);
make_into_f64!(u64);
make_into_f64!(f32);
make_into_f64!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_into_f64_ $t>]() {
                    let a = 8 as $t;
                    let res = a.cast_f64();
                    let b = 8f64;
                    assert!(((b - res) as f64).abs() < std::f64::EPSILON);
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
