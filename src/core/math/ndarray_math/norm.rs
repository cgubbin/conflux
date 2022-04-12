use crate::core::math::FPNorm;
use ndarray::Array1;
use num_complex::Complex;
use num_traits::Float;
use num_traits::Zero;

impl<T: Float + Copy + Zero> FPNorm<T> for Array1<Complex<T>> {
    #[inline]
    fn norm(&self) -> T {
        self.iter()
            .map(|x| x.norm_sqr())
            .fold(T::zero(), |acc, x| acc + x)
            .sqrt()
    }
}

impl<T: Float + Copy + Zero> FPNorm<T> for Array1<T> {
    #[inline]
    fn norm(&self) -> T {
        self.iter()
            .map(|x| x.powi(2))
            .fold(T::zero(), |acc, x| acc + x)
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1};
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_norm_ $t>]() {
                    let a = array![4 as $t, 3 as $t];
                    let res = <Array1<$t> as FPNorm<$t>>::norm(&a);
                    let target = 5 as $t;
                    assert!(((target - res) as f64).abs() < std::f64::EPSILON);
                }
            }
        };
    }

    macro_rules! make_test_signed {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_norm_signed_ $t>]() {
                    let a = array![-4 as $t, -3 as $t];
                    let res = <Array1<$t> as FPNorm<$t>>::norm(&a);
                    let target = 5 as $t;
                    assert!(((target - res) as f64).abs() < std::f64::EPSILON);
                }
            }
        };
    }

    make_test!(f32);
    make_test!(f64);

    make_test_signed!(f32);
    make_test_signed!(f64);
}
