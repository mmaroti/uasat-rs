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
    BooleanLogic, BooleanSolver, BoundedOrder, Countable, GenVec, Logic, MeetSemilattice,
    PartialOrder, Power, Product2, SmallSet, Solver, BOOLEAN,
};

pub fn validate_countable<DOM>(domain: DOM, size: usize)
where
    DOM: Countable,
{
    assert_eq!(domain.size(), size);

    // count matches
    let mut alg = Solver::new("");
    let elem = domain.add_variable(&mut alg);
    let test = domain.contains(&mut alg, &elem);
    alg.bool_add_clause(&[test]);
    let count = alg.bool_find_num_models_method1(elem.copy_iter());
    assert_eq!(count, size);

    // elem and index are inverses of each other
    let mut alg = Logic();
    for index in 0..domain.size() {
        let elem = domain.elem(index);
        assert!(domain.contains(&mut alg, elem.slice()));
        assert_eq!(domain.index(elem.slice()), index);
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
    let mut alg = Solver::new("");
    let elem = domain.add_variable(&mut alg);
    let test = domain.leq(&mut alg, elem.slice(), elem.slice());
    let test = alg.bool_not(test);
    alg.bool_add_clause(&[test]);
    assert!(!alg.bool_solvable());

    // antisymmetric
    let mut alg = Solver::new("");
    let elem0 = domain.add_variable(&mut alg);
    let elem1 = domain.add_variable(&mut alg);
    let test = domain.leq(&mut alg, elem0.slice(), elem1.slice());
    alg.bool_add_clause(&[test]);
    let test = domain.leq(&mut alg, elem1.slice(), elem0.slice());
    alg.bool_add_clause(&[test]);
    let test = domain.equals(&mut alg, elem0.slice(), elem1.slice());
    let test = alg.bool_not(test);
    alg.bool_add_clause(&[test]);
    assert!(!alg.bool_solvable());

    // transitive
    let mut alg = Solver::new("");
    let elem0 = domain.add_variable(&mut alg);
    let elem1 = domain.add_variable(&mut alg);
    let elem2 = domain.add_variable(&mut alg);
    let test = domain.leq(&mut alg, elem0.slice(), elem1.slice());
    alg.bool_add_clause(&[test]);
    let test = domain.leq(&mut alg, elem1.slice(), elem2.slice());
    alg.bool_add_clause(&[test]);
    let test = domain.leq(&mut alg, elem0.slice(), elem2.slice());
    let test = alg.bool_not(test);
    alg.bool_add_clause(&[test]);
    assert!(!alg.bool_solvable());
}

#[test]
fn partial_order() {
    validate_partial_order(BOOLEAN);
    validate_partial_order(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_partial_order(Product2::new(BOOLEAN, BOOLEAN));
}

pub fn validate_bounded_order<DOM>(domain: DOM)
where
    DOM: BoundedOrder,
{
    // top is in domain
    let mut alg = Logic();
    let top = domain.top();
    assert!(domain.contains(&mut alg, top.slice()));

    // bottom is in domain
    let bottom = domain.bottom();
    assert!(domain.contains(&mut alg, bottom.slice()));
    assert!(domain.leq(&mut alg, bottom.slice(), top.slice()));

    // top is above everything
    let mut alg = Solver::new("");
    let top = alg.bool_lift_vec(top.slice());
    let elem = domain.add_variable(&mut alg);
    let test = domain.leq(&mut alg, elem.slice(), top.slice());
    let test = alg.bool_not(test);
    alg.bool_add_clause(&[test]);
    assert!(!alg.bool_solvable());

    // bottom is below everything
    let mut alg = Solver::new("");
    let bottom = alg.bool_lift_vec(bottom.slice());
    let elem = domain.add_variable(&mut alg);
    let test = domain.leq(&mut alg, bottom.slice(), elem.slice());
    let test = alg.bool_not(test);
    alg.bool_add_clause(&[test]);
    assert!(!alg.bool_solvable());
}

#[test]
fn bounded_order() {
    validate_bounded_order(BOOLEAN);
    validate_bounded_order(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_bounded_order(Product2::new(BOOLEAN, BOOLEAN));
}

pub fn validate_meet_semilattice<DOM>(domain: DOM)
where
    DOM: MeetSemilattice,
{
    // meet is in domain
    let mut alg = Solver::new("");
    let elem0 = domain.add_variable(&mut alg);
    let elem1 = domain.add_variable(&mut alg);
    let elem2 = domain.meet(&mut alg, elem0.slice(), elem1.slice());
    let test = domain.contains(&mut alg, elem2.slice());
    let test = alg.bool_not(test);
    alg.bool_add_clause(&[test]);
    assert!(!alg.bool_solvable());

    // meet is lower bound
    let mut alg = Solver::new("");
    let elem0 = domain.add_variable(&mut alg);
    let elem1 = domain.add_variable(&mut alg);
    let elem2 = domain.meet(&mut alg, elem0.slice(), elem1.slice());
    let test0 = domain.leq(&mut alg, elem2.slice(), elem0.slice());
    let test1 = domain.leq(&mut alg, elem2.slice(), elem1.slice());
    let test = [alg.bool_not(test0), alg.bool_not(test1)];
    alg.bool_add_clause(&test);
    assert!(!alg.bool_solvable());

    // meet is maximal lower bound
    let mut alg = Solver::new("");
    let elem0 = domain.add_variable(&mut alg);
    let elem1 = domain.add_variable(&mut alg);
    let elem2 = domain.add_variable(&mut alg);
    let test = domain.leq(&mut alg, elem2.slice(), elem0.slice());
    alg.bool_add_clause(&[test]);
    let test = domain.leq(&mut alg, elem2.slice(), elem1.slice());
    alg.bool_add_clause(&[test]);
    let elem3 = domain.meet(&mut alg, elem0.slice(), elem1.slice());
    let test = domain.leq(&mut alg, elem2.slice(), elem3.slice());
    let test = alg.bool_not(test);
    alg.bool_add_clause(&[test]);
    assert!(!alg.bool_solvable());
}

#[test]
fn meet_semilattice() {
    validate_meet_semilattice(BOOLEAN);
    validate_meet_semilattice(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_meet_semilattice(Product2::new(BOOLEAN, BOOLEAN));
}
