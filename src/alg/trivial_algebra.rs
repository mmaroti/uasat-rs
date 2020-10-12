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

use super::{BooleanAlgebra, BoundedLattice, Domain, Group, Lattice, Monoid, Semigroup};

/// The one-element trivial algebra.
pub struct TrivialAlgebra();

/// The one-element trivial algebra.
pub const TRIVIAL_ALGEBRA: TrivialAlgebra = TrivialAlgebra();

impl Domain for TrivialAlgebra {
    type Elem = ();

    fn size(&self) -> Option<usize> {
        Some(1)
    }
}

impl Lattice for TrivialAlgebra {
    fn meet(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {
        ()
    }

    fn join(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {
        ()
    }
}

impl BoundedLattice for TrivialAlgebra {
    fn unit(&self) -> Self::Elem {
        ()
    }

    fn zero(&self) -> Self::Elem {
        ()
    }
}

impl BooleanAlgebra for TrivialAlgebra {
    fn complement(&self, _elem: &Self::Elem) -> Self::Elem {
        ()
    }
}

impl Semigroup for TrivialAlgebra {
    fn product(&self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {
        ()
    }
}

impl Monoid for TrivialAlgebra {
    fn identity(&self) -> Self::Elem {
        ()
    }
}

impl Group for TrivialAlgebra {
    fn inverse(&self, _elem: &Self::Elem) -> Self::Elem {
        ()
    }
}
