/*
* Copyright (C) 2023, Miklos Maroti
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
    BooleanLogic, BooleanSolver, BoundedOrder, Countable, GenVec, Lattice, Logic,
    MeetSemilattice, PartialOrder, Power, Product2, SmallSet, Solver, BOOLEAN,
};

pub fn validate_domain<DOM>(domain: DOM)
where
    DOM: Countable,
{
    // containment
    let mut logic = Solver::new("");
    let elem = domain.add_variable(&mut logic);
    let test = domain.contains(&mut logic, elem.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // reflexivity
    let mut logic = Solver::new("");
    let elem = domain.add_variable(&mut logic);
    let test = domain.equals(&mut logic, elem.slice(), elem.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // equality
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let test = domain.equals(&mut logic, elem0.slice(), elem1.slice());
    logic.bool_add_clause1(test);
    let test = logic.bool_cmp_equ(elem0.copy_iter().zip(elem1.copy_iter()));
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn domain() {
    validate_domain(BOOLEAN);
    validate_domain(SmallSet::new(5));
    validate_domain(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_domain(Power::new(SmallSet::new(3), BOOLEAN));
    validate_domain(Product2::new(BOOLEAN, SmallSet::new(3)));
}

pub fn validate_countable<DOM>(domain: DOM, size: usize)
where
    DOM: Countable,
{
    assert_eq!(domain.size(), size);

    // count matches
    let mut logic = Solver::new("");
    let elem = domain.add_variable(&mut logic);
    let test = domain.contains(&mut logic, elem.slice());
    logic.bool_add_clause1(test);
    let count = logic.bool_find_num_models_method1(elem.copy_iter());
    assert_eq!(count, size);

    // elem and index are inverses of each other
    let mut logic = Logic();
    for index in 0..domain.size() {
        let elem = domain.elem(index);
        assert!(domain.contains(&mut logic, elem.slice()));
        assert_eq!(domain.index(elem.slice()), index);
    }

    // equality works
    let mut logic = Logic();
    for index0 in 0..domain.size() {
        let elem0 = domain.elem(index0);
        assert!(domain.equals(&mut logic, elem0.slice(), elem0.slice()));
        for index1 in 0..index0 {
            let elem1 = domain.elem(index1);
            assert!(!domain.equals(&mut logic, elem0.slice(), elem1.slice()));
        }
    }
}

#[test]
fn countable() {
    validate_countable(BOOLEAN, 2);
    validate_countable(SmallSet::new(5), 5);
    validate_countable(Power::new(BOOLEAN, SmallSet::new(3)), 8);
    validate_countable(Power::new(SmallSet::new(3), BOOLEAN), 9);
    validate_countable(Product2::new(BOOLEAN, SmallSet::new(3)), 6);
}

pub fn validate_partial_order<DOM>(domain: DOM)
where
    DOM: PartialOrder,
{
    // reflexive
    let mut logic = Solver::new("");
    let elem = domain.add_variable(&mut logic);
    let test = domain.leq(&mut logic, elem.slice(), elem.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // antisymmetric
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let test = domain.leq(&mut logic, elem0.slice(), elem1.slice());
    logic.bool_add_clause1(test);
    let test = domain.leq(&mut logic, elem1.slice(), elem0.slice());
    logic.bool_add_clause1(test);
    let test = domain.equals(&mut logic, elem0.slice(), elem1.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // transitive
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.add_variable(&mut logic);
    let test = domain.leq(&mut logic, elem0.slice(), elem1.slice());
    logic.bool_add_clause1(test);
    let test = domain.leq(&mut logic, elem1.slice(), elem2.slice());
    logic.bool_add_clause1(test);
    let test = domain.leq(&mut logic, elem0.slice(), elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn partial_order() {
    validate_partial_order(BOOLEAN);
    validate_partial_order(SmallSet::new(7));
    validate_partial_order(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_partial_order(Product2::new(BOOLEAN, BOOLEAN));
}

pub fn validate_bounded_order<DOM>(domain: DOM)
where
    DOM: BoundedOrder,
{
    // top is in domain
    let mut logic = Logic();
    let top = domain.top();
    assert!(domain.contains(&mut logic, top.slice()));

    // bottom is in domain
    let bottom = domain.bottom();
    assert!(domain.contains(&mut logic, bottom.slice()));
    assert!(domain.leq(&mut logic, bottom.slice(), top.slice()));

    // top is above everything
    let mut logic = Solver::new("");
    let top = logic.bool_lift_vec(top.copy_iter());
    let elem = domain.add_variable(&mut logic);
    let test = domain.leq(&mut logic, elem.slice(), top.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // bottom is below everything
    let mut logic = Solver::new("");
    let bottom = logic.bool_lift_vec(bottom.copy_iter());
    let elem = domain.add_variable(&mut logic);
    let test = domain.leq(&mut logic, bottom.slice(), elem.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn bounded_order() {
    validate_bounded_order(BOOLEAN);
    validate_bounded_order(SmallSet::new(7));
    validate_bounded_order(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_bounded_order(Product2::new(BOOLEAN, BOOLEAN));
}

pub fn validate_meet_semilattice<DOM>(domain: DOM)
where
    DOM: MeetSemilattice,
{
    // meet is in domain
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.meet(&mut logic, elem0.slice(), elem1.slice());
    let test = domain.contains(&mut logic, elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // meet is lower bound
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.meet(&mut logic, elem0.slice(), elem1.slice());
    let test0 = domain.leq(&mut logic, elem2.slice(), elem0.slice());
    let test1 = domain.leq(&mut logic, elem2.slice(), elem1.slice());
    logic.bool_add_clause2(logic.bool_not(test0), logic.bool_not(test1));
    assert!(!logic.bool_solvable());

    // meet is maximal lower bound
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.add_variable(&mut logic);
    let test = domain.leq(&mut logic, elem2.slice(), elem0.slice());
    logic.bool_add_clause1(test);
    let test = domain.leq(&mut logic, elem2.slice(), elem1.slice());
    logic.bool_add_clause1(test);
    let elem3 = domain.meet(&mut logic, elem0.slice(), elem1.slice());
    let test = domain.leq(&mut logic, elem2.slice(), elem3.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn meet_semilattice() {
    validate_meet_semilattice(BOOLEAN);
    validate_meet_semilattice(SmallSet::new(7));
    validate_meet_semilattice(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_meet_semilattice(Product2::new(BOOLEAN, Power::new(BOOLEAN, BOOLEAN)));
}

pub fn validate_lattice<DOM>(domain: DOM)
where
    DOM: Lattice,
{
    // join is in domain
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.join(&mut logic, elem0.slice(), elem1.slice());
    let test = domain.contains(&mut logic, elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // join is upper bound
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.join(&mut logic, elem0.slice(), elem1.slice());
    let test0 = domain.leq(&mut logic, elem0.slice(), elem2.slice());
    let test1 = domain.leq(&mut logic, elem1.slice(), elem2.slice());
    logic.bool_add_clause2(logic.bool_not(test0), logic.bool_not(test1));
    assert!(!logic.bool_solvable());

    // join is minimal lower bound
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.add_variable(&mut logic);
    let test = domain.leq(&mut logic, elem0.slice(), elem2.slice());
    logic.bool_add_clause1(test);
    let test = domain.leq(&mut logic, elem1.slice(), elem2.slice());
    logic.bool_add_clause1(test);
    let elem3 = domain.join(&mut logic, elem0.slice(), elem1.slice());
    let test = domain.leq(&mut logic, elem3.slice(), elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn lattice() {
    validate_lattice(BOOLEAN);
    validate_lattice(SmallSet::new(7));
    validate_lattice(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_lattice(Product2::new(BOOLEAN, Power::new(BOOLEAN, BOOLEAN)));
}
