use crate::core::math::FPNorm;
use num_complex::Complex;

macro_rules! make_norm_unsigned {
    ($t:ty) => {
        impl FPNorm<$t> for $t {
            #[inline]
            fn norm(&self) -> $t {
                *self
            }
        }
    };
}

macro_rules! make_norm {
    ($t:ty) => {
        impl FPNorm<$t> for $t {
            #[inline]
            fn norm(&self) -> $t {
                (*self).abs()
            }
        }
    };
}


macro_rules! make_norm_complex {
    ($t:ty) => {
        impl FPNorm<$t> for Complex<$t> {
            #[inline]
            fn norm(&self) -> $t {
                (*self).re.hypot((*self).im)
            }
        }
    };
}

make_norm!(isize);
make_norm_unsigned!(usize);
make_norm!(i8);
make_norm!(i16);
make_norm!(i32);
make_norm!(i64);
make_norm_unsigned!(u8);
make_norm_unsigned!(u16);
make_norm_unsigned!(u32);
make_norm_unsigned!(u64);
make_norm!(f32);
make_norm!(f64);
make_norm_complex!(f32);
make_norm_complex!(f64);
