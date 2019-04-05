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

use super::solver::Solver;

/// A boolean algebra supporting boolean calculation.
pub trait BoolAlg {
    /// The element type of this bool algebra.
    type Elem: Copy + Eq;

    /// Returns the logical true (top) element of the algebra.
    fn one(self: &mut Self) -> Self::Elem;

    /// Returns the logical false (bottom) element of the algebra.
    fn zero(self: &mut Self) -> Self::Elem;

    /// Return the logical negation of the element.
    fn not(self: &mut Self, elem: Self::Elem) -> Self::Elem;

    /// Returns the logical or (lattice join) of a pair of elements.
    fn or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the exclusive or (boolean addition) of a pair of elements.
    fn add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the logical and (lattice meet) of a pair of elements.
    fn and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let a = self.not(elem1);
        let b = self.not(elem2);
        let c = self.or(a, b);
        self.not(c)
    }

    /// Returns the logical equivalence of a pair of elements.
    fn equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let a = self.not(elem1);
        self.add(a, elem2)
    }

    /// Returns the logical implication of a pair of elements.
    fn leq(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let a = self.not(elem1);
        self.or(a, elem2)
    }
}

/// The two element boolean algebra with `bool` elements.
#[derive(Default, Debug)]
pub struct Booleans();

impl Booleans {
    /// Creates a new two element boolean algebra.
    pub fn new() -> Self {
        Booleans()
    }
}

impl BoolAlg for Booleans {
    type Elem = bool;

    fn one(self: &mut Self) -> Self::Elem {
        true
    }

    fn zero(self: &mut Self) -> Self::Elem {
        false
    }

    fn not(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        !elem
    }

    fn or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 || elem2
    }

    fn add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 ^ elem2
    }

    fn and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 && elem2
    }

    fn equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 == elem2
    }

    fn leq(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        elem1 <= elem2
    }
}

/// The free boolean algebra backed by a SAT solver.
#[derive(Debug)]
pub struct FreeAlg<SOLVER: Solver> {
    solver: SOLVER,
    one: SOLVER::Literal,
    zero: SOLVER::Literal,
}

impl<SOLVER: Solver> FreeAlg<SOLVER> {
    /// Creates a new free boolean algebra.
    pub fn new() -> Self {
        let mut solver = SOLVER::new();
        let one = solver.add_literal();
        let zero = solver.negate(one);
        FreeAlg { solver, one, zero }
    }

    /// Adds a new free variable to the algebra
    pub fn add_variable(self: &mut Self) -> <Self as BoolAlg>::Elem {
        self.solver.add_literal()
    }

    /// Adds a new clause (disjunctive requirement) to the solver.
    pub fn add_clause(self: &mut Self, vars: &[<Self as BoolAlg>::Elem]) {
        self.solver.add_clause(vars);
    }

    /// Runs the solver and finds a model satisfying all requirements.
    pub fn solve(self: &mut Self) -> bool {
        self.solver.solve()
    }

    /// Returns the logical value of the element in the last solution.
    pub fn get_value(self: &Self, elem: <Self as BoolAlg>::Elem) -> bool {
        self.solver.get_value(elem)
    }
}

impl<SOLVER: Solver> BoolAlg for FreeAlg<SOLVER> {
    type Elem = SOLVER::Literal;

    fn one(self: &mut Self) -> Self::Elem {
        self.one
    }

    fn zero(self: &mut Self) -> Self::Elem {
        self.zero
    }

    #[inline]
    fn not(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        self.solver.negate(elem)
    }

    fn or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let not_elem1 = self.not(elem1);
        let not_elem2 = self.not(elem2);
        if elem1 == self.one || elem2 == self.one || elem1 == not_elem2 {
            self.one
        } else if elem1 == self.zero || elem1 == elem2 {
            elem2
        } else if elem2 == self.zero {
            elem1
        } else {
            let elem3 = self.solver.add_literal();
            let not_elem3 = self.not(elem3);
            self.solver.add_clause(&[not_elem1, elem3]);
            self.solver.add_clause(&[not_elem2, elem3]);
            self.solver.add_clause(&[elem1, elem2, not_elem3]);
            elem3
        }
    }

    fn add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let not_elem1 = self.not(elem1);
        let not_elem2 = self.not(elem2);
        if elem1 == self.zero {
            elem2
        } else if elem1 == self.one {
            not_elem2
        } else if elem2 == self.zero {
            elem1
        } else if elem2 == self.one {
            not_elem1
        } else if elem1 == elem2 {
            self.zero
        } else if elem1 == self.not(elem2) {
            self.one
        } else {
            let elem3 = self.solver.add_literal();
            let not_elem3 = self.not(elem3);
            self.solver.add_clause(&[not_elem1, elem2, elem3]);
            self.solver.add_clause(&[elem1, not_elem2, elem3]);
            self.solver.add_clause(&[elem1, elem2, not_elem3]);
            self.solver.add_clause(&[not_elem1, not_elem2, not_elem3]);
            elem3
        }
    }
}

impl<SOLVER: Solver> Default for FreeAlg<SOLVER> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::solver::MiniSat;
    use super::*;

    #[test]
    fn test_booleans() {
        let mut alg = Booleans::new();
        let a = alg.one();
        let b = alg.not(a);
        assert_eq!(alg.add(a, b), a);
        assert_eq!(alg.and(a, b), b);
    }

    #[test]
    fn test_freealg() {
        let mut alg: FreeAlg<MiniSat> = FreeAlg::new();
        let a = alg.add_variable();
        let b = alg.add_variable();
        let c = alg.and(a, b);
        alg.add_clause(&[c]);
        assert!(alg.solve());
        assert!(alg.get_value(a), true);
        assert!(alg.get_value(b), true);
        let an = alg.not(a);
        let bn = alg.not(b);
        alg.add_clause(&[an, bn]);
        assert!(!alg.solve());
    }
}
