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
use minisat::sys::*;
use std::fmt;

/// Generic SAT solver interface
pub trait Solver {
    /// The literal type for this solver.
    type Literal: Copy + Eq;

    /// Creates a new solver.
    fn new() -> Self;

    /// Adds a pair of fresh literals to the solver.
    fn add_literal(self: &mut Self) -> Self::Literal;

    /// Negates the given literal.
    fn negate(self: &Self, lit: Self::Literal) -> Self::Literal;

    /// Adds the clause to the solver.
    fn add_clause(self: &mut Self, lits: &[Self::Literal]);

    /// Runs the solver and returns if a solution is available.
    fn solve(self: &mut Self) -> bool;

    /// Returns the value of the literal in the found model.
    fn get_value(self: &Self, lit: Self::Literal) -> bool;

    /// Returns the number of variables in the solver.
    fn num_variables(self: &Self) -> usize;

    /// Returns the number of clauses in the solver.
    fn num_clauses(self: &Self) -> usize;
}

/// MiniSAT 2.1 implementation of Solver
pub struct MiniSat {
    ptr: *mut minisat_solver_t,
}

impl MiniSat {
    #[inline]
    fn is_true(lbool: i32) -> bool {
        lbool > 0
    }
}

impl Solver for MiniSat {
    type Literal = i32;

    fn new() -> Self {
        let ptr = unsafe { minisat_new() };
        unsafe { minisat_eliminate(ptr, 1) };
        MiniSat { ptr }
    }

    fn add_literal(self: &mut Self) -> Self::Literal {
        unsafe { minisat_newLit(self.ptr) }
    }

    fn negate(self: &Self, lit: Self::Literal) -> Self::Literal {
        unsafe { minisat_negate(lit) }
    }

    fn add_clause(self: &mut Self, lits: &[Self::Literal]) {
        unsafe { minisat_addClause_begin(self.ptr) };
        for lit in lits {
            unsafe { minisat_addClause_addLit(self.ptr, *lit) };
        }
        unsafe { minisat_addClause_commit(self.ptr) };
    }

    fn solve(self: &mut Self) -> bool {
        unsafe { minisat_solve_begin(self.ptr) };
        MiniSat::is_true(unsafe { minisat_solve_commit(self.ptr) })
    }

    fn get_value(self: &Self, lit: Self::Literal) -> bool {
        MiniSat::is_true(unsafe { minisat_modelValue_Lit(self.ptr, lit) })
    }

    fn num_variables(self: &Self) -> usize {
        unsafe { minisat_num_vars(self.ptr) as usize }
    }

    fn num_clauses(self: &Self) -> usize {
        unsafe { minisat_num_clauses(self.ptr) as usize }
    }
}

impl Drop for MiniSat {
    fn drop(&mut self) {
        unsafe { minisat_delete(self.ptr) };
    }
}

impl fmt::Debug for MiniSat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MiniSat {{ variables: {}, clauses: {} }}",
            self.num_variables(),
            self.num_clauses()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minisat() {
        let mut sat = MiniSat::new();
        let a = sat.add_literal();
        let b = sat.add_literal();
        sat.add_clause(&[a, b]);
        sat.add_clause(&[sat.negate(a), b]);
        sat.add_clause(&[sat.negate(a), sat.negate(b)]);
        assert_eq!(sat.num_variables(), 2);
        assert_eq!(sat.num_clauses(), 3);
        assert!(sat.solve());
        assert!(!sat.get_value(a));
        assert!(sat.get_value(b));
        sat.add_clause(&[a, sat.negate(b)]);
        assert!(!sat.solve());
    }
}
