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

/// The representation of an arbitrary set of elements over a fixed type.
pub trait Domain {
    /// The logic used to perform propositional calculations.
    type Logic: BooleanAlgebra;

    /// Returns the underlying logic object of this domain.
    fn logic(&self) -> &Self::Logic;

    /// The type of elements of this algebra.
    type Elem: Clone;

    /// Checks if this object is a member of this domain.
    fn contains(&self, elem: &Self::Elem) -> <Self::Logic as Domain>::Elem;

    /// Checks if the two objects represent the same element of the domain.
    fn equals(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> <Self::Logic as Domain>::Elem;
}

/// An arbitrary binary relation over a domain.
pub trait DirectedGraph: Domain {
    fn related(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> <Self::Logic as Domain>::Elem;
}

/// A directed graph whose relation is reflexive, anti-symmetric and transitive.
pub trait PartialOrder: DirectedGraph {}

/// A bounded lattice which is also a partial order with the less than or equals relation.
pub trait BoundedLattice: PartialOrder {
    /// The largest element of the lattice.
    fn unit(&self) -> Self::Elem;

    /// The smallest element of the lattice.
    fn zero(&self) -> Self::Elem;

    /// The meet (largest lower bound) of two elements in the lattice.
    fn meet(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// The join (least upper bound) of two elements in the lattice.
    fn join(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;
}

/// A boolean algebra which is also a bounded distributive lattice.
pub trait BooleanAlgebra: BoundedLattice {
    /// Returns the complement of the given element.
    fn complement(&self, elem: &Self::Elem) -> Self::Elem;

    /// The symmetric difference of two elements.
    fn difference(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let elem3 = self.meet(elem1, &self.complement(elem2));
        let elem4 = self.meet(&self.complement(elem1), elem2);
        self.join(&elem3, &elem4)
    }
}

/// A semigroup, which is an associative groupoid.
pub trait Semigroup: Domain {
    /// The product of two elements in the semigroup
    fn product(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;
}

/// A monoid, which is a semigroup with an identity (or unit) element.
pub trait Monoid: Semigroup {
    /// The multiplicative identity (unit) element of the monoid.
    fn identity(&self) -> Self::Elem;
}

/// A multiplicative group, which is a monoid where every element has an inverse.
pub trait Group: Monoid {
    /// The inverse element of the given element in a group.
    fn inverse(&self, elem: &Self::Elem) -> Self::Elem;
}
