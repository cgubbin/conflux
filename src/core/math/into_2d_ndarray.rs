use crate::core::math::FPInto2D;
use ndarray::{Array1, Array2};

impl<T> FPInto2D<Array2<T>> for Array1<T>
where
    T: Clone
    {
        #[inline]
        fn into_2d(&self) -> Array2<T> {
            self
                .clone()
                .into_shape((1, self.len()))
                .unwrap()
        }
    }