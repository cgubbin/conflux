use crate::core::math::FPNorm;
use ndarray::Array1;
use num::integer::Roots;
use num_complex::Complex;

macro_rules! make_norm_float {
    ($t:ty) => {
        impl FPNorm<$t> for Array1<$t> {
            #[inline]
            fn norm(&self) -> $t {
                self.iter().map(|x| x.powi(2)).sum::<$t>().sqrt()
            }
        }
    };
}

macro_rules! make_norm_complex_float {
    ($t:ty) => {
        impl FPNorm<Complex<$t>> for Array1<Complex<$t>> {
            #[inline]
            fn norm(&self) -> Complex<$t> {
                self.iter().map(|a| a.powf(2.0)).sum::<Complex<$t>>().sqrt()
            }
        }

        impl FPNorm<$t> for Array1<Complex<$t>> {
            #[inline]
            fn norm(&self) -> $t {
                self.iter().map(|a| a.powf(2.0)).sum::<Complex<$t>>().sqrt().norm()
            }
        }
    };
}

macro_rules! make_norm_integer {
    ($t:ty) => {
        impl FPNorm<$t> for Array1<$t> {
            #[inline]
            fn norm(&self) -> $t {
                self.iter().map(|a| a.pow(2)).sum::<$t>().sqrt()
            }
        }
    };
}

make_norm_integer!(isize);
make_norm_integer!(usize);
make_norm_integer!(i8);
make_norm_integer!(i16);
make_norm_integer!(i32);
make_norm_integer!(i64);
make_norm_integer!(u8);
make_norm_integer!(u16);
make_norm_integer!(u32);
make_norm_integer!(u64);
make_norm_float!(f32);
make_norm_float!(f64);
make_norm_complex_float!(f32);
make_norm_complex_float!(f64);
