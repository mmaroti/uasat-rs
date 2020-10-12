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

/// An arbitrary set of elements over a fixed type.
pub trait Algebra {
    /// The type of objects representing the elements of this algebra. Not all objects are
    /// valid elements, and structurally different objects may represent the same element.
    type Elem: Clone;
}

/// A lattice, which is an ordered set where every two elements have a largest lower bound
/// and a smallest upper bound.
pub trait Lattice: Algebra {
    /// The meet (largest lower bound) of two elements in the lattice.
    fn meet(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;

    /// The join (least upper bound) of two elements in the lattice.
    fn join(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

/// A bounded lattice, which is a lattice with a smallest and a largest element.
pub trait BoundedLattice: Lattice {
    /// The smallest (bottom) element of the lattice.
    fn bot(&mut self) -> Self::Elem;

    /// The largest (top) element of the lattice.
    fn top(&mut self) -> Self::Elem;
}

/// A boolean algebra, which is a complemented distributive bounded lattice.
pub trait BooleanAlgebra: BoundedLattice {
    /// Returns the complement of the given element.
    fn neg(&mut self, elem: &Self::Elem) -> Self::Elem;

    /// The symmetric difference of two elements.
    fn add(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        let neg0 = self.neg(elem0);
        let neg1 = self.neg(elem1);
        let elem3 = self.meet(elem0, &neg1);
        let elem4 = self.meet(&neg0, elem1);
        self.join(&elem3, &elem4)
    }

    /// The logical implication of the given elements(disjunction of the negation of the first
    /// with the second one).
    fn imp(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        let neg0 = self.neg(elem0);
        self.join(&neg0, elem1)
    }

    /// The logical equivalence of the given elements (the symmetric difference of the negaton of
    /// the first element with the second one).
    fn equ(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        let neg0 = self.neg(elem0);
        self.add(&neg0, elem1)
    }
}

/// A semigroup, which is domain with an associative binary operation.
pub trait Semigroup: Algebra {
    /// The product of two elements in the semigroup
    fn mul(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

/// A monoid, which is a semigroup with an identity (unit) element.
pub trait Monoid: Semigroup {
    /// The multiplicative identity (unit) element of the monoid.
    fn unit(&mut self) -> Self::Elem;
}

/// A multiplicative group, which is a monoid where every element has an inverse.
pub trait Group: Monoid {
    /// The inverse element of the given element in a group.
    fn inv(&mut self, elem: &Self::Elem) -> Self::Elem;
}

/// A ring, which is a additive abelian group together with multiplicative semigroup that
/// distributes over the addition.
pub trait Ring: Algebra {
    /// The zero element (additive identity) of the ring.
    fn zero(&mut self) -> Self::Elem;

    /// The additive inverse of the given element in the ring.
    fn neg(&mut self, elem: &Self::Elem) -> Self::Elem;

    /// The additive abelian group operation of the ring.
    fn add(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;

    /// The multiplicative semigroup operation of the ring.
    fn mul(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

/// A unitary ring, which is a ring with a multiplicative unit element.
pub trait UnitaryRing: Ring {
    /// The multiplicative unit element of the ring.
    fn unit(&mut self) -> Self::Elem;
}

impl<A: BooleanAlgebra> Ring for A {
    fn zero(&mut self) -> Self::Elem {
        self.bot()
    }

    fn neg(&mut self, elem: &Self::Elem) -> Self::Elem {
        BooleanAlgebra::neg(self, elem)
    }

    fn add(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.add(elem0, elem1)
    }

    fn mul(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.meet(elem0, elem1)
    }
}

impl<A: BooleanAlgebra> UnitaryRing for A {
    fn unit(&mut self) -> Self::Elem {
        self.top()
    }
}

/// A set of elements where equality and other relations can be calculated.
pub trait Domain: Algebra {
    /// The logic used to perform propositional calculations.
    type Logic: BooleanAlgebra + Eq;

    /// Returns the underlying logic object of this domain.
    fn logic(&mut self) -> &mut Self::Logic;

    /// Checks if this object is a member of this domain.
    fn contains(&mut self, elem: &Self::Elem) -> <Self::Logic as Algebra>::Elem;

    /// Checks if the two objects represent the same element of the domain.
    fn equals(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Algebra>::Elem;
}

/// An arbitrary binary relation over a domain.
pub trait DirectedGraph: Domain {
    fn edge(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Algebra>::Elem;
}

/// A directed graph whose relation is reflexive, anti-symmetric and transitive.
pub trait PartialOrder: DirectedGraph {}
