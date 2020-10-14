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

use super::{Algebra, BooleanAlgebra, BoundedLattice, Lattice};
use crate::solver::{create_solver, Literal, Solver};
use std::cell::Cell;

/// A boolean algebra backed by a SAT solver.
pub struct SolverLogic {
    solver: Cell<Option<Box<dyn Solver>>>,
    unit: Literal,
    zero: Literal,
}

impl SolverLogic {
    /// Creates a new free boolean algebra.
    pub fn new(solver_name: &str) -> Self {
        let mut solver = create_solver(solver_name);
        let unit = solver.add_variable();
        let zero = solver.negate(unit);
        solver.add_clause(&[unit]);
        let solver = Cell::new(Some(solver));
        SolverLogic { solver, unit, zero }
    }

    /// Takes the solver out of its cell, performs the given operation with the solver and then
    /// returns the solver back into its cell. This method will panic if it is called recursively.
    fn mutate<F, R>(&self, fun: F) -> R
    where
        F: FnOnce(&mut Box<dyn Solver>) -> R,
    {
        let mut solver = self.solver.replace(None).expect("recursion error");
        let value = fun(&mut solver);
        self.solver.set(Some(solver));
        value
    }

    /// Returns the name of the solver.
    pub fn get_name(&self) -> &'static str {
        self.mutate(|solver| solver.get_name())
    }
}

impl Algebra for SolverLogic {
    type Elem = Literal;
}

impl Lattice for SolverLogic {
    fn meet(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.mutate(|solver| {
            let not_elem0 = solver.negate(*elem0);
            if *elem0 == self.zero || *elem1 == self.zero || not_elem0 == *elem1 {
                self.zero
            } else if *elem0 == self.unit || *elem0 == *elem1 {
                *elem1
            } else if *elem1 == self.unit {
                *elem0
            } else {
                let not_elem1 = solver.negate(*elem1);
                let elem2 = solver.add_variable();
                let not_elem2 = solver.negate(elem2);
                solver.add_clause(&[not_elem2, *elem0]);
                solver.add_clause(&[not_elem2, *elem1]);
                solver.add_clause(&[not_elem0, not_elem1, elem2]);
                elem2
            }
        })
    }

    fn join(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.mutate(|solver| {
            let not_elem0 = solver.negate(*elem0);
            if *elem0 == self.unit || *elem1 == self.unit || not_elem0 == *elem1 {
                self.unit
            } else if *elem0 == self.zero || *elem0 == *elem1 {
                *elem1
            } else if *elem1 == self.zero {
                *elem0
            } else {
                let not_elem1 = solver.negate(*elem1);
                let elem2 = solver.add_variable();
                let not_elem2 = solver.negate(elem2);
                solver.add_clause(&[not_elem0, elem2]);
                solver.add_clause(&[not_elem1, elem2]);
                solver.add_clause(&[*elem0, *elem1, not_elem2]);
                elem2
            }
        })
    }
}

impl BoundedLattice for SolverLogic {
    fn bot(&self) -> Self::Elem {
        self.zero
    }

    fn top(&self) -> Self::Elem {
        self.unit
    }
}

impl BooleanAlgebra for SolverLogic {
    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        self.mutate(|solver| solver.negate(*elem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algebra() {
        let log = SolverLogic::new("");
        let a = log.top();
        let b = log.bot();
        let _c = log.meet(&a, &b);
        let _d = log.join(&a, &b);
    }
}
