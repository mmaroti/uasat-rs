/*
* Copyright (C) 2020, Miklos Maroti
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use super::{
    BooleanAlgebra, BoundedPartialOrder, DirectedGraph, Domain, Group, Lattice, Monoid,
    PartialOrder, Semigroup, TwoElementAlg, TWO_ELEMENT_ALG,
};

/// The one-element trivial algebra, which is boolean algebra, unitary ring and a partial order.
#[derive(Debug)]
pub struct TrivialAlgebra();

/// The unique one-element trivial algebra.
pub const TRIVIAL_ALGEBRA: TrivialAlgebra = TrivialAlgebra();

impl Domain for TrivialAlgebra {
    type Elem = ();

    type Logic = TwoElementAlg;

    fn logic(&self) -> &Self::Logic {
        &TWO_ELEMENT_ALG
    }

    fn contains(&self, _elem: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        true
    }

    fn equals(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        true
    }
}

impl DirectedGraph for TrivialAlgebra {
    fn edge(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        true
    }
}

impl PartialOrder for TrivialAlgebra {}

impl BoundedPartialOrder for TrivialAlgebra {
    fn bot(&self) -> Self::Elem {}

    fn top(&self) -> Self::Elem {}
}

impl Lattice for TrivialAlgebra {
    fn meet(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {}

    fn join(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {}
}

impl BooleanAlgebra for TrivialAlgebra {
    fn not(&self, _elem: &Self::Elem) -> Self::Elem {}
}

impl Semigroup for TrivialAlgebra {
    fn mul(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {}
}

impl Monoid for TrivialAlgebra {
    fn unit(&self) -> Self::Elem {}
}

impl Group for TrivialAlgebra {
    fn inv(&self, _elem: &Self::Elem) -> Self::Elem {}
}
