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

use super::{BooleanAlgebra, BoundedLattice, DirectedGraph, Domain, PartialOrder};

/// The two-element boolean algebra.
pub struct BooleanLogic();

/// The two-element boolean algebra.
pub const BOOLEAN_LOGIC: BooleanLogic = BooleanLogic();

impl Domain for BooleanLogic {
    type Logic = BooleanLogic;

    fn logic(&self) -> &Self::Logic {
        self
    }

    type Elem = bool;

    fn contains(&self, _elem: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        true
    }

    fn equals(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        elem1 == elem2
    }
}

impl DirectedGraph for BooleanLogic {
    fn related(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        elem1 <= elem2
    }
}

impl PartialOrder for BooleanLogic {}

impl BoundedLattice for BooleanLogic {
    fn unit(&self) -> Self::Elem {
        true
    }

    fn zero(&self) -> Self::Elem {
        false
    }

    fn meet(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        *elem1 && *elem2
    }

    fn join(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        *elem1 || *elem2
    }
}

impl BooleanAlgebra for BooleanLogic {
    fn complement(&self, elem: &Self::Elem) -> Self::Elem {
        !*elem
    }
}
