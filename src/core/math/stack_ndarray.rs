use crate::core::math::{FPInto2D, FPStack};
use ndarray::{Array1, Array2};

impl<T> FPStack<Array1<T>> for Array2<T>
where
    T: Clone
    {
        #[inline]
        fn stack(&self, other: &Array1<T>) -> Array2<T> {
            ndarray::concatenate(
                ndarray::Axis(0),
                &[self.view(), other.into_2d().view()]
            ).unwrap()
        }
    }