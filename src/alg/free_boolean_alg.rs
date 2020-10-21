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

use super::{
    AdditiveGroup, BooleanAlgebra, BoundedPartialOrder, ClassicalDomain, DirectedGraph, Domain,
    Lattice, Monoid, PartialOrder, Ring, Semigroup, TwoElementAlg, UnitaryRing, TWO_ELEMENT_ALG,
};
use crate::solver::{create_solver, Literal, Solver};
use std::cell::Cell;

/// The free boolean algebra backed by a SAT solver.
pub struct FreeBooleanAlg {
    solver: Cell<Option<Box<dyn Solver>>>,
    unit: Literal,
    zero: Literal,
}

impl Default for FreeBooleanAlg {
    fn default() -> Self {
        Self::new("")
    }
}

impl FreeBooleanAlg {
    /// Creates a new free boolean algebra.
    pub fn new(solver_name: &str) -> Self {
        let mut solver = create_solver(solver_name);
        let unit = solver.add_variable();
        let zero = solver.negate(unit);
        solver.add_clause(&[unit]);
        let solver = Cell::new(Some(solver));
        Self { solver, unit, zero }
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
    pub fn get_solver_name(&self) -> &'static str {
        self.mutate(|solver| solver.get_name())
    }

    /// Returns a new generator element.
    pub fn add_generator(&self) -> Literal {
        self.mutate(|solver| solver.add_variable())
    }
}

impl Domain for FreeBooleanAlg {
    type Elem = Literal;

    type Logic = TwoElementAlg;

    fn logic(&self) -> &Self::Logic {
        &TWO_ELEMENT_ALG
    }

    fn contains(&self, _elem: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        // TODO: Check the number of variables
        true
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        let temp0 = self.edge(elem0, elem1);
        let temp1 = self.edge(elem1, elem0);
        self.logic().meet(&temp0, &temp1)
    }
}

impl ClassicalDomain for FreeBooleanAlg {}

impl DirectedGraph for FreeBooleanAlg {
    fn edge(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        self.mutate(|solver| {
            let not_elem1 = solver.negate(*elem1);
            !solver.solve_with(&[*elem0, not_elem1])
        })
    }
}

impl PartialOrder for FreeBooleanAlg {}

impl BoundedPartialOrder for FreeBooleanAlg {
    fn bot(&self) -> Self::Elem {
        self.zero
    }

    fn top(&self) -> Self::Elem {
        self.unit
    }
}

impl Lattice for FreeBooleanAlg {
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

impl BooleanAlgebra for FreeBooleanAlg {
    fn not(&self, elem: &Self::Elem) -> Self::Elem {
        self.mutate(|solver| solver.negate(*elem))
    }
}

impl AdditiveGroup for FreeBooleanAlg {
    fn zero(&self) -> Self::Elem {
        self.bot()
    }

    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        *elem
    }

    fn add(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.xor(elem0, elem1)
    }
}

impl Semigroup for FreeBooleanAlg {
    fn mul(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.meet(elem0, elem1)
    }
}

impl Monoid for FreeBooleanAlg {
    fn unit(&self) -> Self::Elem {
        self.top()
    }
}

impl Ring for FreeBooleanAlg {}

impl UnitaryRing for FreeBooleanAlg {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain() {
        let alg = FreeBooleanAlg::new("");
        let x = alg.add_generator();
        assert!(alg.edge(&x, &alg.unit()));
        assert!(!alg.edge(&alg.unit(), &x));
        assert!(alg.edge(&alg.zero(), &x));
        assert!(!alg.edge(&x, &alg.zero()));

        let y = alg.add_generator();
        let a = alg.join(&x, &y);
        let b = alg.meet(&x, &a);
        assert!(alg.equals(&b, &x));
        assert!(!alg.equals(&b, &y));

        let z = alg.add_generator();
        let c = alg.meet(&z, &a);
        let d = alg.join(&alg.meet(&z, &x), &alg.meet(&z, &y));
        assert!(alg.equals(&c, &d));
    }
}
