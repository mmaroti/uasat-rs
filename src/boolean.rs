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

use super::genvec;
use super::solver;
use std::iter;

/// A boolean algebra supporting boolean calculation.
pub trait BoolAlg {
    /// The element type of this bool algebra.
    type Elem: Clone;

    /// Returns the logical true (top) element of the algebra.
    fn bool_unit(self: &Self) -> Self::Elem {
        self.bool_lift(true)
    }

    /// Returns the logical false (bottom) element of the algebra.
    fn bool_zero(self: &Self) -> Self::Elem {
        self.bool_lift(false)
    }

    /// Returns either the unit or zero element depending of the argument.
    fn bool_lift(self: &Self, elem: bool) -> Self::Elem;

    /// Return the logical negation of the element.
    fn bool_not(self: &mut Self, elem: Self::Elem) -> Self::Elem;

    /// Returns the logical or (lattice join) of a pair of elements.
    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the exclusive or (boolean addition) of a pair of elements.
    fn bool_xor(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the logical and (lattice meet) of a pair of elements.
    fn bool_and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp1 = self.bool_not(elem1);
        let tmp2 = self.bool_not(elem2);
        let tmp3 = self.bool_or(tmp1, tmp2);
        self.bool_not(tmp3)
    }

    /// Returns the logical equivalence of a pair of elements.
    fn bool_equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp = self.bool_not(elem1);
        self.bool_xor(tmp, elem2)
    }

    /// Returns the logical implication of a pair of elements.
    fn bool_imp(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp = self.bool_not(elem1);
        self.bool_or(tmp, elem2)
    }

    /// Returns the boolean sum of three values.
    fn bool_sum3(
        self: &mut Self,
        elem1: Self::Elem,
        elem2: Self::Elem,
        elem3: Self::Elem,
    ) -> Self::Elem {
        let tmp = self.bool_xor(elem1, elem2);
        self.bool_xor(tmp, elem3)
    }

    /// Returns the majority of the given values.
    fn bool_maj(
        self: &mut Self,
        elem1: Self::Elem,
        elem2: Self::Elem,
        elem3: Self::Elem,
    ) -> Self::Elem {
        let tmp1 = self.bool_and(elem1.clone(), elem2.clone());
        let tmp2 = self.bool_and(elem1, elem3.clone());
        let tmp3 = self.bool_and(elem2, elem3);
        let tmp4 = self.bool_or(tmp1, tmp2);
        self.bool_or(tmp3, tmp4)
    }

    /// Computes the conjunction of the elements.
    fn bool_fold_all<ITER>(self: &mut Self, elems: ITER) -> Self::Elem
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
    fn bool_fold_any<ITER>(self: &mut Self, elems: ITER) -> Self::Elem
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
    fn bool_fold_sum<ITER>(self: &mut Self, elems: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut result = self.bool_zero();
        for elem in elems {
            result = self.bool_xor(result, elem);
        }
        result
    }

    fn bool_fold_equ<ITER>(self: &mut Self, pairs: ITER) -> Self::Elem
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

    fn bool_fold_neq<ITER>(self: &mut Self, pairs: ITER) -> Self::Elem
    where
        ITER: Iterator<Item = (Self::Elem, Self::Elem)>,
    {
        let mut result = self.bool_zero();
        for (a, b) in pairs {
            let c = self.bool_xor(a, b);
            result = self.bool_or(result, c);
        }
        result
    }

    fn bool_fold_leq<ITER>(self: &mut Self, pairs: ITER) -> Self::Elem
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

    fn bool_fold_ltn<ITER>(self: &mut Self, pairs: ITER) -> Self::Elem
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
}

/// The trivial 1-element boolean algebra over the unit `()` element.
pub struct Trivial();

impl BoolAlg for Trivial {
    type Elem = ();

    fn bool_lift(self: &Self, _elem: bool) -> Self::Elem {}

    fn bool_not(self: &mut Self, _elem: Self::Elem) -> Self::Elem {}

    fn bool_or(self: &mut Self, _elem1: Self::Elem, _elem2: Self::Elem) -> Self::Elem {}

    fn bool_xor(self: &mut Self, _elem1: Self::Elem, _elem2: Self::Elem) -> Self::Elem {}
}

/// The two element boolean algebra with native `bool` elements.
#[derive(Default, Debug)]
pub struct Boolean();

impl BoolAlg for Boolean {
    type Elem = bool;

    fn bool_lift(self: &Self, elem: bool) -> Self::Elem {
        elem
    }

    fn bool_not(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        !elem
    }

    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 || elem2
    }

    fn bool_xor(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 ^ elem2
    }

    fn bool_and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 && elem2
    }

    fn bool_equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 == elem2
    }

    fn bool_imp(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 <= elem2
    }
}

/// The free boolean algebra backed by a SAT solver.
#[derive(Debug)]
pub struct Solver {
    solver: Box<dyn solver::Solver>,
    unit: solver::Literal,
    zero: solver::Literal,
}

impl Solver {
    /// Creates a new free boolean algebra.
    pub fn new(solver_name: &str) -> Self {
        let mut solver = solver::create_solver(solver_name);
        let unit = solver.add_variable();
        let zero = solver.negate(unit);
        solver.add_clause(&[unit]);
        Solver { solver, unit, zero }
    }

    pub fn get_name(self: &Self) -> &'static str {
        self.solver.get_name()
    }
}

impl BoolAlg for Solver {
    type Elem = solver::Literal;

    fn bool_lift(self: &Self, elem: bool) -> Self::Elem {
        if elem {
            self.unit
        } else {
            self.zero
        }
    }

    fn bool_not(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        self.solver.negate(elem)
    }

    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
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

    fn bool_xor(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
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
pub trait BoolSat: BoolAlg {
    /// Adds a new variable to the solver
    fn bool_add_variable(self: &mut Self) -> Self::Elem;

    /// Adds the given (disjunctive) clause to the solver.
    fn bool_add_clause(self: &mut Self, elems: &[Self::Elem]);

    /// Runs the solver and returns the value of the given elements if a
    /// solution is found.
    fn bool_find_one_model<ITER>(self: &mut Self, elems: ITER) -> Option<genvec::VectorFor<bool>>
    where
        ITER: Iterator<Item = Self::Elem>;

    /// Returns the number of models with respect to the given elements.
    fn bool_find_num_models_method1<ITER>(self: &mut Self, elems: ITER) -> usize
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut count = 0;
        let elems: Vec<Self::Elem> = elems.collect();
        let mut clause: Vec<Self::Elem> = Vec::with_capacity(elems.len());
        while let Some(result) = self.bool_find_one_model(elems.iter().cloned()) {
            count += 1;
            clause.clear();
            clause.extend(elems.iter().zip(result.into_iter()).map(|(e, b)| {
                let b = self.bool_lift(b);
                self.bool_xor(b, e.clone())
            }));
            self.bool_add_clause(&clause);
        }
        count
    }

    /// Returns the number of models with respect to the given literals.
    fn bool_find_num_models_method2<ITER>(self: &mut Self, elems: ITER) -> usize
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        let mut elems: Vec<Self::Elem> = elems.collect();
        let len = elems.len();
        let _limit_vars: Vec<Self::Elem> = (0..(2 * len + 2))
            .map(|_| self.bool_add_variable())
            .collect();
        let _vals: Vec<bool> = iter::repeat(true)
            .take(len)
            .chain(iter::repeat(false).take(len + 1))
            .chain(iter::once(true))
            .collect();

        elems.push(self.bool_unit());

        let lower_lit: Vec<Self::Elem> = (0..(elems.len() + 1))
            .map(|_| self.bool_add_variable())
            .collect();

        elems.push(self.bool_unit());
        let result = self.bool_fold_ltn(lower_lit.iter().cloned().zip(elems.iter().cloned()));
        self.bool_add_clause(&[result]);
        elems.pop();

        let upper_lit: Vec<Self::Elem> = (0..(elems.len() + 1))
            .map(|_| self.bool_add_variable())
            .collect();

        elems.push(self.bool_zero());
        let result = self.bool_fold_ltn(elems.iter().cloned().zip(upper_lit.iter().cloned()));
        self.bool_add_clause(&[result]);
        elems.pop();

        0
    }
}

impl BoolSat for Solver {
    fn bool_add_variable(self: &mut Self) -> Self::Elem {
        self.solver.add_variable()
    }

    fn bool_add_clause(self: &mut Self, elems: &[Self::Elem]) {
        self.solver.add_clause(elems)
    }

    fn bool_find_one_model<ITER>(self: &mut Self, elems: ITER) -> Option<genvec::VectorFor<bool>>
    where
        ITER: Iterator<Item = Self::Elem>,
    {
        if self.solver.solve_with(&[]) {
            Some(elems.map(|e| self.solver.get_value(e)).collect())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::genvec::Vector as _;
    use super::*;

    #[test]
    fn boolops() {
        let mut alg = Boolean();
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
        let s = alg.bool_find_one_model([a, b].iter().cloned());
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.len(), 2);
        assert_eq!(s.get(0), true);
        assert_eq!(s.get(1), true);
    }
}
