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
    BoundedLattice, DirectedGraph, Domain, Lattice, PartialOrder, Ring, TwoElementAlg, UnitaryRing,
    TWO_ELEMENT_ALG,
};

/// The ring of integers whose elements are represented as `i32` values. The operations are
/// partial, thus they will panic if the result does not fit into an `i32` value. The integers
/// form a chain with the natural order.
#[derive(Debug)]
pub struct SmallIntegers();

/// The unique ring of small integers with the natural chain order.
pub const SMALL_INTEGERS: SmallIntegers = SmallIntegers();

impl Domain for SmallIntegers {
    type Elem = i32;

    type Logic = TwoElementAlg;

    fn logic(&self) -> &Self::Logic {
        &TWO_ELEMENT_ALG
    }

    fn contains(&self, _elem: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        true
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        elem0 == elem1
    }
}

impl Lattice for SmallIntegers {
    fn meet(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0.min(elem1)
    }

    fn join(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0.max(elem1)
    }
}

impl BoundedLattice for SmallIntegers {
    fn bot(&self) -> Self::Elem {
        i32::MIN
    }

    fn top(&self) -> Self::Elem {
        i32::MAX
    }
}

impl DirectedGraph for SmallIntegers {
    fn edge(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        *elem0 <= *elem1
    }
}

impl PartialOrder for SmallIntegers {}

impl Ring for SmallIntegers {
    fn zero(&self) -> Self::Elem {
        0
    }

    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        elem.checked_neg().unwrap()
    }

    fn add(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        elem0.checked_add(*elem1).unwrap()
    }

    fn sub(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        elem0.checked_sub(*elem1).unwrap()
    }

    fn mul(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        elem0.checked_mul(*elem1).unwrap()
    }
}

impl UnitaryRing for SmallIntegers {
    fn unit(&self) -> Self::Elem {
        1
    }
}
