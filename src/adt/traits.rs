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

use super::{BitVec, BooleanLogic, BooleanSolver, GenSlice, GenVec, Solver};

/// An arbitrary set of elements that can be representable by bit vectors.
pub trait Domain: Clone {
    /// Returns the number of bits used to represent the elements of this
    /// domain.
    fn num_bits(&self) -> usize;

    /// Verifies that the given bit vector is encoding a valid element of
    /// this domain.
    fn contains<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>;

    /// Checks if the two bit vectors are exactly the same. This offers a
    /// faster implementation than bitwise comparison, since it has to work
    /// only for valid bit patterns that encode elements.
    fn equals<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>;

    /// Adds a new variable to the given solver, which is just a list of
    /// fresh literals. It also enforces that the returned variable
    /// is contained in the domain, but adding the appropriate constraint.
    fn add_variable<LOGIC>(&self, logic: &mut LOGIC) -> Vec<LOGIC::Elem>
    where
        LOGIC: BooleanSolver,
    {
        let mut elem: Vec<LOGIC::Elem> = Vec::with_capacity(self.num_bits());
        for _ in 0..self.num_bits() {
            elem.push(logic.bool_add_variable());
        }
        let test = self.contains(logic, elem.slice());
        logic.bool_add_clause1(test);
        elem
    }

    /// Returns an object for formatting the given element.
    fn format<ELEM>(&self, elem: ELEM) -> Format<'_, Self, ELEM>
    where
        ELEM: GenSlice<Item = bool>,
    {
        Format { domain: self, elem }
    }

    /// Formats the given element using the provided formatter.
    fn display_elem<ELEM>(&self, f: &mut std::fmt::Formatter<'_>, elem: ELEM) -> std::fmt::Result
    where
        ELEM: GenSlice<Item = bool>,
    {
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
pub struct Format<'a, DOM, ELEM>
where
    DOM: Domain,
    ELEM: GenSlice<Item = bool>,
{
    domain: &'a DOM,
    elem: ELEM,
}

impl<'a, DOM, ELEM> std::fmt::Display for Format<'a, DOM, ELEM>
where
    DOM: Domain,
    ELEM: GenSlice<Item = bool>,
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
    fn index<ELEM>(&self, elem: ELEM) -> usize
    where
        ELEM: GenSlice<Item = bool>;
}

/// A domain with a reflexive, transitive and antisymmetric relation.
pub trait PartialOrder: Domain {
    /// Returns true if the first element is less than or equal to the
    /// second one in the partial order.
    fn leq<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>;

    /// Returns true if the first element is strictly less than the
    /// second one.
    fn less_than<LOGIC, ELEM>(&self, alg: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let test0 = self.leq(alg, elem0, elem1);
        let test1 = self.leq(alg, elem1, elem0);
        let test1 = alg.bool_not(test1);
        alg.bool_and(test0, test1)
    }

    /// Returns true if one of the elements is less than or equal to
    /// the other.
    fn comparable<LOGIC, ELEM>(&self, alg: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let test0 = self.leq(alg, elem0, elem1);
        let test1 = self.leq(alg, elem1, elem0);
        alg.bool_or(test0, test1)
    }
}

/// A partial order that has a largest and smallest element.
pub trait BoundedOrder: PartialOrder {
    /// Returns the largest element of the partial order.
    fn top(&self) -> BitVec;

    /// Returns the smallest element of the partial order.
    fn bottom(&self) -> BitVec;
}

/// A semilattice with a meet operation.
pub trait MeetSemilattice: PartialOrder {
    /// Calculates the meet (the largest lower bound) of
    /// a pair of elements
    fn meet<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>;
}

pub trait Lattice: MeetSemilattice {
    /// Calculates the join (the smallest upper bound) of
    /// a pair of elements.
    fn join<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>;
}

pub trait BooleanLattice: Lattice + BoundedOrder {
    /// Calculates the complement of the given element.
    fn complement<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>;
}

/// A binary relation between two domains
pub trait BinaryRelation<DOM0, DOM1>: Clone
where
    DOM0: Domain,
    DOM1: Domain,
{
    /// Returns the domain of the relation.
    fn domain(&self) -> &DOM0;

    /// Returns the co-domain of the relation.
    fn codomain(&self) -> &DOM1;

    /// Returns true if the two elements are related.
    fn related<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>;
}
