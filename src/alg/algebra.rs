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

    /// Returns the size of the algebra if it known and a small value,
    fn size(&self) -> Option<usize>;

    /// Obtains one of the elements of an enumerable algebra.
    fn element(&mut self, index: usize) -> Self::Elem;
}

/// A lattice, which is an ordered set where every two elements have a largest lower bound
/// and a smallest upper bound.
pub trait Lattice: Algebra {
    /// The meet (largest lower bound) of two elements in the lattice.
    fn meet(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;

    /// The join (least upper bound) of two elements in the lattice.
    fn join(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

/// A bounded lattice, which is a lattice with a smallest (zero) and largest (unit) element.
pub trait BoundedLattice: Lattice {
    /// The smallest element of the lattice.
    fn zero(&mut self) -> Self::Elem;

    /// The largest element of the lattice.
    fn unit(&mut self) -> Self::Elem;
}

/// A boolean algebra, which is a complemented distributive bounded lattice.
pub trait BooleanAlgebra: BoundedLattice {
    /// Returns the complement of the given element.
    fn complement(&mut self, elem: &Self::Elem) -> Self::Elem;

    /// The symmetric difference of two elements.
    fn difference(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        let comp0 = self.complement(elem0);
        let comp1 = self.complement(elem1);
        let elem3 = self.meet(elem0, &comp1);
        let elem4 = self.meet(&comp0, elem1);
        self.join(&elem3, &elem4)
    }
}

/// A semigroup, which is domain with an associative binary operation.
pub trait Semigroup: Algebra {
    /// The product of two elements in the semigroup
    fn product(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

/// A monoid, which is a semigroup with an identity (unit) element.
pub trait Monoid: Semigroup {
    /// The multiplicative identity (unit) element of the monoid.
    fn identity(&mut self) -> Self::Elem;
}

/// A multiplicative group, which is a monoid where every element has an inverse.
pub trait Group: Monoid {
    /// The inverse element of the given element in a group.
    fn inverse(&mut self, elem: &Self::Elem) -> Self::Elem;
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
