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

use super::{Algebra, BooleanAlgebra, BoundedLattice, Domain, Lattice};

/// The two-element boolean algebra.
#[derive(Debug)]
pub struct BooleanLogic();

/// The two-element boolean algebra.
pub const BOOLEAN_LOGIC: BooleanLogic = BooleanLogic();

impl Algebra for BooleanLogic {
    type Elem = bool;
}

impl Lattice for BooleanLogic {
    fn meet(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0 && *elem1
    }

    fn join(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        *elem0 || *elem1
    }
}

impl BoundedLattice for BooleanLogic {
    fn bot(&self) -> Self::Elem {
        false
    }

    fn top(&self) -> Self::Elem {
        true
    }
}

impl BooleanAlgebra for BooleanLogic {
    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        !*elem
    }
}

impl Domain for BooleanLogic {
    type Logic = BooleanLogic;

    fn logic(&self) -> &Self::Logic {
        &BOOLEAN_LOGIC
    }

    fn contains(&self, _elem: &Self::Elem) -> <Self::Logic as Algebra>::Elem {
        true
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Algebra>::Elem {
        elem0 == elem1
    }
}
