use crate::core::math::FPTranspose;

use nalgebra::{
    base::{allocator::Allocator, dimension::Dim, storage::Storage, Scalar},
    DefaultAllocator, Matrix, OMatrix,
};

//impl<N, R, C, S> FPTranspose<OMatrix<N, C, R>> for Matrix<N, R, C, S>
//where
//    N: Scalar,
//    R: Dim,
//    C: Dim,
//    S: Storage<N, R, C>,
//    DefaultAllocator: Allocator<N, C, R>,
//{
//    #[inline]
//    fn t(&self) -> OMatrix<N, C, R> {
//        self.transpose()
//    }
//}

use nalgebra::{DMatrix, DVector};

impl<N> FPTranspose<DMatrix<N>> for DMatrix<N>
where
    N: Scalar,
{
    #[inline]
    fn t(&self) -> DMatrix<N> {
        self.transpose()
    }
}

impl<N> FPTranspose<DVector<N>> for DVector<N>
where
    N: Scalar,
{
    #[inline]
    fn t(&self) -> DVector<N> {
        self.to_owned()
    }
}
