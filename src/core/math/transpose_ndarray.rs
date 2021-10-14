use crate::core::math::FPTranspose;

impl<T> FPTranspose for ndarray::Array2<T>
where
    T: Clone
{
    #[inline]
    fn t(&self) -> ndarray::Array2<T> {
        self.t().to_owned()
    }
}

impl<T> FPTranspose for ndarray::Array1<T>
where
    T: Clone
{
    #[inline]
    fn t(&self) -> ndarray::Array1<T> {
        self.t().to_owned()
    }
}