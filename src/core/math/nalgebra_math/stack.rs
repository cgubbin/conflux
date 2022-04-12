use crate::core::math::{FPInto2D, FPStack};
use nalgebra::constraint::{DimEq, ShapeConstraint};
use nalgebra::{
    allocator::Allocator, base::Matrix, dimension::Dim, DefaultAllocator, OMatrix, Vector,
};
use nalgebra::{
    dimension::{DimAdd, DimSum},
    Scalar, Storage, U1,
};
use nalgebra::{Const, DimName};
use num_traits::Zero;

//impl<N, R1, R2, C1, S1, S2> FPStack<Vector<N, R2, S2>, OMatrix<N, DimSum<R1, U1>, C1>>
//    for Matrix<N, R1, C1, S1>
//where
//    N: Scalar + Zero + Copy,
//    R1: Dim + DimAdd<U1>,
//    R2: Dim,
//    C1: Dim,
//    ShapeConstraint: DimEq<R2, C1>, // We want to append a row so the number of entries in the Vec R2 must equal the col number in the matrix
//    S1: Storage<N, R1, C1>,
//    S2: Storage<N, R2, U1>,
//    <R1 as nalgebra::DimAdd<Const<1_usize>>>::Output: DimName,
//    C1: DimName,
//    DefaultAllocator: Allocator<N, DimSum<R1, U1>, C1>
//        + Allocator<N, R1, C1>
//        + nalgebra::allocator::Reallocator<N, R1, C1, DimSum<R1, U1>, C1>,
//{
//    #[inline]
//    fn stack(&self, other: &Vector<N, R2, S2>) -> OMatrix<N, DimSum<R1, U1>, C1> {
//        let values = self.iter().chain(other.into_iter()).map(|x| *x);
//        let out: OMatrix<N, DimSum<R1, U1>, C1> =
//            OMatrix::<N, DimSum<R1, U1>, C1>::from_iterator(values);
//        out
//    }
//}
//
use nalgebra::{DMatrix, DVector};

impl<N> FPStack<DVector<N>, DMatrix<N>> for DMatrix<N>
where
    N: Scalar + Zero + Copy,
{
    #[inline]
    fn stack(&self, other: &DVector<N>) -> DMatrix<N> {
        let values = self.iter().chain(other.into_iter()).map(|x| *x);
        let out: DMatrix<N> = DMatrix::from_iterator(self.nrows() + 1, self.ncols(), values);
        out
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use nalgebra::Matrix2x3;
//    use nalgebra::Matrix3;
//    use nalgebra::Vector3;
//    use paste::item;
//
//    macro_rules! make_test {
//        ($t:ty) => {
//            item! {
//                #[test]
//                fn [<test_stack_nalgebra_ $t>]() {
//                    let e: Vector3<$t> = Vector3::new(1 as $t, 2 as $t, 120 as $t);
//                    let mat: Matrix2x3<$t> = Matrix2x3::from_iterator(vec![
//                        2 as $t, 3 as $t, 4 as $t,
//                        3 as $t, 4 as $t, 5 as $t
//                    ].into_iter());
//                    let res = mat.stack(&e);
//                    let target: Matrix3<$t> = Matrix3::from_iterator(vec![
//                        2 as $t, 3 as $t, 4 as $t,
//                        3 as $t, 4 as $t, 5 as $t,
//                        1 as $t, 2 as $t, 120 as $t
//                    ].into_iter());
//                    for i in 0..3 {
//                        for j in 0..3 {
//                            assert!((((res[(i, j)] - target[(i, j)]) as f64).abs()) < std::f64::EPSILON);
//                        }
//                    }
//                }
//            }
//        };
//    }
//
//    make_test!(isize);
//    make_test!(usize);
//    make_test!(i8);
//    make_test!(u8);
//    make_test!(i16);
//    make_test!(u16);
//    make_test!(i32);
//    make_test!(u32);
//    make_test!(i64);
//    make_test!(u64);
//    make_test!(f32);
//    make_test!(f64);
//}
//
