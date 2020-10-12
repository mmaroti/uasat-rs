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

use super::{BooleanAlgebra, BoundedLattice, Domain, Lattice};

/// The two-element boolean algebra.
pub struct BooleanLogic();

/// The two-element boolean algebra.
pub const BOOLEAN_LOGIC: BooleanLogic = BooleanLogic();

impl Domain for BooleanLogic {
    type Elem = bool;

    fn size(&self) -> Option<usize> {
        Some(2)
    }
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
    fn unit(&self) -> Self::Elem {
        true
    }

    fn zero(&self) -> Self::Elem {
        false
    }
}

impl BooleanAlgebra for BooleanLogic {
    fn complement(&self, elem: &Self::Elem) -> Self::Elem {
        !*elem
    }
}
