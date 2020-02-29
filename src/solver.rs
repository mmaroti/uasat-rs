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

//! A generic trait to work with various SAT solvers.

#[cfg(feature = "varisat")]
extern crate bit_vec;
#[cfg(feature = "cryptominisat")]
extern crate cryptominisat;
#[cfg(feature = "minisat")]
extern crate minisat;
#[cfg(feature = "varisat")]
extern crate varisat;

#[cfg(feature = "varisat")]
use bit_vec::BitVec;
use std::fmt;

#[cfg(feature = "varisat")]
use varisat::ExtendFormula;

/// Uniform literal to allow runtime solver selection.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Literal {
    pub value: u32,
}

/// Generic SAT solver interface
pub trait Solver {
    /// Adds a fresh variable to the solver.
    fn add_variable(self: &mut Self) -> Literal;

    /// Negates the given literal.
    fn negate(self: &Self, lit: Literal) -> Literal;

    /// Adds the clause to the solver.
    fn add_clause(self: &mut Self, lits: &[Literal]);

    /// Runs the solver and returns true if a solution is available.
    fn solve(self: &mut Self) -> bool {
        self.solve_with(&[])
    }

    /// Runs the solver with the given assumptions and finds a model satisfying
    /// all requirements. Returns false is no solution was found.
    fn solve_with(self: &mut Self, lits: &[Literal]) -> bool;

    /// Returns the value of the literal in the found model.
    fn get_value(self: &Self, lit: Literal) -> bool;

    /// Returns the name of the solver
    fn get_name(self: &Self) -> &'static str;

    /// Returns the number of variables in the solver.
    fn num_variables(self: &Self) -> u32;

    /// Returns the number of clauses in the solver.
    fn num_clauses(self: &Self) -> u32;
}

/// Tries to create a SAT solver with the given name. Currently "varisat",
/// "minisat" and "cryptominisat" (not on wasm) are supported. Use the empty
/// string to match the first available solver.
pub fn create_solver(name: &str) -> Box<dyn Solver> {
    let mut enabled_solvers = 0;

    #[cfg(feature = "varisat")]
    {
        enabled_solvers += 1;
        if name == "varisat" || name == "" {
            let sat: VariSat = Default::default();
            return Box::new(sat);
        }
    }

    #[cfg(feature = "minisat")]
    {
        enabled_solvers += 1;
        if name == "minisat" || name == "" {
            let sat: MiniSat = Default::default();
            return Box::new(sat);
        }
    }

    #[cfg(feature = "cryptominisat")]
    {
        enabled_solvers += 1;
        if name == "cryptominisat" || name == "" {
            let sat: CryptoMiniSat = Default::default();
            return Box::new(sat);
        }
    }

    if enabled_solvers == 0 {
        panic!("No SAT solvers are available.")
    } else {
        panic!("Unknown SAT solver: {}", name);
    }
}

impl fmt::Debug for dyn Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {{ variables: {}, clauses: {} }}",
            self.get_name(),
            self.num_variables(),
            self.num_clauses()
        )
    }
}

/// MiniSAT 2.1 external C library based SAT solver
#[cfg(feature = "minisat")]
pub struct MiniSat {
    ptr: *mut minisat::sys::minisat_solver_t,
}

#[cfg(feature = "minisat")]
impl Default for MiniSat {
    /// Creates a new solver instance.
    fn default() -> Self {
        let ptr = unsafe { minisat::sys::minisat_new() };
        unsafe { minisat::sys::minisat_eliminate(ptr, 1) };
        MiniSat { ptr }
    }
}

#[cfg(feature = "minisat")]
impl MiniSat {
    fn is_true(lbool: i32) -> bool {
        lbool > 0
    }

    fn encode(value: i32) -> Literal {
        Literal {
            value: value as u32,
        }
    }

    fn decode(lit: Literal) -> i32 {
        lit.value as i32
    }
}

#[cfg(feature = "minisat")]
impl Solver for MiniSat {
    fn add_variable(self: &mut Self) -> Literal {
        MiniSat::encode(unsafe { minisat::sys::minisat_newLit(self.ptr) })
    }

    fn negate(self: &Self, lit: Literal) -> Literal {
        MiniSat::encode(unsafe { minisat::sys::minisat_negate(MiniSat::decode(lit)) })
    }

    fn add_clause(self: &mut Self, lits: &[Literal]) {
        unsafe { minisat::sys::minisat_addClause_begin(self.ptr) };
        for lit in lits {
            unsafe { minisat::sys::minisat_addClause_addLit(self.ptr, MiniSat::decode(*lit)) };
        }
        unsafe { minisat::sys::minisat_addClause_commit(self.ptr) };
    }

    fn solve_with(self: &mut Self, lits: &[Literal]) -> bool {
        unsafe { minisat::sys::minisat_solve_begin(self.ptr) };
        for lit in lits {
            unsafe { minisat::sys::minisat_solve_addLit(self.ptr, MiniSat::decode(*lit)) };
        }
        MiniSat::is_true(unsafe { minisat::sys::minisat_solve_commit(self.ptr) })
    }

    fn get_value(self: &Self, lit: Literal) -> bool {
        MiniSat::is_true(unsafe {
            minisat::sys::minisat_modelValue_Lit(self.ptr, MiniSat::decode(lit))
        })
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

/// A modern SAT solver implemented in pure rust.
#[cfg(feature = "varisat")]
pub struct VariSat<'a> {
    num_variables: u32,
    num_clauses: u32,
    solver: varisat::solver::Solver<'a>,
    solution: BitVec,
}

#[cfg(feature = "varisat")]
impl<'a> Default for VariSat<'a> {
    /// Creates a new solver instance.
    fn default() -> Self {
        VariSat {
            num_variables: 0,
            num_clauses: 0,
            solver: varisat::solver::Solver::new(),
            solution: BitVec::new(),
        }
    }
}

#[cfg(feature = "varisat")]
impl<'a> VariSat<'a> {
    fn encode(lit: varisat::lit::Lit) -> Literal {
        Literal {
            value: lit.code() as u32,
        }
    }

    fn decode(lit: Literal) -> varisat::lit::Lit {
        varisat::lit::Lit::from_code(lit.value as usize)
    }
}

#[cfg(feature = "varisat")]
impl<'a> Solver for VariSat<'a> {
    fn add_variable(self: &mut Self) -> Literal {
        let var = varisat::lit::Var::from_index(self.num_variables as usize);
        self.num_variables += 1;
        VariSat::encode(varisat::lit::Lit::from_var(var, false))
    }

    fn negate(self: &Self, lit: Literal) -> Literal {
        VariSat::encode(!VariSat::decode(lit))
    }

    fn add_clause(self: &mut Self, lits: &[Literal]) {
        let mut formula = varisat::cnf::CnfFormula::new();
        // TODO: do we need to allocate?
        let lits: Vec<varisat::Lit> = lits.iter().map(|lit| VariSat::decode(*lit)).collect();
        formula.add_clause(&lits);
        self.solver.add_formula(&formula);
        self.num_clauses += 1;
    }

    fn solve_with(self: &mut Self, lits: &[Literal]) -> bool {
        let mut assumptions = Vec::new();
        assumptions.extend(lits.iter().map(|lit| VariSat::decode(*lit)));
        self.solver.assume(&assumptions);

        self.solution.clear();
        let solvable = self.solver.solve().unwrap();
        if solvable {
            self.solution.grow(self.num_variables() as usize, false);
            for lit in self.solver.model().unwrap() {
                let var = lit.index();
                self.solution.set(var, lit.is_positive());
            }
        }
        solvable
    }

    fn get_value(self: &Self, lit: Literal) -> bool {
        let lit = VariSat::decode(lit);
        let var = lit.index();
        self.solution.get(var).unwrap() ^ lit.is_negative()
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

/// An advanced SAT solver supporting XOR clauses.
#[cfg(feature = "cryptominisat")]
pub struct CryptoMiniSat {
    solver: cryptominisat::Solver,
    num_clauses: u32,
}

#[cfg(feature = "cryptominisat")]
impl Default for CryptoMiniSat {
    /// Creates a new solver instance.
    fn default() -> Self {
        CryptoMiniSat {
            solver: cryptominisat::Solver::new(),
            num_clauses: 0,
        }
    }
}

#[cfg(feature = "cryptominisat")]
impl CryptoMiniSat {
    fn encode(lit: cryptominisat::Lit) -> Literal {
        Literal {
            value: (lit.var() << 1) | (lit.isneg() as u32),
        }
    }

    fn decode(lit: Literal) -> cryptominisat::Lit {
        cryptominisat::Lit::new(lit.value >> 1, (lit.value & 1) != 0).unwrap()
    }
}

#[cfg(feature = "cryptominisat")]
impl Solver for CryptoMiniSat {
    fn add_variable(self: &mut Self) -> Literal {
        CryptoMiniSat::encode(self.solver.new_var())
    }

    fn negate(self: &Self, lit: Literal) -> Literal {
        Literal {
            value: lit.value ^ 1,
        }
    }

    fn add_clause(self: &mut Self, lits: &[Literal]) {
        let lits: Vec<cryptominisat::Lit> =
            lits.iter().map(|lit| CryptoMiniSat::decode(*lit)).collect();
        self.solver.add_clause(&lits);
        self.num_clauses += 1;
    }

    fn solve_with(self: &mut Self, lits: &[Literal]) -> bool {
        let lits: Vec<cryptominisat::Lit> =
            lits.iter().map(|lit| CryptoMiniSat::decode(*lit)).collect();
        self.solver.solve_with_assumptions(&lits) == cryptominisat::Lbool::True
    }

    fn get_value(self: &Self, lit: Literal) -> bool {
        self.solver.is_true(CryptoMiniSat::decode(lit))
    }

    fn get_name(self: &Self) -> &'static str {
        "CryptoMiniSat"
    }

    fn num_variables(self: &Self) -> u32 {
        self.solver.nvars()
    }

    fn num_clauses(self: &Self) -> u32 {
        self.num_clauses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "minisat")]
    #[test]
    fn minisat() {
        let mut sat: MiniSat = Default::default();
        let a = sat.add_variable();
        let b = sat.add_variable();
        sat.add_clause(&[a, b]);
        assert!(sat.solve_with(&[sat.negate(b)]));
        assert!(sat.get_value(a));
        assert!(!sat.get_value(b));
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

    #[cfg(feature = "varisat")]
    #[test]
    fn varisat() {
        let mut sat: VariSat = Default::default();
        let a = sat.add_variable();
        let b = sat.add_variable();
        sat.add_clause(&[a, b]);
        assert!(sat.solve_with(&[sat.negate(b)]));
        assert!(sat.get_value(a));
        assert!(!sat.get_value(b));
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

    #[cfg(feature = "cryptominisat")]
    #[test]
    fn cryptominisat() {
        let mut sat: CryptoMiniSat = Default::default();
        let a = sat.add_variable();
        let b = sat.add_variable();
        sat.add_clause(&[a, b]);
        assert!(sat.solve_with(&[sat.negate(b)]));
        assert!(sat.get_value(a));
        assert!(!sat.get_value(b));
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
