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

#[cfg(feature = "varisat")]
extern crate bit_vec;
#[cfg(feature = "minisat")]
extern crate minisat;
#[cfg(feature = "varisat")]
extern crate varisat;

#[cfg(feature = "varisat")]
use bit_vec::BitVec;
use std::fmt;

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
    fn bool_not(self: &Self, elem: Self::Elem) -> Self::Elem;

    /// Returns the logical or (lattice join) of a pair of elements.
    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the exclusive or (boolean addition) of a pair of elements.
    fn bool_add(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns the logical and (lattice meet) of a pair of elements.
    fn bool_and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let tmp = self.bool_or(self.bool_not(elem1), self.bool_not(elem2));
        self.bool_not(tmp)
    }

    /// Returns the logical equivalence of a pair of elements.
    fn bool_equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        self.bool_add(elem1, self.bool_not(elem2))
    }

    /// Returns the logical implication of a pair of elements.
    fn bool_leq(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        self.bool_or(self.bool_not(elem1), elem2)
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

    fn bool_not(self: &Self, elem: Self::Elem) -> Self::Elem {
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
pub struct FreeAlg {
    solver: Box<Solver>,
    unit: Literal,
    zero: Literal,
}

impl FreeAlg {
    /// Creates a new free boolean algebra.
    pub fn new(solver_name: &str) -> Self {
        let mut solver = create_solver(solver_name);
        let unit = solver.add_variable();
        let zero = solver.negate(unit);
        FreeAlg { solver, unit, zero }
    }

    /// Adds a new free variable to the algebra
    pub fn add_variable(self: &mut Self) -> Literal {
        self.solver.add_variable()
    }

    /// Runs the solver and finds a model where the given assumptions are true.
    pub fn find_model(self: &mut Self, vars: &[Literal]) -> bool {
        self.solver.solve_with(vars)
    }

    /// Returns the logical value of the element in the found model.
    pub fn get_value(self: &Self, elem: Literal) -> bool {
        self.solver.get_value(elem)
    }
}

impl BoolAlg for FreeAlg {
    type Elem = Literal;

    fn bool_unit(self: &Self) -> Self::Elem {
        self.unit
    }

    fn bool_zero(self: &Self) -> Self::Elem {
        self.zero
    }

    fn bool_not(self: &Self, elem: Self::Elem) -> Self::Elem {
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

/// Tries to create a SAT solver with the given name. Currently only "varisat"
/// and "minisat" (not on wasm) are supported. Use "" to match the first
/// available solver.
pub fn create_solver(name: &str) -> Box<Solver> {
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

    if enabled_solvers == 0 {
        panic!("No SAT solvers are available.")
    } else {
        panic!("Unknown SAT solver: {}", name);
    }
}

impl fmt::Debug for Solver {
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
        formula.add_clause(lits.iter().map(|lit| VariSat::decode(*lit)));
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
        let mut alg = FreeAlg::new("");
        let a = alg.add_variable();
        let b = alg.add_variable();
        let c = alg.bool_and(a, b);
        assert!(alg.find_model(&[c]));
        assert!(alg.get_value(a), true);
        assert!(alg.get_value(b), true);
        let d = alg.bool_not(a);
        assert!(!alg.find_model(&[c, d]));
    }

    #[cfg(feature = "minisat")]
    #[test]
    fn minisat() {
        let mut sat: MiniSat = Default::default();
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

    #[cfg(feature = "varisat")]
    #[test]
    fn varisat() {
        let mut sat: VariSat = Default::default();
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
