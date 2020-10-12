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
    Algebra, BooleanAlgebra, BooleanLogic, BoundedLattice, Domain, Group, Lattice, Monoid,
    Semigroup,
};

/// The one-element trivial algebra.
#[derive(PartialEq, Eq, Debug)]
pub struct TrivialAlgebra {
    logic: BooleanLogic,
}

/// The one-element trivial algebra.
pub const TRIVIAL_ALGEBRA: TrivialAlgebra = TrivialAlgebra {
    logic: BooleanLogic(),
};

impl Algebra for TrivialAlgebra {
    type Elem = ();
}

impl Lattice for TrivialAlgebra {
    fn meet(&mut self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {}

    fn join(&mut self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {}
}

impl BoundedLattice for TrivialAlgebra {
    fn bot(&mut self) -> Self::Elem {}

    fn top(&mut self) -> Self::Elem {}
}

impl BooleanAlgebra for TrivialAlgebra {
    fn neg(&mut self, _elem: &Self::Elem) -> Self::Elem {}
}

impl Semigroup for TrivialAlgebra {
    fn mul(&mut self, _elem0: &Self::Elem, _elem1: &Self::Elem) -> Self::Elem {}
}

impl Monoid for TrivialAlgebra {
    fn unit(&mut self) -> Self::Elem {}
}

impl Group for TrivialAlgebra {
    fn inv(&mut self, _elem: &Self::Elem) -> Self::Elem {}
}

impl Domain for TrivialAlgebra {
    type Logic = BooleanLogic;

    fn logic(&mut self) -> &mut Self::Logic {
        &mut self.logic
    }

    fn contains(&mut self, _elem: &Self::Elem) -> <Self::Logic as Algebra>::Elem {
        true
    }

    fn equals(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Algebra>::Elem {
        elem0 == elem1
    }
}