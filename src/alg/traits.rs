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
    /// Returns the number of bits used to represent the elements of this
    /// domain.
    fn num_bits(&self) -> usize;

    fn lift<LOGIC>(&self, logic: &mut LOGIC, elem: BitSlice) -> LOGIC::Vector
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
    /// is contained in the domain, but adding the appropriate constraint.
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

    /// Returns an object for formatting the given element.
    fn format<'a>(&'a self, elem: BitSlice<'a>) -> Format<'a, Self> {
        Format { domain: self, elem }
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

    /// Finds an element of this domain if it has one.
    fn find_element(&self) -> Option<BitVec> {
        let mut solver = Solver::new("");
        let elem = self.add_variable(&mut solver);
        let test = self.contains(&mut solver, elem.slice());
        solver.bool_add_clause(&[test]);
        solver.bool_find_one_model(&[], elem.copy_iter())
    }
}

/// A helper structure for displaying domain elements.
pub struct Format<'a, DOM>
where
    DOM: Domain,
{
    domain: &'a DOM,
    elem: BitSlice<'a>,
}

impl<'a, DOM> std::fmt::Display for Format<'a, DOM>
where
    DOM: Domain,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.domain.display_elem(f, self.elem)
    }
}

/// A domain where the elements can be counted and indexed.
pub trait Countable: Domain {
    /// Returns the number of elements of the domain.
    fn size(&self) -> usize;

    /// Returns the given element of the domain.
    fn elem(&self, index: usize) -> BitVec;

    /// Returns the index of the given element.
    fn index(&self, elem: BitSlice<'_>) -> usize;
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
}

/// A domain with a reflexive, transitive and antisymmetric relation.
pub trait PartialOrder: DirectedGraph {
    /// Returns true if the first element is strictly less than the
    /// second one.
    fn is_less<LOGIC>(
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
    fn comparable<LOGIC>(
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
    fn top(&self) -> BitVec;

    /// Returns true if the given element is the top one.
    fn is_top<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;

    /// Returns the smallest element of the partial order.
    fn bottom(&self) -> BitVec;

    /// Returns true if the given element is the bottom one.
    fn is_bottom<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;
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

/// A binary relation between two domains
pub trait BipartiteGraph<DOM0, DOM1>: Clone
where
    DOM0: Domain,
    DOM1: Domain,
{
    /// Returns the domain of the relation.
    fn domain(&self) -> &DOM0;

    /// Returns the co-domain of the relation.
    fn codomain(&self) -> &DOM1;

    /// Returns true if the two elements are related.
    fn edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;
}

/// A domain of function from a fixed domain and codomain.
pub trait Functions: Domain {
    /// Returns the arity (rank) of all functions in the domain.
    fn arity(&self) -> usize;

    /// Returns the domain of functions with the given arity.
    fn change(&self, arity: usize) -> Self;

    /// Creates a new function of the given arity from an old function with
    /// permuted, identified and/or new dummy coordinates. The mapping is a
    /// vector of length of the arity of the original element with entries
    /// identifying the matching coordinates in the new function.
    fn polymer<'a, ELEM>(&self, elem: ELEM, arity: usize, mapping: &[usize]) -> ELEM::Vec
    where
        ELEM: Slice<'a>;

    /// Returns the unary function with all variables identified.
    fn identify<'a, ELEM>(&self, elem: ELEM) -> ELEM::Vec
    where
        ELEM: Slice<'a>,
    {
        assert!(self.arity() >= 1);
        self.polymer(elem, 1, &vec![0; self.arity()])
    }

    /// Reverses all coordinates of the function.
    fn converse<'a, ELEM>(&self, elem: ELEM) -> ELEM::Vec
    where
        ELEM: Slice<'a>,
    {
        let map: Vec<usize> = (0..self.arity()).rev().collect();
        self.polymer(elem, map.len(), &map)
    }
}

/// A domain of relations, which are functions to the BOOLEAN domain.
pub trait Relations: Functions + BooleanLattice {
    /// Returns the relation that is true if and only if all arguments are
    /// the same. This method panics if the arity is zero.
    fn get_diagonal(&self) -> BitVec;

    /// Checks if the given relation is the diagonal relation (only the
    /// elements in the diagonal are set).
    fn is_diagonal<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;

    /// Returns a unary relation containing only the given tuple. This
    /// method panics if the number of elements in the tuple does not
    /// match the arity of the domain.
    fn get_singleton(&self, elem: &[BitSlice<'_>]) -> BitVec;

    /// Checks if the given element is a singleton.
    fn is_singleton<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using the logical and operation.
    fn fold_all<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using the logical or operation.
    fn fold_any<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic;
}

/// A domain of binary relations.
pub trait BinaryRelations: Relations {
    /// Checks if the given binary relation is reflexive.
    fn is_reflexive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(self.arity(), 2);
        let elem = self.identify(elem);
        let dom1 = self.change(1);
        dom1.is_top(logic, elem.slice())
    }

    /// Returns true if the given binary relation is symmetric.
    fn is_symmetric<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let conv = self.polymer(elem, 2, &[1, 0]);
        let elem = self.implies(logic, elem, conv.slice());
        self.is_top(logic, elem.slice())
    }

    /// Checks if the given binary relation is antisymmetric.
    fn is_antisymmetric<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let conv = self.polymer(elem, 2, &[1, 0]);
        let elem = self.meet(logic, elem, conv.slice());
        let diag = self.lift(logic, self.get_diagonal().slice());
        self.is_edge(logic, elem.slice(), diag.slice())
    }

    /// Returns the composition of the given binary relations.
    fn compose<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(self.arity(), 2);
        let dom3 = self.change(3);
        let elem0: LOGIC::Vector = self.polymer(elem0, 3, &[1, 0]);
        let elem1: LOGIC::Vector = self.polymer(elem1, 3, &[0, 2]);
        let elem2 = dom3.meet(logic, elem0.slice(), elem1.slice());
        dom3.fold_any(logic, elem2.slice())
    }

    /// Returns true if the given binary relation is transitive.
    fn is_transitive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let comp = self.compose(logic, elem, elem);
        let elem = self.implies(logic, comp.slice(), elem);
        self.is_top(logic, elem.slice())
    }

    /// Returns true if the given binary relation is an equivalence relation.
    fn is_equivalence<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test0 = self.is_reflexive(logic, elem);
        let test1 = self.is_symmetric(logic, elem);
        let test2 = self.is_transitive(logic, elem);
        let test3 = logic.bool_and(test0, test1);
        logic.bool_and(test2, test3)
    }

    /// Returns true if the given binary relation is a partial order relation.
    fn is_partial_order<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test0 = self.is_reflexive(logic, elem);
        let test1 = self.is_antisymmetric(logic, elem);
        let test2 = self.is_transitive(logic, elem);
        let test3 = logic.bool_and(test0, test1);
        logic.bool_and(test2, test3)
    }
}
