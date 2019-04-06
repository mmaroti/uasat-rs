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

#[cfg(feature = "minisat")]
extern crate minisat;
extern crate varisat;

use std::{fmt, iter};

/// A boolean algebra supporting boolean calculation.
pub trait BoolAlg {
    /// The element type of this bool algebra.
    type Elem: Copy + Eq;

    /// Returns the logical true (top) element of the algebra.
    fn unit(self: &mut Self) -> Self::Elem;

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
pub struct Boolean();

impl Boolean {
    /// Creates a new two element boolean algebra.
    pub fn new() -> Self {
        Boolean()
    }
}

impl BoolAlg for Boolean {
    type Elem = bool;

    fn unit(self: &mut Self) -> Self::Elem {
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
pub struct FreeAlg<SOL: Solver> {
    solver: SOL,
    one: SOL::Literal,
    zero: SOL::Literal,
}

impl<SOL: Solver> FreeAlg<SOL> {
    /// Creates a new free boolean algebra.
    pub fn new() -> Self {
        let mut solver = SOL::new();
        let one = solver.add_variable();
        let zero = solver.negate(one);
        FreeAlg { solver, one, zero }
    }

    /// Adds a new free variable to the algebra
    pub fn add_variable(self: &mut Self) -> <Self as BoolAlg>::Elem {
        self.solver.add_variable()
    }

    /// Runs the solver and finds a model where the given assumptions are true.
    pub fn find_model<'a, VARS>(self: &'a mut Self, vars: VARS) -> bool
    where
        VARS: IntoIterator<Item = &'a <Self as BoolAlg>::Elem>,
    {
        self.solver.solve_with(vars)
    }

    /// Returns the logical value of the element in the found model.
    pub fn get_value(self: &Self, elem: <Self as BoolAlg>::Elem) -> bool {
        self.solver.get_value(elem)
    }
}

impl<SOL: Solver> BoolAlg for FreeAlg<SOL> {
    type Elem = SOL::Literal;

    fn unit(self: &mut Self) -> Self::Elem {
        self.one
    }

    fn zero(self: &mut Self) -> Self::Elem {
        self.zero
    }

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
            let elem3 = self.solver.add_variable();
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
            let elem3 = self.solver.add_variable();
            let not_elem3 = self.not(elem3);
            self.solver.add_clause(&[not_elem1, elem2, elem3]);
            self.solver.add_clause(&[elem1, not_elem2, elem3]);
            self.solver.add_clause(&[elem1, elem2, not_elem3]);
            self.solver.add_clause(&[not_elem1, not_elem2, not_elem3]);
            elem3
        }
    }
}

impl<SOL: Solver> Default for FreeAlg<SOL> {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic SAT solver interface
pub trait Solver {
    /// The literal type for this solver.
    type Literal: Copy + Eq;

    /// Creates a new solver.
    fn new() -> Self;

    /// Adds a fresh variable to the solver.
    fn add_variable(self: &mut Self) -> Self::Literal;

    /// Negates the given literal.
    fn negate(self: &Self, lit: Self::Literal) -> Self::Literal;

    /// Adds the clause to the solver.
    fn add_clause<'a, LITS>(self: &'a mut Self, lits: LITS)
    where
        LITS: IntoIterator<Item = &'a Self::Literal>;

    /// Runs the solver and returns if a solution is available.
    fn solve(self: &mut Self) -> bool {
        self.solve_with(iter::empty())
    }

    /// Runs the solver with the given assumptions and finds a model
    /// satisfying all requirements.
    fn solve_with<'a, LITS>(self: &'a mut Self, lits: LITS) -> bool
    where
        LITS: IntoIterator<Item = &'a Self::Literal>;

    /// Returns the value of the literal in the found model.
    fn get_value(self: &Self, lit: Self::Literal) -> bool;

    /// Returns the name of the solver
    fn get_name(self: &Self) -> &'static str;

    /// Returns the number of variables in the solver.
    fn num_variables(self: &Self) -> u32;

    /// Returns the number of clauses in the solver.
    fn num_clauses(self: &Self) -> u32;
}

/// MiniSAT 2.1 external C library based SAT solver
#[cfg(feature = "minisat")]
pub struct MiniSat {
    ptr: *mut minisat::sys::minisat_solver_t,
}

#[cfg(feature = "minisat")]
impl MiniSat {
    fn is_true(lbool: i32) -> bool {
        lbool > 0
    }
}

#[cfg(feature = "minisat")]
impl Solver for MiniSat {
    type Literal = i32;

    fn new() -> Self {
        let ptr = unsafe { minisat::sys::minisat_new() };
        unsafe { minisat::sys::minisat_eliminate(ptr, 1) };
        MiniSat { ptr }
    }

    fn add_variable(self: &mut Self) -> Self::Literal {
        unsafe { minisat::sys::minisat_newLit(self.ptr) }
    }

    fn negate(self: &Self, lit: Self::Literal) -> Self::Literal {
        unsafe { minisat::sys::minisat_negate(lit) }
    }

    fn add_clause<'a, LITS>(self: &'a mut Self, lits: LITS)
    where
        LITS: IntoIterator<Item = &'a Self::Literal>,
    {
        unsafe { minisat::sys::minisat_addClause_begin(self.ptr) };
        for lit in lits {
            unsafe { minisat::sys::minisat_addClause_addLit(self.ptr, *lit) };
        }
        unsafe { minisat::sys::minisat_addClause_commit(self.ptr) };
    }

    fn solve_with<'a, LITS>(self: &'a mut Self, lits: LITS) -> bool
    where
        LITS: IntoIterator<Item = &'a Self::Literal>,
    {
        unsafe { minisat::sys::minisat_solve_begin(self.ptr) };
        for lit in lits {
            unsafe { minisat::sys::minisat_solve_addLit(self.ptr, *lit) };
        }
        MiniSat::is_true(unsafe { minisat::sys::minisat_solve_commit(self.ptr) })
    }

    fn get_value(self: &Self, lit: Self::Literal) -> bool {
        MiniSat::is_true(unsafe { minisat::sys::minisat_modelValue_Lit(self.ptr, lit) })
    }

    fn get_name(self: &Self) -> &'static str {
        "MiniSat"
    }

    fn num_variables(self: &Self) -> u32 {
        unsafe { minisat::sys::minisat_num_vars(self.ptr) as u32 }
    }

    fn num_clauses(self: &Self) -> u32 {
        unsafe { minisat::sys::minisat_num_clauses(self.ptr) as u32 }
    }
}

#[cfg(feature = "minisat")]
impl Drop for MiniSat {
    fn drop(&mut self) {
        unsafe { minisat::sys::minisat_delete(self.ptr) };
    }
}

#[cfg(feature = "minisat")]
impl std::fmt::Debug for MiniSat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MiniSat {{ variables: {}, clauses: {} }}",
            self.num_variables(),
            self.num_clauses()
        )
    }
}

/// A modern SAT solver implemented in pure rust.
pub struct VariSat {
    num_variables: u32,
    num_clauses: u32,
    solver: varisat::solver::Solver,
    solution: Vec<bool>,
}

impl Solver for VariSat {
    type Literal = varisat::lit::Lit;

    fn new() -> Self {
        VariSat {
            num_variables: 0,
            num_clauses: 0,
            solver: varisat::solver::Solver::new(),
            solution: Vec::new(),
        }
    }

    fn add_variable(self: &mut Self) -> Self::Literal {
        let var = varisat::lit::Var::from_index(self.num_variables as usize);
        self.num_variables += 1;
        varisat::lit::Lit::from_var(var, false)
    }

    fn negate(self: &Self, lit: Self::Literal) -> Self::Literal {
        !lit
    }

    fn add_clause<'a, LITS>(self: &'a mut Self, lits: LITS)
    where
        LITS: IntoIterator<Item = &'a Self::Literal>,
    {
        let mut formula = varisat::cnf::CnfFormula::new();
        formula.add_clause(lits);
        self.solver.add_formula(&formula);
        self.num_clauses += 1;
    }

    fn solve_with<'a, LITS>(self: &'a mut Self, lits: LITS) -> bool
    where
        LITS: IntoIterator<Item = &'a Self::Literal>,
    {
        let mut assumptions = Vec::new();
        assumptions.extend(lits);
        self.solver.assume(&assumptions);

        self.solution.clear();
        let solvable = self.solver.solve().unwrap();
        if solvable {
            for lit in self.solver.model().unwrap() {
                let var = lit.var().index();
                if self.solution.len() <= var {
                    self.solution.resize(var + 1, false);
                }
                self.solution[var] = lit.is_positive();
            }
        }
        solvable
    }

    fn get_value(self: &Self, lit: Self::Literal) -> bool {
        let var = lit.var().index();
        assert!(var < self.solution.len());
        self.solution[var] ^ lit.is_negative()
    }

    fn get_name(self: &Self) -> &'static str {
        "VariSat"
    }

    fn num_variables(self: &Self) -> u32 {
        self.num_variables
    }

    fn num_clauses(self: &Self) -> u32 {
        self.num_clauses
    }
}

impl fmt::Debug for VariSat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VariSat {{ variables: {}, clauses: {} }}",
            self.num_variables(),
            self.num_clauses()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        let mut alg = Boolean::new();
        let a = alg.unit();
        let b = alg.not(a);
        assert_eq!(alg.add(a, b), a);
        assert_eq!(alg.and(a, b), b);
    }

    #[test]
    fn test_freealg() {
        let mut alg: FreeAlg<VariSat> = FreeAlg::new();
        let a = alg.add_variable();
        let b = alg.add_variable();
        let c = alg.and(a, b);
        assert!(alg.find_model(&[c]));
        assert!(alg.get_value(a), true);
        assert!(alg.get_value(b), true);
        let d = alg.not(a);
        assert!(!alg.find_model(&[c, d]));
    }

    #[cfg(feature = "minisat")]
    #[test]
    fn test_minisat() {
        let mut sat = MiniSat::new();
        let a = sat.add_variable();
        let b = sat.add_variable();
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

    #[test]
    fn test_varisat() {
        let mut sat = VariSat::new();
        let a = sat.add_variable();
        let b = sat.add_variable();
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
