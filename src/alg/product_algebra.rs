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

use super::{Algebra, BooleanAlgebra, BoundedLattice, Domain, Group, Lattice, Monoid, Semigroup};

/// The product of two algebras.
#[derive(PartialEq, Eq, Debug)]
pub struct ProductAlgebra<A0: Algebra, A1: Algebra>(A0, A1);

impl<A0: Algebra, A1: Algebra> ProductAlgebra<A0, A1> {
    /// Creates a new product algebra from the two factors.
    pub fn new(alg0: A0, alg1: A1) -> Self {
        Self(alg0, alg1)
    }
}

impl<A0: Algebra, A1: Algebra> Algebra for ProductAlgebra<A0, A1> {
    type Elem = (A0::Elem, A1::Elem);

    fn size(&self) -> Option<usize> {
        let a0 = self.0.size()?;
        let a1 = self.1.size()?;
        a0.checked_mul(a1)
    }

    fn element(&mut self, index: usize) -> Self::Elem {
        let a0 = self.0.size().unwrap();
        let a1 = self.1.size().unwrap();
        assert!(a0 != 0);
        let b0 = index % a0;
        let b1 = index / a0;
        assert!(b1 < a1);
        (self.0.element(b0), self.1.element(b1))
    }
}

impl<A0: Lattice, A1: Lattice> Lattice for ProductAlgebra<A0, A1> {
    fn meet(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        (
            self.0.meet(&elem0.0, &elem1.0),
            self.1.meet(&elem0.1, &elem1.1),
        )
    }

    fn join(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        (
            self.0.join(&elem0.0, &elem1.0),
            self.1.join(&elem0.1, &elem1.1),
        )
    }
}

impl<A0: BoundedLattice, A1: BoundedLattice> BoundedLattice for ProductAlgebra<A0, A1> {
    fn zero(&mut self) -> Self::Elem {
        (self.0.unit(), self.1.unit())
    }

    fn unit(&mut self) -> Self::Elem {
        (self.0.unit(), self.1.unit())
    }
}

impl<A0: BooleanAlgebra, A1: BooleanAlgebra> BooleanAlgebra for ProductAlgebra<A0, A1> {
    fn complement(&mut self, elem: &Self::Elem) -> Self::Elem {
        (self.0.complement(&elem.0), self.1.complement(&elem.1))
    }
}

impl<A0: Semigroup, A1: Semigroup> Semigroup for ProductAlgebra<A0, A1> {
    fn product(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        (
            self.0.product(&elem0.0, &elem1.0),
            self.1.product(&elem0.1, &elem1.1),
        )
    }
}

impl<A0: Monoid, A1: Monoid> Monoid for ProductAlgebra<A0, A1> {
    fn identity(&mut self) -> Self::Elem {
        (self.0.identity(), self.1.identity())
    }
}

impl<A0: Group, A1: Group> Group for ProductAlgebra<A0, A1> {
    fn inverse(&mut self, elem: &Self::Elem) -> Self::Elem {
        (self.0.inverse(&elem.0), self.1.inverse(&elem.1))
    }
}

impl<A0, A1> Domain for ProductAlgebra<A0, A1>
where
    A0: Domain,
    A1: Domain<Logic = A0::Logic>,
{
    type Logic = A0::Logic;

    fn logic(&mut self) -> &mut Self::Logic {
        assert!(self.0.logic() == self.1.logic());
        self.0.logic()
    }

    fn contains(&mut self, elem: &Self::Elem) -> <Self::Logic as Algebra>::Elem {
        let a0 = self.0.contains(&elem.0);
        let a1 = self.1.contains(&elem.1);
        self.logic().meet(&a0, &a1)
    }

    fn equals(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Algebra>::Elem {
        let a0 = self.0.equals(&elem0.0, &elem1.0);
        let a1 = self.1.equals(&elem0.1, &elem1.1);
        self.logic().meet(&a0, &a1)
    }
}
