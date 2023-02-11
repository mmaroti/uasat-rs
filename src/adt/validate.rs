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
    BooleanLogic, BooleanSolver, Countable, GenVec, Logic, PartialOrder, Power, Product2, SmallSet,
    Solver, BOOLEAN,
};

pub fn validate_countable<DOM>(domain: DOM, size: usize)
where
    DOM: Countable,
{
    assert_eq!(domain.size(), size);

    let mut solver = Solver::new("");
    let elem = domain.add_variable(&mut solver);
    let test = domain.contains(&mut solver, &elem);
    solver.bool_add_clause(&[test]);
    let count = solver.bool_find_num_models_method1(elem.copy_iter());
    assert_eq!(count, size);

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
    let mut solver = Solver::new("");
    let elem = domain.add_variable(&mut solver);
    let test = domain.leq(&mut solver, elem.slice(), elem.slice());
    let test = solver.bool_not(test);
    solver.bool_add_clause(&[test]);
    assert!(!solver.bool_solvable());

    let mut solver = Solver::new("");
    let elem0 = domain.add_variable(&mut solver);
    let elem1 = domain.add_variable(&mut solver);
    let test = domain.leq(&mut solver, elem0.slice(), elem1.slice());
    solver.bool_add_clause(&[test]);
    let test = domain.leq(&mut solver, elem1.slice(), elem0.slice());
    solver.bool_add_clause(&[test]);
    let test = solver.bool_cmp_neq(elem0.copy_iter().zip(elem1.copy_iter()));
    solver.bool_add_clause(&[test]);
    assert!(!solver.bool_solvable());

    let mut solver = Solver::new("");
    let elem0 = domain.add_variable(&mut solver);
    let elem1 = domain.add_variable(&mut solver);
    let elem2 = domain.add_variable(&mut solver);
    let test = domain.leq(&mut solver, elem0.slice(), elem1.slice());
    solver.bool_add_clause(&[test]);
    let test = domain.leq(&mut solver, elem1.slice(), elem2.slice());
    solver.bool_add_clause(&[test]);
    let test = domain.leq(&mut solver, elem0.slice(), elem2.slice());
    let test = solver.bool_not(test);
    solver.bool_add_clause(&[test]);
    assert!(!solver.bool_solvable());
}

#[test]
fn partial_order() {
    validate_partial_order(BOOLEAN);
    validate_partial_order(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_partial_order(Product2::new(BOOLEAN, BOOLEAN));
}
