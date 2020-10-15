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
    BooleanAlgebra, BoundedLattice, DirectedGraph, Domain, Group, Lattice, Monoid, PartialOrder,
    Semigroup,
};

/// The product of two algebras.
#[derive(Debug)]
pub struct ProductAlgebra<A0, A1>(A0, A1);

impl<A0, A1> ProductAlgebra<A0, A1>
where
    A0: Domain,
    A1: Domain<Logic = A0::Logic>,
{
    /// Creates a new product algebra from the two factors and checks that the underlying
    /// logic of the two domains coincide (they are the same objects).
    pub fn new(alg0: A0, alg1: A1) -> Self {
        assert_eq!(
            alg0.logic() as *const A0::Logic,
            alg1.logic() as *const A1::Logic
        );
        Self(alg0, alg1)
    }
}

impl<A0, A1> Domain for ProductAlgebra<A0, A1>
where
    A0: Domain,
    A1: Domain<Logic = A0::Logic>,
{
    type Elem = (A0::Elem, A1::Elem);

    type Logic = A0::Logic;

    fn logic(&self) -> &Self::Logic {
        self.0.logic()
    }

    fn contains(&self, elem: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        let a0 = self.0.contains(&elem.0);
        let a1 = self.1.contains(&elem.1);
        self.logic().meet(&a0, &a1)
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        let a0 = self.0.equals(&elem0.0, &elem1.0);
        let a1 = self.1.equals(&elem0.1, &elem1.1);
        self.logic().meet(&a0, &a1)
    }
}

impl<A0, A1> Lattice for ProductAlgebra<A0, A1>
where
    A0: Lattice,
    A1: Lattice<Logic = A0::Logic>,
{
    fn meet(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        (
            self.0.meet(&elem0.0, &elem1.0),
            self.1.meet(&elem0.1, &elem1.1),
        )
    }

    fn join(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        (
            self.0.join(&elem0.0, &elem1.0),
            self.1.join(&elem0.1, &elem1.1),
        )
    }
}

impl<A0, A1> BoundedLattice for ProductAlgebra<A0, A1>
where
    A0: BoundedLattice,
    A1: BoundedLattice<Logic = A0::Logic>,
{
    fn bot(&self) -> Self::Elem {
        (self.0.bot(), self.1.bot())
    }

    fn top(&self) -> Self::Elem {
        (self.0.top(), self.1.top())
    }
}

impl<A0, A1> BooleanAlgebra for ProductAlgebra<A0, A1>
where
    A0: BooleanAlgebra,
    A1: BooleanAlgebra<Logic = A0::Logic>,
{
    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        (self.0.neg(&elem.0), self.1.neg(&elem.1))
    }
}

impl<A0, A1> Semigroup for ProductAlgebra<A0, A1>
where
    A0: Semigroup,
    A1: Semigroup<Logic = A0::Logic>,
{
    fn mul(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        (
            self.0.mul(&elem0.0, &elem1.0),
            self.1.mul(&elem0.1, &elem1.1),
        )
    }
}

impl<A0, A1> Monoid for ProductAlgebra<A0, A1>
where
    A0: Monoid,
    A1: Monoid<Logic = A0::Logic>,
{
    fn unit(&self) -> Self::Elem {
        (self.0.unit(), self.1.unit())
    }
}

impl<A0, A1> Group for ProductAlgebra<A0, A1>
where
    A0: Group,
    A1: Group<Logic = A0::Logic>,
{
    fn inv(&self, elem: &Self::Elem) -> Self::Elem {
        (self.0.inv(&elem.0), self.1.inv(&elem.1))
    }
}

impl<A0, A1> DirectedGraph for ProductAlgebra<A0, A1>
where
    A0: DirectedGraph,
    A1: DirectedGraph<Logic = A0::Logic>,
{
    fn edge(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        let a0 = self.0.edge(&elem0.0, &elem1.0);
        let a1 = self.1.edge(&elem0.1, &elem1.1);
        self.logic().meet(&a0, &a1)
    }
}

impl<A0, A1> PartialOrder for ProductAlgebra<A0, A1>
where
    A0: PartialOrder,
    A1: PartialOrder<Logic = A0::Logic>,
{
}
