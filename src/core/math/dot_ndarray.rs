use crate::core::math::FPDot;
use ndarray::{Array1, Array2};

macro_rules! make_dot {
    ($t:ty) => {
        impl FPDot<Array1<$t>, $t> for Array1<$t> {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> $t {
                ndarray::Array1::dot(self, other)
            }
        }

        impl FPDot<$t, Array1<$t>> for Array1<$t> {
            #[inline]
            fn dot(&self, other: &$t) -> Array1<$t> {
                *other * self
            }
        }

        impl<'a> FPDot<Array1<$t>, Array1<$t>> for $t {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> Array1<$t> {
                other * *self
            }
        }

        impl FPDot<Array1<$t>, Array2<$t>> for Array1<$t> {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> Array2<$t> {
                let mut out = Array2::zeros((self.len(), other.len()));
                for i in 0..self.len() {
                    for j in 0..other.len() {
                        out[(i, j)] = self[i] * other[j];
                    }
                }
                out
            }
        }

        impl FPDot<Array1<$t>, Array1<$t>> for Array2<$t> {
            #[inline]
            fn dot(&self, other: &Array1<$t>) -> Array1<$t> {
                ndarray::Array2::dot(self, other)
            }
        }

        impl FPDot<Array2<$t>, Array2<$t>> for Array2<$t> {
            #[inline]
            fn dot(&self, other: &Array2<$t>) -> Array2<$t> {
                ndarray::Array2::dot(self, other)
            }
        }

        impl FPDot<$t, Array2<$t>> for Array2<$t> {
            #[inline]
            fn dot(&self, other: &$t) -> Array2<$t> {
                *other * self
            }
        }

        impl<'a> FPDot<Array2<$t>, Array2<$t>> for $t {
            #[inline]
            fn dot(&self, other: &Array2<$t>) -> Array2<$t> {
                other * *self
            }
        }
    };
}

make_dot!(i8);
make_dot!(u8);
make_dot!(i16);
make_dot!(u16);
make_dot!(i32);
make_dot!(u32);
make_dot!(i64);
make_dot!(u64);
make_dot!(f32);
make_dot!(f64);