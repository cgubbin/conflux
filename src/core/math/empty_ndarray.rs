use crate::core::math::FPEmpty;

impl<T> FPEmpty for ndarray::Array1<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> FPEmpty for ndarray::Array2<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}