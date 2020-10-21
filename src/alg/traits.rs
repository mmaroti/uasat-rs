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

use super::TwoElementAlg;

/// An arbitrary set of elements over a fixed type.
pub trait Domain {
    /// The type of objects representing the elements of this algebra. Not all objects are
    /// valid elements, and structurally different objects may represent the same element.
    type Elem: Clone;

    /// The logic used to perform propositional calculations.
    type Logic: BooleanAlgebra;

    /// Returns the underlying logic object of this domain.
    fn logic(&self) -> &Self::Logic;

    /// Checks if this object is a member of this domain.
    fn contains(&self, elem: &Self::Elem) -> <Self::Logic as Domain>::Elem;

    /// Checks if the two objects represent the same element of the domain.
    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem;
}

/// A domain with classic `bool` logical values.
pub trait ClassicalDomain: Domain<Logic = TwoElementAlg> {}

/// An arbitrary binary relation over a domain.
pub trait DirectedGraph: Domain {
    /// Checks if there is an edge from the first element to the second.
    fn edge(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem;

    /// Checks if there is an edge forward but not backward.
    fn forward(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        let temp = self.logic().not(&self.edge(elem1, elem0));
        self.logic().meet(&self.edge(elem0, elem1), &temp)
    }
}

/// A directed graph whose relation is reflexive, anti-symmetric and transitive.
pub trait PartialOrder: DirectedGraph {}

/// A bounded partial order with a smallest and largest element.
pub trait BoundedPartialOrder: PartialOrder {
    /// The smallest (bottom) element in the partial order.
    fn bot(&self) -> Self::Elem;

    /// The largest (top) element in the partial order.
    fn top(&self) -> Self::Elem;
}

/// A lattice, which is an ordered set where every two elements have a largest lower bound
/// and a smallest upper bound.
pub trait Lattice: PartialOrder {
    /// The meet (largest lower bound) of two elements in the lattice.
    fn meet(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;

    /// The join (least upper bound) of two elements in the lattice.
    fn join(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

/// A boolean algebra, which is a complemented distributive bounded lattice.
pub trait BooleanAlgebra: Lattice + BoundedPartialOrder {
    /// Returns the complement of the given element.
    fn not(&self, elem: &Self::Elem) -> Self::Elem;

    /// The symmetric difference of two elements.
    fn xor(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        let elem3 = self.meet(elem0, &self.not(elem1));
        let elem4 = self.meet(&self.not(elem0), elem1);
        self.join(&elem3, &elem4)
    }

    /// The logical implication of the given elements(disjunction of the negation of the first
    /// with the second one).
    fn imp(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.join(&self.not(elem0), elem1)
    }

    /// The logical equivalence of the given elements (the symmetric difference of the negaton of
    /// the first element with the second one).
    fn equ(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.xor(&self.not(elem0), elem1)
    }
}

/// An additive Abelian (commutative) group.
pub trait AdditiveGroup: Domain {
    /// The additive identity element.
    fn zero(&self) -> Self::Elem;

    /// The additive inverse of the given element.
    fn neg(&self, elem: &Self::Elem) -> Self::Elem;

    /// The additive commutative group operation.
    fn add(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;

    /// Subtracts the second element from the first.
    fn sub(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.add(elem0, &self.neg(elem1))
    }
}

/// A multiplicative semigroup, which is domain with an associative binary operation.
pub trait Semigroup: Domain {
    /// The product of two elements in the semigroup
    fn mul(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

/// A monoid, which is a semigroup with an identity (unit) element.
pub trait Monoid: Semigroup {
    /// The multiplicative identity (unit) element of the monoid.
    fn unit(&self) -> Self::Elem;
}

/// A multiplicative group, which is a monoid where every element has an inverse.
pub trait Group: Monoid {
    /// The inverse element of the given element.
    fn inv(&self, elem: &Self::Elem) -> Self::Elem;
}

/// A ring, which is a additive Abelian group together with multiplicative semigroup that
/// distributes over the addition.
pub trait Ring: Semigroup + AdditiveGroup {}

/// A unitary ring, which is a ring with a multiplicative unit element.
pub trait UnitaryRing: Ring + Monoid {}

/// A field, which is a commutative unitary ring where every non-zero element has a multiplicative
/// inverse.
pub trait Field: UnitaryRing {
    /// Returns the multiplicative inverse of the given non-zero element. To make this operation
    /// total, it returns zero for the zero element.
    fn inv(&self, elem0: &Self::Elem) -> Self::Elem;
}
