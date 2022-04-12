// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::core::math::FPNorm;

use nalgebra::{DMatrix, DVector, SimdComplexField};

impl<N> FPNorm<N::SimdRealField> for DVector<N>
where
    N: SimdComplexField,
{
    #[inline]
    fn norm(&self) -> N::SimdRealField {
        self.norm()
    }
}

impl<N> FPNorm<N::SimdRealField> for DMatrix<N>
where
    N: SimdComplexField,
{
    #[inline]
    fn norm(&self) -> N::SimdRealField {
        self.norm()
    }
}
