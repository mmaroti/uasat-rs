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

use super::{BooleanAlgebra, BoundedLattice, DirectedGraph, Domain, Field, Lattice, PartialOrder};

/// The two-element boolean algebra, which is also a field and an ordered chain.
#[derive(Debug)]
pub struct TwoElementAlg();

/// The unique two-element boolean algebra used for classical logic.
pub const TWO_ELEMENT_ALG: TwoElementAlg = TwoElementAlg();

impl Domain for TwoElementAlg {
    type Elem = bool;

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

impl Lattice for TwoElementAlg {
    fn meet(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0 && *elem1
    }

    fn join(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0 || *elem1
    }
}

impl BoundedLattice for TwoElementAlg {
    fn bot(&self) -> Self::Elem {
        false
    }

    fn top(&self) -> Self::Elem {
        true
    }
}

impl BooleanAlgebra for TwoElementAlg {
    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        !*elem
    }

    fn add(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0 != *elem1
    }

    fn imp(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0 <= *elem1
    }

    fn equ(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0 == *elem1
    }
}

impl Field for TwoElementAlg {
    fn inv(&self, elem: &Self::Elem) -> Self::Elem {
        *elem
    }
}

impl DirectedGraph for TwoElementAlg {
    fn edge(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        *elem0 <= *elem1
    }
}

impl PartialOrder for TwoElementAlg {}
