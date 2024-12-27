/*
* Copyright (C) 2022-2023, Miklos Maroti
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

use std::fmt::Debug;

use super::{BitSlice, BitVec, BooleanLogic, BooleanSolver, Slice, Solver, Vector};

/// An arbitrary set of elements that can be representable by bit vectors.
pub trait Domain: Clone + PartialEq + Debug {
    /// Returns the number of bits used to represent the elements of the
    /// domain.
    fn num_bits(&self) -> usize;

    /// Returns an object for formatting the given element.
    fn format<'a>(&'a self, elem: BitSlice<'a>) -> Format<'a, Self> {
        Format { base: self, elem }
    }

    /// Formats the given element using the provided formatter.
    fn display_elem(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        elem: BitSlice<'_>,
    ) -> std::fmt::Result {
        assert!(elem.len() == self.num_bits());
        for v in elem.copy_iter() {
            write!(f, "{}", if v { '1' } else { '0' })?;
        }
        Ok(())
    }

    /// Returns an element of the domain, if it has one.
    fn find_element(&self) -> Option<BitVec> {
        let mut solver = Solver::new("");
        let elem = self.add_variable(&mut solver);
        let test = self.contains(&mut solver, elem.slice());
        solver.bool_add_clause(&[test]);
        solver.bool_find_one_model(&[], elem.copy_iter())
    }

    /// Lifts the given bool vector to the logic associated with the domain.
    fn lift<LOGIC>(&self, logic: &LOGIC, elem: BitSlice) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut result: LOGIC::Vector = Vector::with_capacity(elem.len());
        for a in elem.copy_iter() {
            result.push(logic.bool_lift(a));
        }
        result
    }

    /// Verifies that the given bit vector is encoding a valid element of
    /// this domain.
    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;

    /// Checks if the two bit vectors are exactly the same. This offers a
    /// faster implementation than bitwise comparison, since it has to work
    /// only for valid bit patterns that encode elements.
    fn equals<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;

    /// Adds a new variable to the given solver, which is just a list of
    /// fresh literals. It also enforces that the returned variable
    /// is contained in the domain buy adding the appropriate constraint.
    fn add_variable<LOGIC>(&self, logic: &mut LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanSolver,
    {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for _ in 0..self.num_bits() {
            elem.push(logic.bool_add_variable());
        }
        let test = self.contains(logic, elem.slice());
        logic.bool_add_clause1(test);
        elem
    }
}

/// A helper structure for displaying domain elements.
pub struct Format<'a, BASE>
where
    BASE: Domain,
{
    base: &'a BASE,
    elem: BitSlice<'a>,
}

impl<BASE> std::fmt::Display for Format<'_, BASE>
where
    BASE: Domain,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.display_elem(f, self.elem)
    }
}

/// A domain where the elements can be counted and indexed.
pub trait Indexable: Domain {
    /// Returns the number of elements of the domain.
    fn size(&self) -> usize;

    /// Returns the given element of the domain.
    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;

    /// Returns the index of the given element.
    fn get_index(&self, elem: BitSlice<'_>) -> usize;

    /// Returns the one hot encoding of the given element.
    fn onehot<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut result: LOGIC::Vector = Vector::with_capacity(self.size());
        for index in 0..self.size() {
            let value = self.get_elem(logic, index);
            result.push(self.equals(logic, elem, value.slice()));
        }
        result
    }
}

/// A directed graph on a domain.
pub trait DirectedGraph: Domain {
    /// Returns true if there is an edge from the first element to the second.
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;

    /// Returns true if this directed graph is reflexive
    /// by constructing a suitable SAT problem and solving it.
    fn test_reflexivity(&self) -> bool {
        let mut logic = Solver::new("");
        let elem = self.add_variable(&mut logic);
        let test = self.is_edge(&mut logic, elem.slice(), elem.slice());
        logic.bool_add_clause1(logic.bool_not(test));
        !logic.bool_solvable()
    }

    /// Returns true if this directed graph is symmertic
    /// by constructing a suitable SAT problem and solving it.
    fn test_symmetricity(&self) -> bool {
        let mut logic = Solver::new("");
        let elem0 = self.add_variable(&mut logic);
        let elem1 = self.add_variable(&mut logic);
        let test = self.is_edge(&mut logic, elem0.slice(), elem1.slice());
        logic.bool_add_clause1(test);
        let test = self.is_edge(&mut logic, elem1.slice(), elem0.slice());
        logic.bool_add_clause1(logic.bool_not(test));
        !logic.bool_solvable()
    }

    /// Returns true if this directed graph is antisymmertic
    /// by constructing a suitable SAT problem and solving it.
    fn test_antisymmetricity(&self) -> bool {
        let mut logic = Solver::new("");
        let elem0 = self.add_variable(&mut logic);
        let elem1 = self.add_variable(&mut logic);
        let test = self.is_edge(&mut logic, elem0.slice(), elem1.slice());
        logic.bool_add_clause1(test);
        let test = self.is_edge(&mut logic, elem1.slice(), elem0.slice());
        logic.bool_add_clause1(test);
        let test = self.equals(&mut logic, elem0.slice(), elem1.slice());
        logic.bool_add_clause1(logic.bool_not(test));
        !logic.bool_solvable()
    }

    /// Returns true if this directed graph is transitive
    /// by constructing a suitable SAT problem and solving it.
    fn test_transitivity(&self) -> bool {
        let mut logic = Solver::new("");
        let elem0 = self.add_variable(&mut logic);
        let elem1 = self.add_variable(&mut logic);
        let elem2 = self.add_variable(&mut logic);
        let test = self.is_edge(&mut logic, elem0.slice(), elem1.slice());
        logic.bool_add_clause1(test);
        let test = self.is_edge(&mut logic, elem1.slice(), elem2.slice());
        logic.bool_add_clause1(test);
        let test = self.is_edge(&mut logic, elem0.slice(), elem2.slice());
        logic.bool_add_clause1(logic.bool_not(test));
        !logic.bool_solvable()
    }

    /// Returns true if this directed graph is an equivalence relation
    /// by constructing suitable SAT problems and solving them.
    fn test_equivalence(&self) -> bool {
        self.test_reflexivity() && self.test_symmetricity() && self.test_transitivity()
    }

    /// Returns true if this directed graph is a partial order
    /// by constructing suitable SAT problems and solving them.
    fn test_partial_order(&self) -> bool {
        self.test_reflexivity() && self.test_antisymmetricity() && self.test_transitivity()
    }
}

/// A domain with a reflexive, transitive and antisymmetric relation.
pub trait PartialOrder: DirectedGraph {
    /// Returns true if the first element is strictly less than the
    /// second one.
    fn is_less_than<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test0 = self.is_edge(logic, elem0, elem1);
        let test1 = self.is_edge(logic, elem1, elem0);
        let test1 = logic.bool_not(test1);
        logic.bool_and(test0, test1)
    }

    /// Returns true if one of the elements is less than or equal to
    /// the other.
    fn is_comparable<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test0 = self.is_edge(logic, elem0, elem1);
        let test1 = self.is_edge(logic, elem1, elem0);
        logic.bool_or(test0, test1)
    }
}

/// A partial order that has a largest and smallest element.
pub trait BoundedOrder: PartialOrder {
    /// Returns the largest element of the partial order.
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;

    /// Returns true if the given element is the top one.
    fn is_top<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let top = self.get_top(logic);
        self.equals(logic, elem, top.slice())
    }

    /// Returns the smallest element of the partial order.
    fn get_bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;

    /// Returns true if the given element is the bottom one.
    fn is_bottom<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let bottom = self.get_bottom(logic);
        self.equals(logic, elem, bottom.slice())
    }
}

/// A semilattice with a meet operation.
pub trait MeetSemilattice: PartialOrder {
    /// Calculates the meet (the largest lower bound) of
    /// a pair of elements
    fn meet<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;
}

/// A domain with lattice operations.
pub trait Lattice: MeetSemilattice {
    /// Calculates the join (the smallest upper bound) of
    /// a pair of elements.
    fn join<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;
}

/// A domain with boolean algebra operations.
pub trait BooleanLattice: Lattice + BoundedOrder {
    /// Calculates the complement of the given element.
    fn complement<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;

    /// Returns the logical implication between the two elements.
    fn implies<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let elem0 = self.complement(logic, elem0);
        self.join(logic, elem0.slice(), elem1)
    }
}

/// A domain with a associative binary operation.
pub trait Semigroup: Domain {
    /// Returns the product of the given two elements.
    fn product<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;
}

pub trait Monoid: Semigroup {
    /// Returns the identity element of the monoid.
    fn get_identity<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;

    /// Returns true if the given element is the bottom one.
    fn is_identity<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let unit = self.get_identity(logic);
        self.equals(logic, elem, unit.slice())
    }
}

pub trait Group: Monoid {
    /// Calculates the inverse of the given element.
    fn inverse<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;
}

/// An arbitrary n-ary operation on a domain.
pub trait Operation {
    /// The domain of the elements.
    type Domain: Domain;

    /// The domain of the operation. If the operation is multiary, then this
    /// domain will be a product domain.
    fn domain(&self) -> &Self::Domain;

    /// The arity of the operation.
    fn arity(&self) -> usize;

    /// Evaluates the operation at the given element.
    fn evaluate<LOGIC>(&self, logic: &mut LOGIC, args: &[LOGIC::Slice<'_>]) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;
}

/// An arbitrary n-ary relation on a domain.
pub trait Relation {
    /// The domain of the elements.
    type Domain: Domain;

    /// The domain of the operation. If the operation is multiary, then this
    /// domain will be a product domain.
    fn domain(&self) -> &Self::Domain;

    /// The arity of the operation.
    fn arity(&self) -> usize;

    /// Evaluates the operation at the given element.
    fn related<LOGIC>(&self, logic: &mut LOGIC, args: &[LOGIC::Slice<'_>]) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;
}

pub trait Structure2: Domain {
    /// The type of relations in this structure.
    type Relation: Relation<Domain = Self>;

    /// Returns the relations of this structure.
    fn relations(&self) -> &[Self::Relation];
}

/// A binary relation between two domains.
pub trait BipartiteGraph {
    /// The type of the first domain of the bipartite graph.
    type Domain0: Domain;

    /// The the type of the second domain of the bipartite graph.
    type Domain1: Domain;

    /// Returns the first domain of the bipartite graph.
    fn dom0(&self) -> &Self::Domain0;

    /// Returns the second domain of the bipartite graph.
    fn dom1(&self) -> &Self::Domain1;

    /// Returns true if the two elements are related, the first
    /// from the domain, the second from the codomain.
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;
}
