/*
* Copyright (C) 2019-2020, Miklos Maroti
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

//! An implementation of the free boolean algebra backed by a SAT solver.
//! This can be used to calculate with boolean terms and ask for a model
//! where a given set of terms are all true.

use std::fmt::Debug;
use std::iter;

use super::{create_solver, Literal, SatInterface};
use crate::genvec::{BitSlice, BitVec, Slice, Vector};

/// A boolean algebra supporting boolean calculation.
pub trait BooleanLogic {
    /// The element type of this bool algebra.
    type Elem: Copy;

    /// The type of vector holding the elements.
    type Vector: Debug + for<'a> Vector<Item = Self::Elem, Slice<'a> = Self::Slice<'a>>;

    /// The type of slices for the element vectors.
    type Slice<'a>: Slice<'a, Item = Self::Elem, Vector = Self::Vector>;

    /// Returns the logical true (top) element of the algebra.
    fn bool_unit(&self) -> Self::Elem {
        self.bool_lift(true)
    }

    /// Returns the logical false (bottom) element of the algebra.
    fn bool_zero(&self) -> Self::Elem {
        self.bool_lift(false)
    }

    /// Returns either the unit or zero element depending of the argument.
    fn bool_lift(&self, elem: bool) -> Self::Elem;

    /// Returns true if the element is always true.
    fn bool_is_unit(&self, elem: Self::Elem) -> bool;

    /// Returns true if the element is always false.
    fn bool_is_zero(&self, elem: Self::Elem) -> bool;

    /// Return the logical negation of the element.
    fn bool_not(&self, elem: Self::Elem) -> Self::Elem;

    /// Returns the logical or (lattice join) of a pair of elements.
    fn bool_or(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the exclusive or (boolean addition) of a pair of elements.
    fn bool_xor(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the logical and (lattice meet) of a pair of elements.
    fn bool_and(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp1 = self.bool_not(elem1);
        let tmp2 = self.bool_not(elem2);
        let tmp3 = self.bool_or(tmp1, tmp2);
        self.bool_not(tmp3)
    }

    /// Returns the logical equivalence of a pair of elements.
    fn bool_equ(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp = self.bool_not(elem1);
        self.bool_xor(tmp, elem2)
    }

    /// Returns the logical implication of a pair of elements.
    fn bool_imp(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp = self.bool_not(elem1);
        self.bool_or(tmp, elem2)
    }

    /// Returns the boolean sum of three values.
    fn bool_sum3(&mut self, elem1: Self::Elem, elem2: Self::Elem, elem3: Self::Elem) -> Self::Elem {
        let tmp = self.bool_xor(elem1, elem2);
        self.bool_xor(tmp, elem3)
    }

    /// Returns the majority of the given values.
    fn bool_maj(&mut self, elem1: Self::Elem, elem2: Self::Elem, elem3: Self::Elem) -> Self::Elem {
        let tmp1 = self.bool_and(elem1, elem2);
        let tmp2 = self.bool_and(elem1, elem3);
        let tmp3 = self.bool_and(elem2, elem3);
        let tmp4 = self.bool_or(tmp1, tmp2);
        self.bool_or(tmp3, tmp4)
    }

    /// Computes the conjunction of the elements.
    fn bool_fold_all<ITER>(&mut self, elems: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut result = self.bool_unit();
        for elem in elems {
            result = self.bool_and(result, elem);
        }
        result
    }

    /// Computes the disjunction of the elements.
    fn bool_fold_any<ITER>(&mut self, elems: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut result = self.bool_zero();
        for elem in elems {
            result = self.bool_or(result, elem);
        }
        result
    }

    /// Computes the boolean sum of the elements.
    fn bool_fold_sum<ITER>(&mut self, elems: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut result = self.bool_zero();
        for elem in elems {
            result = self.bool_xor(result, elem);
        }
        result
    }

    /// Computes the exactly one predicate over the given elements.
    fn bool_fold_one<ITER>(&mut self, elems: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut min1 = self.bool_zero();
        let mut min2 = self.bool_zero();
        for elem in elems {
            let tmp = self.bool_and(min1, elem);
            min2 = self.bool_or(min2, tmp);
            min1 = self.bool_or(min1, elem);
        }
        min2 = self.bool_not(min2);
        self.bool_and(min1, min2)
    }

    /// Computes the at most one predicate over the given elements.
    fn bool_fold_amo<ITER>(&mut self, elems: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut min1 = self.bool_zero();
        let mut min2 = self.bool_zero();
        for elem in elems {
            let tmp = self.bool_and(min1, elem);
            min2 = self.bool_or(min2, tmp);
            min1 = self.bool_or(min1, elem);
        }
        self.bool_not(min2)
    }

    /// Returns true if the two sequences are equal.
    fn bool_cmp_equ<ITER>(&mut self, pairs: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = (Self::Elem, Self::Elem)>,
    {
        let mut result = self.bool_unit();
        for (a, b) in pairs {
            let c = self.bool_equ(a, b);
            result = self.bool_and(result, c);
        }
        result
    }

    /// Returns true if the two sequences are not equal.
    fn bool_cmp_neq<ITER>(&mut self, pairs: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = (Self::Elem, Self::Elem)>,
    {
        let result = self.bool_cmp_equ(pairs);
        self.bool_not(result)
    }

    /// Returns true if the first sequence is lexicographically smaller
    /// than or equal to the second one.
    fn bool_cmp_leq<ITER>(&mut self, pairs: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = (Self::Elem, Self::Elem)>,
    {
        let mut result = self.bool_unit();
        for (a, b) in pairs {
            let a = self.bool_not(a);
            result = self.bool_maj(a, b, result);
        }
        result
    }

    /// Returns true if the first sequence is lexicographically smaller
    /// than the second one.
    fn bool_cmp_ltn<ITER>(&mut self, pairs: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = (Self::Elem, Self::Elem)>,
    {
        let mut result = self.bool_zero();
        for (a, b) in pairs {
            let a = self.bool_not(a);
            result = self.bool_maj(a, b, result);
        }
        result
    }

    /// Lifts each element to this boolean algebra.
    fn bool_lift_vec<ITER>(&self, elems: ITER) -> Vec<Self::Elem>
    where
        ITER: Iterator<Item = bool>,
    {
        elems.map(|elem| self.bool_lift(elem)).collect()
    }
}

/// The two element boolean algebra with native `bool` elements.
#[derive(Default, Debug)]
pub struct Logic();

impl BooleanLogic for Logic {
    type Elem = bool;

    type Vector = BitVec;

    type Slice<'a> = BitSlice<'a>;

    fn bool_unit(&self) -> Self::Elem {
        true
    }

    fn bool_zero(&self) -> Self::Elem {
        false
    }

    fn bool_lift(&self, elem: bool) -> Self::Elem {
        elem
    }

    fn bool_is_unit(&self, elem: Self::Elem) -> bool {
        elem
    }

    fn bool_is_zero(&self, elem: Self::Elem) -> bool {
        !elem
    }

    fn bool_not(&self, elem: Self::Elem) -> Self::Elem {
        !elem
    }

    fn bool_or(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 || elem2
    }

    fn bool_xor(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 ^ elem2
    }

    fn bool_and(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 && elem2
    }

    fn bool_equ(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 == elem2
    }

    fn bool_imp(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 <= elem2
    }
}

pub const LOGIC: Logic = Logic();

/// The free boolean algebra backed by a SAT solver.
#[derive(Debug)]
pub struct Solver {
    solver: Box<dyn SatInterface>,
    unit: Literal,
    zero: Literal,
}

impl Solver {
    /// Creates a new free boolean algebra.
    pub fn new(solver_name: &str) -> Self {
        let mut solver = create_solver(solver_name);
        let unit = solver.add_variable();
        let zero = solver.negate(unit);
        solver.add_clause(&[unit]);
        Solver { solver, unit, zero }
    }

    /// Returns the name of the solver
    pub fn get_name(&self) -> &'static str {
        self.solver.get_name()
    }

    /// Returns the number of variables in the solver.
    pub fn num_variables(&self) -> u32 {
        self.solver.num_variables() - 1
    }

    /// Returns the number of clauses in the solver.
    pub fn num_clauses(&self) -> usize {
        self.solver.num_clauses() - 1
    }
}

impl BooleanLogic for Solver {
    type Elem = Literal;

    type Vector = Vec<Literal>;

    type Slice<'a> = &'a [Literal];

    fn bool_unit(&self) -> Self::Elem {
        self.unit
    }

    fn bool_zero(&self) -> Self::Elem {
        self.zero
    }

    fn bool_lift(&self, elem: bool) -> Self::Elem {
        if elem {
            self.unit
        } else {
            self.zero
        }
    }

    fn bool_is_unit(&self, elem: Self::Elem) -> bool {
        elem == self.unit
    }

    fn bool_is_zero(&self, elem: Self::Elem) -> bool {
        elem == self.zero
    }

    fn bool_not(&self, elem: Self::Elem) -> Self::Elem {
        self.solver.negate(elem)
    }

    fn bool_or(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let not_elem2 = self.solver.negate(elem2);
        if elem1 == self.unit || elem2 == self.unit || elem1 == not_elem2 {
            self.unit
        } else if elem1 == self.zero || elem1 == elem2 {
            elem2
        } else if elem2 == self.zero {
            elem1
        } else {
            let not_elem1 = self.solver.negate(elem1);
            let elem3 = self.solver.add_variable();
            let not_elem3 = self.solver.negate(elem3);
            self.solver.add_clause(&[not_elem1, elem3]);
            self.solver.add_clause(&[not_elem2, elem3]);
            self.solver.add_clause(&[elem1, elem2, not_elem3]);
            elem3
        }
    }

    fn bool_xor(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let not_elem2 = self.solver.negate(elem2);
        if elem1 == self.zero {
            elem2
        } else if elem1 == self.unit {
            not_elem2
        } else if elem2 == self.zero {
            elem1
        } else if elem2 == self.unit {
            self.solver.negate(elem1)
        } else if elem1 == elem2 {
            self.zero
        } else if elem1 == not_elem2 {
            self.unit
        } else {
            let elem3 = self.solver.add_variable();
            self.solver.add_xor_clause(elem1, elem2, elem3);
            elem3
        }
    }
}

/// Constraint solving over a boolean algebra.
pub trait BooleanSolver: BooleanLogic + Sized {
    /// Adds a new variable to the solver
    fn bool_add_variable(&mut self) -> Self::Elem;

    /// Adds the given (disjunctive) clause to the solver.
    fn bool_add_clause(&mut self, clause: &[Self::Elem]);

    /// Adds a unary clause to the solver.
    fn bool_add_clause1(&mut self, lit0: Self::Elem) {
        self.bool_add_clause(&[lit0]);
    }

    /// Adds a binary clause to the solver.
    fn bool_add_clause2(&mut self, lit0: Self::Elem, lit1: Self::Elem) {
        self.bool_add_clause(&[lit0, lit1]);
    }

    /// Adds a ternary clause to the solver.
    fn bool_add_clause3(&mut self, lit0: Self::Elem, lit1: Self::Elem, lit2: Self::Elem) {
        self.bool_add_clause(&[lit0, lit1, lit2]);
    }

    /// Returns if the current set of clauses is solvable.
    fn bool_solvable(&mut self) -> bool;

    /// Runs the solver with the given assumptions and returns the value of
    /// the given literals if a solution is found.
    fn bool_find_one_model<ITER>(
        &mut self,
        assumptions: &[Self::Elem],
        literals: ITER,
    ) -> Option<BitVec>
    where
        ITER: Iterator<Item = Self::Elem>;

    /// Returns the number of models with respect to the given elements.
    fn bool_find_num_models_method1<ITER>(mut self, literals: ITER) -> usize
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut count = 0;
        let literals: Vec<Self::Elem> = literals.collect();
        let mut clause: Vec<Self::Elem> = Vec::with_capacity(literals.len());
        while let Some(result) = self.bool_find_one_model(&[], literals.copy_iter()) {
            count += 1;
            clause.clear();
            clause.extend(
                literals
                    .copy_iter()
                    .zip(result.into_iter())
                    .map(|(l, b)| self.bool_xor(self.bool_lift(b), l)),
            );
            self.bool_add_clause(&clause);
        }
        count
    }

    /// Returns the number of models with respect to the given literals.
    fn bool_find_num_models_method2<ITER>(mut self, literals: ITER) -> usize
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let literals: Vec<Self::Elem> = literals
            .chain([self.bool_unit(), self.bool_zero()].iter().copied())
            .collect();
        let len = literals.len();

        // bound variables
        let variables: Vec<Self::Elem> = (0..(2 * len)).map(|_| self.bool_add_variable()).collect();

        // lower bound
        let result = self.bool_cmp_ltn(variables.copy_iter().take(len).zip(literals.copy_iter()));
        self.bool_add_clause(&[result]);

        // upper bound
        let result = self.bool_cmp_ltn(literals.copy_iter().zip(variables.copy_iter().skip(len)));
        self.bool_add_clause(&[result]);

        let mut lower_bound: BitVec = iter::repeat_n(true, len - 2)
            .chain([false, false].iter().copied())
            .collect();
        let mut upper_bounds: BitVec = iter::repeat_n(false, len - 2)
            .chain([false, true].iter().copied())
            .collect();

        let mut count = 0;
        let mut assumptions: Vec<Self::Elem> = Vec::with_capacity(2 * len);
        while !upper_bounds.is_empty() {
            let end = upper_bounds.len();
            let last = end - len;
            assumptions.clear();
            assumptions.extend(
                variables
                    .copy_iter()
                    .take(len)
                    .zip(lower_bound.copy_iter())
                    .map(|(v, b)| self.bool_equ(self.bool_lift(b), v)),
            );
            assumptions.extend(
                variables
                    .copy_iter()
                    .skip(len)
                    .zip(upper_bounds.copy_iter().skip(last))
                    .map(|(v, b)| self.bool_equ(self.bool_lift(b), v)),
            );

            match self.bool_find_one_model(&assumptions, literals.copy_iter()) {
                None => {
                    lower_bound.clear();
                    lower_bound.extend(upper_bounds.copy_iter().skip(last));
                    upper_bounds.truncate(last);
                }
                Some(result) => {
                    count += 1;
                    assert_eq!(result.len(), len);
                    upper_bounds.extend(result.copy_iter());
                }
            }
        }

        count
    }
}

impl BooleanSolver for Solver {
    fn bool_add_variable(&mut self) -> Self::Elem {
        self.solver.add_variable()
    }

    fn bool_add_clause(&mut self, clause: &[Self::Elem]) {
        self.solver.add_clause(clause)
    }

    fn bool_solvable(&mut self) -> bool {
        self.solver.solve()
    }

    fn bool_find_one_model<ITER>(
        &mut self,
        assumptions: &[Self::Elem],
        literals: ITER,
    ) -> Option<BitVec>
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        if self.solver.solve_with(assumptions) {
            Some(literals.map(|e| self.solver.get_value(e)).collect())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_ops() {
        let mut alg = Logic();
        let a = alg.bool_unit();
        let b = alg.bool_not(a);
        assert_eq!(alg.bool_xor(a, b), a);
        assert_eq!(alg.bool_and(a, b), b);
    }

    #[test]
    fn solver() {
        let mut alg = Solver::new("");
        let a = alg.bool_add_variable();
        let b = alg.bool_add_variable();
        let c = alg.bool_and(a, b);
        alg.bool_add_clause(&[c]);
        let s = alg.bool_find_one_model(&[], [a, b].iter().copied());
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.len(), 2);
        assert_eq!(s.get(0), true);
        assert_eq!(s.get(1), true);
    }
}
