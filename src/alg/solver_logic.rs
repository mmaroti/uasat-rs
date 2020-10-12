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
use std::sync::atomic::{AtomicUsize, Ordering};

static UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);

/// A boolean algebra backed by a SAT solver.
#[derive(Debug)]
pub struct SolverLogic {
    unique_id: usize,
    solver: Box<dyn Solver>,
    unit: Literal,
    zero: Literal,
}

impl SolverLogic {
    /// Creates a new free boolean algebra.
    pub fn new(solver_name: &str) -> Self {
        let unique_id = UNIQUE_ID.fetch_add(1, Ordering::Relaxed);
        let mut solver = create_solver(solver_name);
        let unit = solver.add_variable();
        let zero = solver.negate(unit);
        solver.add_clause(&[unit]);
        SolverLogic {
            unique_id,
            solver,
            unit,
            zero,
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.solver.get_name()
    }
}

impl PartialEq for SolverLogic {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}

impl Eq for SolverLogic {}

impl Algebra for SolverLogic {
    type Elem = Literal;

    fn size(&self) -> Option<usize> {
        None
    }

    fn element(&mut self, _index: usize) -> Self::Elem {
        unreachable!();
    }
}

impl Lattice for SolverLogic {
    fn meet(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        let not_elem0 = self.solver.negate(*elem0);
        if *elem0 == self.zero || *elem1 == self.zero || not_elem0 == *elem1 {
            self.zero
        } else if *elem0 == self.unit || *elem0 == *elem1 {
            *elem1
        } else if *elem1 == self.unit {
            *elem0
        } else {
            let not_elem1 = self.solver.negate(*elem1);
            let elem2 = self.solver.add_variable();
            let not_elem2 = self.solver.negate(elem2);
            self.solver.add_clause(&[not_elem2, *elem0]);
            self.solver.add_clause(&[not_elem2, *elem1]);
            self.solver.add_clause(&[not_elem0, not_elem1, elem2]);
            elem2
        }
    }

    fn join(&mut self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        let tmp0 = self.solver.negate(*elem0);
        let tmp1 = self.solver.negate(*elem1);
        let tmp2 = self.meet(&tmp0, &tmp1);
        self.solver.negate(tmp2)
    }
}

impl BoundedLattice for SolverLogic {
    fn zero(&mut self) -> Self::Elem {
        self.zero
    }

    fn unit(&mut self) -> Self::Elem {
        self.unit
    }
}

impl BooleanAlgebra for SolverLogic {
    fn complement(&mut self, elem: &Self::Elem) -> Self::Elem {
        self.solver.negate(*elem)
    }
}
