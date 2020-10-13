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

//! A generic trait to work with various SAT solvers.

#[cfg(feature = "batsat")]
use batsat::intmap::AsIndex as _;
#[cfg(feature = "batsat")]
use batsat::SolverInterface as _;
#[cfg(feature = "varisat")]
use varisat::ExtendFormula as _;

/// Uniform literal to allow runtime solver selection.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Literal {
    pub value: u32,
}

/// Generic SAT solver interface
pub trait Solver {
    /// Adds a fresh variable to the solver.
    fn add_variable(&mut self) -> Literal;

    /// Negates the given literal.
    fn negate(&self, lit: Literal) -> Literal;

    /// Adds the clause to the solver.
    fn add_clause(&mut self, lits: &[Literal]);

    /// Adds an XOR clause to the solver where the binary sum of the literals
    /// must be zero.
    fn add_xor_clause(&mut self, lit1: Literal, lit2: Literal, lit3: Literal) {
        let not_lit1 = self.negate(lit1);
        let not_lit2 = self.negate(lit2);
        let not_lit3 = self.negate(lit3);
        self.add_clause(&[not_lit1, lit2, lit3]);
        self.add_clause(&[lit1, not_lit2, lit3]);
        self.add_clause(&[lit1, lit2, not_lit3]);
        self.add_clause(&[not_lit1, not_lit2, not_lit3]);
    }

    /// Runs the solver and returns true if a solution is available.
    fn solve(&mut self) -> bool {
        self.solve_with(&[])
    }

    /// Runs the solver with the given assumptions and finds a model satisfying
    /// all requirements. Returns false is no solution was found.
    fn solve_with(&mut self, lits: &[Literal]) -> bool;

    /// Returns the value of the literal in the found model.
    fn get_value(&self, lit: Literal) -> bool;

    /// Returns the name of the solver
    fn get_name(&self) -> &'static str;

    /// Returns the number of variables in the solver.
    fn num_variables(&self) -> u32;

    /// Returns the number of clauses in the solver.
    fn num_clauses(&self) -> usize;
}

/// Tries to create a SAT solver with the given name. Currently "batsat",
/// "varisat", "minisat" and "cryptominisat" are supported, but not on all
/// platforms. Use the empty string to match the first available solver.
pub fn create_solver(name: &str) -> Box<dyn Solver> {
    #[cfg(feature = "cadical")]
    {
        if name == "cadical" || name == "" {
            let sat: CaDiCaL = Default::default();
            return Box::new(sat);
        } else if name == "cadical-sat" {
            let sat: CaDiCaL = CaDiCaL::with_config("sat");
            return Box::new(sat);
        } else if name == "cadical-unsat" {
            let sat: CaDiCaL = CaDiCaL::with_config("unsat");
            return Box::new(sat);
        } else if name == "cadical-plain" {
            let sat: CaDiCaL = CaDiCaL::with_config("plain");
            return Box::new(sat);
        }
    }

    #[cfg(feature = "batsat")]
    {
        if name == "batsat" || name == "" {
            let sat: BatSat = Default::default();
            return Box::new(sat);
        }
    }

    #[cfg(feature = "minisat")]
    {
        if name == "minisat" || name == "" {
            let sat: MiniSat = Default::default();
            return Box::new(sat);
        }
    }

    #[cfg(feature = "cryptominisat")]
    {
        if name == "cryptominisat" || name == "" {
            let sat: CryptoMiniSat = Default::default();
            return Box::new(sat);
        }
    }

    #[cfg(feature = "varisat")]
    {
        if name == "varisat" || name == "" {
            let sat: VariSat = Default::default();
            return Box::new(sat);
        }
    }

    panic!("Unknown SAT solver: {}", name);
}

impl std::fmt::Debug for dyn Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    fn add_variable(&mut self) -> Literal {
        MiniSat::encode(unsafe { minisat::sys::minisat_newLit(self.ptr) })
    }

    fn negate(&self, lit: Literal) -> Literal {
        MiniSat::encode(unsafe { minisat::sys::minisat_negate(MiniSat::decode(lit)) })
    }

    fn add_clause(&mut self, lits: &[Literal]) {
        unsafe { minisat::sys::minisat_addClause_begin(self.ptr) };
        for lit in lits {
            unsafe { minisat::sys::minisat_addClause_addLit(self.ptr, MiniSat::decode(*lit)) };
        }
        unsafe { minisat::sys::minisat_addClause_commit(self.ptr) };
    }

    fn solve_with(&mut self, lits: &[Literal]) -> bool {
        unsafe { minisat::sys::minisat_solve_begin(self.ptr) };
        for lit in lits {
            unsafe { minisat::sys::minisat_solve_addLit(self.ptr, MiniSat::decode(*lit)) };
        }
        MiniSat::is_true(unsafe { minisat::sys::minisat_solve_commit(self.ptr) })
    }

    fn get_value(&self, lit: Literal) -> bool {
        MiniSat::is_true(unsafe {
            minisat::sys::minisat_modelValue_Lit(self.ptr, MiniSat::decode(lit))
        })
    }

    fn get_name(&self) -> &'static str {
        "MiniSat"
    }

    fn num_variables(&self) -> u32 {
        unsafe { minisat::sys::minisat_num_vars(self.ptr) as u32 }
    }

    fn num_clauses(&self) -> usize {
        unsafe { minisat::sys::minisat_num_clauses(self.ptr) as usize }
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
    num_clauses: usize,
    solver: varisat::Solver<'a>,
    solution: bit_vec::BitVec,
    temp: Vec<varisat::Lit>,
}

#[cfg(feature = "varisat")]
impl<'a> Default for VariSat<'a> {
    fn default() -> Self {
        VariSat {
            num_variables: 0,
            num_clauses: 0,
            solver: varisat::Solver::new(),
            solution: bit_vec::BitVec::new(),
            temp: Vec::new(),
        }
    }
}

#[cfg(feature = "varisat")]
impl<'a> VariSat<'a> {
    fn encode(lit: varisat::Lit) -> Literal {
        Literal {
            value: lit.code() as u32,
        }
    }

    fn decode(lit: Literal) -> varisat::Lit {
        varisat::Lit::from_code(lit.value as usize)
    }
}

#[cfg(feature = "varisat")]
impl<'a> Solver for VariSat<'a> {
    fn add_variable(&mut self) -> Literal {
        let var = varisat::Var::from_index(self.num_variables as usize);
        self.num_variables += 1;
        VariSat::encode(varisat::Lit::from_var(var, false))
    }

    fn negate(&self, lit: Literal) -> Literal {
        VariSat::encode(!VariSat::decode(lit))
    }

    fn add_clause(&mut self, lits: &[Literal]) {
        self.temp.clear();
        self.temp
            .extend(lits.iter().map(|lit| VariSat::decode(*lit)));
        self.solver.add_clause(&self.temp);
        self.num_clauses += 1;
    }

    fn solve_with(&mut self, lits: &[Literal]) -> bool {
        self.temp.clear();
        self.temp
            .extend(lits.iter().map(|lit| VariSat::decode(*lit)));
        self.solver.assume(&self.temp);

        self.solution.truncate(0);
        let solvable = self.solver.solve().unwrap();
        if solvable {
            self.solution.grow(self.num_variables() as usize, false);
            for lit in self.solver.model().unwrap() {
                if lit.is_positive() {
                    let var = lit.index();
                    self.solution.set(var, true);
                }
            }
        }
        solvable
    }

    fn get_value(&self, lit: Literal) -> bool {
        let lit = VariSat::decode(lit);
        let var = lit.index();
        self.solution.get(var).unwrap() ^ lit.is_negative()
    }

    fn get_name(&self) -> &'static str {
        "VariSat"
    }

    fn num_variables(&self) -> u32 {
        self.num_variables
    }

    fn num_clauses(&self) -> usize {
        self.num_clauses
    }
}

/// An advanced SAT solver supporting XOR clauses.
#[cfg(feature = "cryptominisat")]
pub struct CryptoMiniSat {
    solver: cryptominisat::Solver,
    num_clauses: usize,
    temp: Vec<cryptominisat::Lit>,
}

#[cfg(feature = "cryptominisat")]
impl Default for CryptoMiniSat {
    fn default() -> Self {
        CryptoMiniSat {
            solver: cryptominisat::Solver::new(),
            num_clauses: 0,
            temp: Vec::new(),
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
    fn add_variable(&mut self) -> Literal {
        CryptoMiniSat::encode(self.solver.new_var())
    }

    fn negate(&self, lit: Literal) -> Literal {
        Literal {
            value: lit.value ^ 1,
        }
    }

    fn add_clause(&mut self, lits: &[Literal]) {
        self.temp.clear();
        self.temp
            .extend(lits.iter().map(|lit| CryptoMiniSat::decode(*lit)));
        self.solver.add_clause(&self.temp);
        self.num_clauses += 1;
    }

    fn add_xor_clause(&mut self, lit1: Literal, lit2: Literal, lit3: Literal) {
        let lits = [
            CryptoMiniSat::decode(lit1),
            CryptoMiniSat::decode(lit2),
            CryptoMiniSat::decode(lit3),
        ];
        self.solver.add_xor_literal_clause(&lits, false);
    }

    fn solve_with(&mut self, lits: &[Literal]) -> bool {
        self.temp.clear();
        self.temp
            .extend(lits.iter().map(|lit| CryptoMiniSat::decode(*lit)));
        self.solver.solve_with_assumptions(&self.temp) == cryptominisat::Lbool::True
    }

    fn get_value(&self, lit: Literal) -> bool {
        self.solver.is_true(CryptoMiniSat::decode(lit))
    }

    fn get_name(&self) -> &'static str {
        "CryptoMiniSat"
    }

    fn num_variables(&self) -> u32 {
        self.solver.nvars()
    }

    fn num_clauses(&self) -> usize {
        self.num_clauses
    }
}

/// MiniSAT reimplemented in pure rust.
#[cfg(feature = "batsat")]
pub struct BatSat {
    solver: batsat::BasicSolver,
    temp: Vec<batsat::Lit>,
}

#[cfg(feature = "batsat")]
impl Default for BatSat {
    fn default() -> Self {
        BatSat {
            solver: batsat::Solver::new(Default::default(), Default::default()),
            temp: Vec::new(),
        }
    }
}

#[cfg(feature = "batsat")]
impl BatSat {
    fn encode(lit: batsat::Lit) -> Literal {
        Literal {
            value: lit.as_index() as u32,
        }
    }

    fn decode(lit: Literal) -> batsat::Lit {
        batsat::intmap::AsIndex::from_index(lit.value as usize)
    }
}

#[cfg(feature = "batsat")]
impl Solver for BatSat {
    fn add_variable(&mut self) -> Literal {
        let var = self.solver.new_var_default();
        BatSat::encode(batsat::Lit::new(var, true))
    }

    fn negate(&self, lit: Literal) -> Literal {
        Literal {
            value: lit.value ^ 1,
        }
    }

    fn add_clause(&mut self, lits: &[Literal]) {
        self.temp.clear();
        self.temp
            .extend(lits.iter().map(|lit| BatSat::decode(*lit)));
        self.solver.add_clause_reuse(&mut self.temp);
    }

    fn solve_with(&mut self, lits: &[Literal]) -> bool {
        self.temp.clear();
        self.temp
            .extend(lits.iter().map(|lit| BatSat::decode(*lit)));
        self.solver.solve_limited(&self.temp) == batsat::lbool::TRUE
    }

    fn get_value(&self, lit: Literal) -> bool {
        self.solver.value_lit(BatSat::decode(lit)) == batsat::lbool::TRUE
    }

    fn get_name(&self) -> &'static str {
        "BatSat"
    }

    fn num_variables(&self) -> u32 {
        self.solver.num_vars()
    }

    fn num_clauses(&self) -> usize {
        self.solver.num_clauses() as usize
    }
}

/// A state of the art SAT solver.
#[cfg(feature = "cadical")]
#[derive(Default)]
pub struct CaDiCaL {
    solver: cadical::Solver,
    num_vars: u32,
}

impl CaDiCaL {
    pub fn with_config(config: &str) -> Self {
        let solver = cadical::Solver::with_config(config).unwrap();
        CaDiCaL {
            solver,
            num_vars: 0,
        }
    }
}

#[cfg(feature = "cadical")]
impl Solver for CaDiCaL {
    fn add_variable(&mut self) -> Literal {
        self.num_vars += 1;
        Literal {
            value: self.num_vars,
        }
    }

    fn negate(&self, lit: Literal) -> Literal {
        Literal {
            value: -(lit.value as i32) as u32,
        }
    }

    fn add_clause(&mut self, lits: &[Literal]) {
        self.solver
            .add_clause(lits.iter().map(|lit| lit.value as i32));
    }

    fn solve_with(&mut self, lits: &[Literal]) -> bool {
        self.solver
            .solve_with(lits.iter().map(|lit| lit.value as i32))
            .unwrap()
    }

    fn get_value(&self, lit: Literal) -> bool {
        self.solver.value(lit.value as i32) == Some(true)
    }

    fn get_name(&self) -> &'static str {
        "CaDiCaL"
    }

    fn num_variables(&self) -> u32 {
        self.num_vars
    }

    fn num_clauses(&self) -> usize {
        self.solver.num_clauses() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(sat: &mut dyn Solver) {
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
        let c = sat.add_variable();
        sat.add_xor_clause(a, b, c);
        assert!(sat.solve());
        assert!(!sat.get_value(a));
        assert!(sat.get_value(b));
        assert!(sat.get_value(c));
        sat.add_clause(&[a, sat.negate(b)]);
        assert!(!sat.solve());
    }

    #[cfg(feature = "minisat")]
    #[test]
    fn minisat() {
        let mut sat: MiniSat = Default::default();
        test(&mut sat);
    }

    #[cfg(feature = "varisat")]
    #[test]
    fn varisat() {
        let mut sat: VariSat = Default::default();
        test(&mut sat);
    }

    #[cfg(feature = "cryptominisat")]
    #[test]
    fn cryptominisat() {
        let mut sat: CryptoMiniSat = Default::default();
        test(&mut sat);
    }

    #[cfg(feature = "batsat")]
    #[test]
    fn batsat() {
        let mut sat: BatSat = Default::default();
        test(&mut sat);
    }

    #[cfg(feature = "cadical")]
    #[test]
    fn cadical() {
        let mut sat: CaDiCaL = Default::default();
        test(&mut sat);
    }
}
