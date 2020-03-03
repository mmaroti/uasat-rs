/*
* Copyright (C) 2019, Miklos Maroti
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

use super::solver;

/// A boolean algebra supporting boolean calculation.
pub trait BoolAlg {
    /// The element type of this bool algebra.
    type Elem: Copy;

    /// Returns the logical true (top) element of the algebra.
    fn bool_unit(self: &Self) -> Self::Elem;

    /// Returns the logical false (bottom) element of the algebra.
    fn bool_zero(self: &Self) -> Self::Elem;

    /// Returns either the unit or zero element depending of the argument.
    fn bool_lift(self: &Self, elem: bool) -> Self::Elem {
        if elem {
            self.bool_unit()
        } else {
            self.bool_zero()
        }
    }

    /// Return the logical negation of the element.
    fn bool_not(self: &mut Self, elem: Self::Elem) -> Self::Elem;

    /// Returns the logical or (lattice join) of a pair of elements.
    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the exclusive or (boolean addition) of a pair of elements.
    fn bool_add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

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
        self.bool_add(tmp, elem2)
    }

    /// Returns the logical implication of a pair of elements.
    fn bool_leq(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp = self.bool_not(elem1);
        self.bool_or(tmp, elem2)
    }

    /// Computes the conjunction of the elements.
    fn bool_all(self: &mut Self, elems: &[Self::Elem]) -> Self::Elem {
        let mut result = self.bool_unit();
        for elem in elems {
            result = self.bool_and(result, *elem);
        }
        result
    }

    /// Computes the disjunction of the elements.
    fn bool_any(self: &mut Self, elems: &[Self::Elem]) -> Self::Elem {
        let mut result = self.bool_zero();
        for elem in elems {
            result = self.bool_or(result, *elem);
        }
        result
    }
}

/// The two element boolean algebra with `bool` elements.
#[derive(Default, Debug)]
pub struct Boolean();

impl Boolean {
    /// Creates a new two element boolean algebra.
    pub fn new() -> Self {
        Boolean()
    }
}

impl BoolAlg for Boolean {
    type Elem = bool;

    fn bool_unit(self: &Self) -> Self::Elem {
        true
    }

    fn bool_zero(self: &Self) -> Self::Elem {
        false
    }

    fn bool_lift(self: &Self, elem: bool) -> Self::Elem {
        elem
    }

    fn bool_not(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        !elem
    }

    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 || elem2
    }

    fn bool_add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 ^ elem2
    }

    fn bool_and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 && elem2
    }

    fn bool_equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 == elem2
    }

    fn bool_leq(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
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

    fn bool_unit(self: &Self) -> Self::Elem {
        self.unit
    }

    fn bool_zero(self: &Self) -> Self::Elem {
        self.zero
    }

    fn bool_not(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        self.solver.negate(elem)
    }

    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let not_elem1 = self.solver.negate(elem1);
        let not_elem2 = self.solver.negate(elem2);
        if elem1 == self.unit || elem2 == self.unit || elem1 == not_elem2 {
            self.unit
        } else if elem1 == self.zero || elem1 == elem2 {
            elem2
        } else if elem2 == self.zero {
            elem1
        } else {
            let elem3 = self.solver.add_variable();
            let not_elem3 = self.solver.negate(elem3);
            self.solver.add_clause(&[not_elem1, elem3]);
            self.solver.add_clause(&[not_elem2, elem3]);
            self.solver.add_clause(&[elem1, elem2, not_elem3]);
            elem3
        }
    }

    fn bool_add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let not_elem1 = self.solver.negate(elem1);
        let not_elem2 = self.solver.negate(elem2);
        if elem1 == self.zero {
            elem2
        } else if elem1 == self.unit {
            not_elem2
        } else if elem2 == self.zero {
            elem1
        } else if elem2 == self.unit {
            not_elem1
        } else if elem1 == elem2 {
            self.zero
        } else if elem1 == not_elem2 {
            self.unit
        } else {
            let elem3 = self.solver.add_variable();
            let not_elem3 = self.solver.negate(elem3);
            self.solver.add_clause(&[not_elem1, elem2, elem3]);
            self.solver.add_clause(&[elem1, not_elem2, elem3]);
            self.solver.add_clause(&[elem1, elem2, not_elem3]);
            self.solver.add_clause(&[not_elem1, not_elem2, not_elem3]);
            elem3
        }
    }
}

/// Constraint solving over a boolean algebra.
pub trait BoolSat: BoolAlg {
    /// Adds a new variable to the solver
    fn add_variable(self: &mut Self) -> Self::Elem;

    /// Adds the given (disjunctive) clause to the solver.
    fn add_clause(self: &mut Self, elems: &[Self::Elem]);

    /// Runs the solver and finds a model where the given assumptions are true.
    fn find_model(self: &mut Self, elems: &[Self::Elem]) -> bool;

    /// Returns the logical value of the element in the found model.
    fn get_value(self: &Self, elem: solver::Literal) -> bool;
}

impl BoolSat for Solver {
    fn add_variable(self: &mut Self) -> Self::Elem {
        self.solver.add_variable()
    }

    fn add_clause(self: &mut Self, elems: &[Self::Elem]) {
        self.solver.add_clause(&elems)
    }

    fn find_model(self: &mut Self, elems: &[Self::Elem]) -> bool {
        self.solver.solve_with(elems)
    }

    fn get_value(self: &Self, elem: solver::Literal) -> bool {
        self.solver.get_value(elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolops() {
        let mut alg = Boolean::new();
        let a = alg.bool_unit();
        let b = alg.bool_not(a);
        assert_eq!(alg.bool_add(a, b), a);
        assert_eq!(alg.bool_and(a, b), b);
    }

    #[test]
    fn freealg() {
        let mut alg = Solver::new("");
        let a = alg.add_variable();
        let b = alg.add_variable();
        let c = alg.bool_and(a, b);
        assert!(alg.find_model(&[c]));
        assert!(alg.get_value(a), true);
        assert!(alg.get_value(b), true);
        let d = alg.bool_not(a);
        assert!(!alg.find_model(&[c, d]));
    }
}
