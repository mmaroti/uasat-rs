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

extern crate minisat;

/**
 * A boolean algebra allowing logical calculations.
 */
pub trait BoolAlg {
    /**
     * The element type of this bool algebra.
     */
    type Elem: Copy + PartialEq + Eq;

    /**
     * The logical false value (bottom element).
     */
    const FALSE: Self::Elem;

    /**
     * The logical true value (top element).
     */
    const TRUE: Self::Elem;

    /**
     * Negates the given elem.
     */
    fn not(self: &Self, elem: Self::Elem) -> Self::Elem;

    /**
     * The logical or (lattice join) of a pair of elements.
     */
    fn or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /**
     * The exclusive or (boolean addition) of a pair of elements.
     */
    fn add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /**
     * The logical and (lattice meet) of a pair of elements.
     */
    fn and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let elem3 = self.or(self.not(elem1), self.not(elem2));
        self.not(elem3)
    }

    /**
     * The logical equivalence of a pair of elements.
     */
    fn equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        self.add(self.not(elem1), elem2)
    }

    /**
     * The logical implication of a pair of elements.
     */
    fn leq(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        self.or(self.not(elem1), elem2)
    }
}

#[derive(Default)]
pub struct Booleans();

impl Booleans {
    pub fn new() -> Self {
        Booleans()
    }
}

impl BoolAlg for Booleans {
    type Elem = bool;

    const FALSE: bool = false;
    const TRUE: bool = true;

    fn not(self: &Self, elem: bool) -> bool {
        !elem
    }

    fn or(self: &mut Self, elem1: bool, elem2: bool) -> bool {
        elem1 || elem2
    }

    fn add(self: &mut Self, elem1: bool, elem2: bool) -> bool {
        elem1 ^ elem2
    }

    fn and(self: &mut Self, elem1: bool, elem2: bool) -> bool {
        elem1 && elem2
    }

    fn equ(self: &mut Self, elem1: bool, elem2: bool) -> bool {
        elem1 == elem2
    }
}

/**
 * A free boolean algebra with an associated SAT solver.
 */
pub trait Solver<'a>: BoolAlg {
    /**
     * Creates a new solver.
     */
    fn new() -> Self;

    /**
     * Adds a new literal to the solver.
     */
    fn add_literal(self: &mut Self) -> Self::Elem;

    /**
     * Adds the given clause to the solver.
     */
    fn add_clause(self: &mut Self, lits: Vec<Self::Elem>);

    /**
     * Adds a clause with a pair of literals.
     */
    fn add_clause2(self: &mut Self, lit1: Self::Elem, lit2: Self::Elem) {
        self.add_clause(vec![lit1, lit2]);
    }

    /**
     * Adds a clause with three literals.
     */
    fn add_clause3(self: &mut Self, lit1: Self::Elem, lit2: Self::Elem, lit3: Self::Elem) {
        self.add_clause(vec![lit1, lit2, lit3]);
    }

    /**
     * Runs the solver, returns true if there is a solution.
     */
    fn solve(self: &'a mut Self) -> bool;
}

pub struct MiniSat<'a> {
    solver: minisat::Solver,
    result: Option<minisat::Model<'a>>,
}

impl<'a> BoolAlg for MiniSat<'a> {
    type Elem = minisat::Bool;

    const FALSE: Self::Elem = minisat::Bool::Const(false);

    const TRUE: Self::Elem = minisat::Bool::Const(true);

    fn not(self: &Self, elem: Self::Elem) -> Self::Elem {
        !elem
    }

    fn or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        if elem1 == Self::TRUE || elem2 == Self::TRUE || elem1 == self.not(elem2) {
            Self::TRUE
        } else if elem1 == Self::FALSE || elem1 == elem2 {
            elem2
        } else if elem2 == Self::FALSE {
            elem1
        } else {
            let elem3 = self.add_literal();
            self.add_clause2(self.not(elem1), elem3);
            self.add_clause2(self.not(elem2), elem3);
            self.add_clause3(elem1, elem2, self.not(elem3));
            elem3
        }
    }

    fn add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        if elem1 == Self::FALSE {
            elem2
        } else if elem1 == Self::TRUE {
            self.not(elem2)
        } else if elem2 == Self::FALSE {
            elem1
        } else if elem2 == Self::TRUE {
            self.not(elem1)
        } else if elem1 == elem2 {
            Self::FALSE
        } else if elem1 == self.not(elem2) {
            Self::TRUE
        } else {
            let elem3 = self.add_literal();
            self.add_clause3(self.not(elem1), elem2, elem3);
            self.add_clause3(elem1, self.not(elem2), elem3);
            self.add_clause3(elem1, elem2, self.not(elem3));
            self.add_clause3(self.not(elem1), self.not(elem2), self.not(elem3));
            elem3
        }
    }
}

impl<'a> Solver<'a> for MiniSat<'a> {
    fn new() -> Self {
        MiniSat {
            solver: minisat::Solver::new(),
            result: None,
        }
    }

    fn add_literal(self: &mut Self) -> Self::Elem {
        self.solver.new_lit()
    }

    fn add_clause(self: &mut Self, lits: Vec<Self::Elem>) {
        self.solver.add_clause(lits);
    }

    fn solve(self: &'a mut Self) -> bool {
        match self.solver.solve() {
            Ok(result) => {
                self.result = Some(result);
                true
            }
            Err(()) => {
                self.result = None;
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_booleans() {
        let mut alg = Booleans::new();
        let a = Booleans::TRUE;
        let b = alg.not(a);
        assert_eq!(alg.add(a, b), a);
        assert_eq!(alg.and(a, b), b);
    }

    #[test]
    fn test_solver1() {
        let mut sat = MiniSat::new();
        let a = sat.add_literal();
        let b = sat.add_literal();
        sat.add_clause2(a, b);
        sat.add_clause2(sat.not(a), b);
        sat.add_clause2(a, sat.not(b));
        assert!(sat.solve());
        // let na = sat.not(a);
    }
}
