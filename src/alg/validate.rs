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
    BinaryRelations, BooleanLogic, BooleanSolver, BoundedOrder, Countable, Domain, FixedSet,
    Lattice, Logic, MeetSemilattice, Monoid, Operations, PartialOrder, Permutations, Power,
    Product2, Relations, Semigroup, SmallSet, Solver, UnaryOperations, Vector, BOOLEAN,
};

pub fn validate_domain<DOM>(domain: DOM)
where
    DOM: Domain,
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
    validate_domain(FixedSet::<5>);
    validate_domain(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_domain(Power::new(SmallSet::new(3), BOOLEAN));
    validate_domain(Product2::new(BOOLEAN, SmallSet::new(3)));
    validate_domain(Relations::new(SmallSet::new(3), 3));
    validate_domain(BinaryRelations::new(SmallSet::new(3)));
    validate_domain(Operations::new(SmallSet::new(2), 2));
    validate_domain(UnaryOperations::new(SmallSet::new(3)));
    validate_domain(Permutations::new(SmallSet::new(4)));
}

fn validate_countable<DOM>(domain: DOM, size: usize)
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
        let elem = domain.get_elem(&logic, index);
        assert!(domain.contains(&mut logic, elem.slice()));
        assert_eq!(domain.get_index(elem.slice()), index);
    }

    // equality works
    let mut logic = Logic();
    for index0 in 0..domain.size() {
        let elem0 = domain.get_elem(&logic, index0);
        assert!(domain.equals(&mut logic, elem0.slice(), elem0.slice()));
        for index1 in 0..index0 {
            let elem1 = domain.get_elem(&logic, index1);
            assert!(!domain.equals(&mut logic, elem0.slice(), elem1.slice()));
        }
    }

    // onehot works
    let small = SmallSet::new(domain.size());
    let mut logic = Logic();
    for index in 0..domain.size() {
        let elem0 = domain.get_elem(&logic, index);
        let elem0 = domain.onehot(&mut logic, elem0.slice());
        let elem1 = small.get_elem(&logic, index);
        assert_eq!(elem0, elem1);
    }
}

#[test]
fn countable() {
    validate_countable(BOOLEAN, 2);
    validate_countable(SmallSet::new(5), 5);
    validate_countable(FixedSet::<5>, 5);
    validate_countable(Power::new(BOOLEAN, SmallSet::new(3)), 8);
    validate_countable(Power::new(SmallSet::new(3), BOOLEAN), 9);
    validate_countable(Product2::new(BOOLEAN, SmallSet::new(3)), 6);
    validate_countable(Relations::new(SmallSet::new(2), 3), 256);
    validate_countable(BinaryRelations::new(SmallSet::new(2)), 16);
    validate_countable(Operations::new(SmallSet::new(2), 2), 16);
    validate_countable(UnaryOperations::new(SmallSet::new(3)), 27);
    validate_countable(Permutations::new(SmallSet::new(0)), 1);
    validate_countable(Permutations::new(SmallSet::new(1)), 1);
    validate_countable(Permutations::new(SmallSet::new(4)), 24);
}

pub fn validate_partial_order<DOM>(domain: DOM)
where
    DOM: PartialOrder,
{
    assert!(domain.check_partial_order());
}

#[test]
fn partial_order() {
    validate_partial_order(BOOLEAN);
    validate_partial_order(SmallSet::new(7));
    validate_partial_order(FixedSet::<7>);
    validate_partial_order(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_partial_order(Product2::new(BOOLEAN, BOOLEAN));
    validate_partial_order(Relations::new(SmallSet::new(2), 3));
    validate_partial_order(BinaryRelations::new(SmallSet::new(3)));
}

pub fn validate_bounded_order<DOM>(domain: DOM)
where
    DOM: BoundedOrder,
{
    // top is in domain
    let mut logic = Logic();
    let top = domain.get_top(&logic);
    let test = domain.contains(&mut logic, top.slice());
    assert!(test);

    // bottom is in domain
    let bottom = domain.get_bottom(&logic);
    let test = domain.contains(&mut logic, bottom.slice());
    assert!(test);
    let test = domain.is_edge(&mut logic, bottom.slice(), top.slice());
    assert!(test);

    // top is above everything
    let mut logic = Solver::new("");
    let top = domain.get_top(&logic);
    let elem = domain.add_variable(&mut logic);
    let test = domain.is_edge(&mut logic, elem.slice(), top.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // bottom is below everything
    let mut logic = Solver::new("");
    let bottom = domain.get_bottom(&logic);
    let elem = domain.add_variable(&mut logic);
    let test = domain.is_edge(&mut logic, bottom.slice(), elem.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn bounded_order() {
    validate_bounded_order(BOOLEAN);
    validate_bounded_order(SmallSet::new(7));
    validate_bounded_order(FixedSet::<7>);
    validate_bounded_order(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_bounded_order(Product2::new(BOOLEAN, BOOLEAN));
    validate_bounded_order(Relations::new(SmallSet::new(2), 3));
    validate_bounded_order(BinaryRelations::new(SmallSet::new(3)));
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
    let test0 = domain.is_edge(&mut logic, elem2.slice(), elem0.slice());
    let test1 = domain.is_edge(&mut logic, elem2.slice(), elem1.slice());
    logic.bool_add_clause2(logic.bool_not(test0), logic.bool_not(test1));
    assert!(!logic.bool_solvable());

    // meet is maximal lower bound
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.add_variable(&mut logic);
    let test = domain.is_edge(&mut logic, elem2.slice(), elem0.slice());
    logic.bool_add_clause1(test);
    let test = domain.is_edge(&mut logic, elem2.slice(), elem1.slice());
    logic.bool_add_clause1(test);
    let elem3 = domain.meet(&mut logic, elem0.slice(), elem1.slice());
    let test = domain.is_edge(&mut logic, elem2.slice(), elem3.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn meet_semilattice() {
    validate_meet_semilattice(BOOLEAN);
    validate_meet_semilattice(SmallSet::new(7));
    validate_meet_semilattice(FixedSet::<7>);
    validate_meet_semilattice(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_meet_semilattice(Product2::new(BOOLEAN, Power::new(BOOLEAN, BOOLEAN)));
    validate_meet_semilattice(Relations::new(SmallSet::new(2), 3));
    validate_meet_semilattice(BinaryRelations::new(SmallSet::new(3)));
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
    let test0 = domain.is_edge(&mut logic, elem0.slice(), elem2.slice());
    let test1 = domain.is_edge(&mut logic, elem1.slice(), elem2.slice());
    logic.bool_add_clause2(logic.bool_not(test0), logic.bool_not(test1));
    assert!(!logic.bool_solvable());

    // join is minimal upper bound
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.add_variable(&mut logic);
    let test = domain.is_edge(&mut logic, elem0.slice(), elem2.slice());
    logic.bool_add_clause1(test);
    let test = domain.is_edge(&mut logic, elem1.slice(), elem2.slice());
    logic.bool_add_clause1(test);
    let elem3 = domain.join(&mut logic, elem0.slice(), elem1.slice());
    let test = domain.is_edge(&mut logic, elem3.slice(), elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());
}

#[test]
fn lattice() {
    validate_lattice(BOOLEAN);
    validate_lattice(SmallSet::new(7));
    validate_lattice(FixedSet::<7>);
    validate_lattice(Power::new(BOOLEAN, SmallSet::new(3)));
    validate_lattice(Product2::new(BOOLEAN, Power::new(BOOLEAN, BOOLEAN)));
    validate_lattice(Relations::new(SmallSet::new(2), 3));
    validate_lattice(BinaryRelations::new(SmallSet::new(3)));
}

pub fn validate_semigroup<DOM>(domain: DOM)
where
    DOM: Semigroup,
{
    // product is in domain
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.product(&mut logic, elem0.slice(), elem1.slice());
    let test = domain.contains(&mut logic, elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test));
    assert!(!logic.bool_solvable());

    // associativity
    let mut logic = Solver::new("");
    let elem0 = domain.add_variable(&mut logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.add_variable(&mut logic);
    let elem3 = domain.product(&mut logic, elem0.slice(), elem1.slice());
    let elem4 = domain.product(&mut logic, elem3.slice(), elem2.slice());
    let elem5 = domain.product(&mut logic, elem1.slice(), elem2.slice());
    let elem6 = domain.product(&mut logic, elem0.slice(), elem5.slice());
    let test0 = domain.equals(&mut logic, elem4.slice(), elem6.slice());
    logic.bool_add_clause1(logic.bool_not(test0));
    assert!(!logic.bool_solvable());
}

#[test]
fn semigroup() {
    validate_semigroup(BinaryRelations::new(SmallSet::new(3)));
    validate_semigroup(UnaryOperations::new(SmallSet::new(3)));
    validate_semigroup(Permutations::new(SmallSet::new(3)));
    validate_semigroup(Product2::new(
        Permutations::new(SmallSet::new(2)),
        BinaryRelations::new(SmallSet::new(2)),
    ));
    validate_semigroup(Power::new(
        UnaryOperations::new(SmallSet::new(2)),
        SmallSet::new(2),
    ));
}

pub fn validate_monoid<DOM>(domain: DOM)
where
    DOM: Monoid,
{
    // identity is in domain
    let mut logic = Logic();
    let elem = domain.get_identity(&logic);
    let test0 = domain.contains(&mut logic, elem.slice());
    let test1 = domain.is_identity(&mut logic, elem.slice());
    assert!(test0 && test1);

    // identity is unique
    let mut logic = Solver::new("");
    let elem0 = domain.get_identity(&logic);
    let elem1 = domain.add_variable(&mut logic);
    let test0 = domain.is_identity(&mut logic, elem1.slice());
    let test1 = domain.equals(&mut logic, elem0.slice(), elem1.slice());
    logic.bool_add_clause1(test0);
    logic.bool_add_clause1(logic.bool_not(test1));
    assert!(!logic.bool_solvable());

    // left identity law
    let mut logic = Solver::new("");
    let elem0 = domain.get_identity(&logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.product(&mut logic, elem0.slice(), elem1.slice());
    let test0 = domain.equals(&mut logic, elem1.slice(), elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test0));
    assert!(!logic.bool_solvable());

    // right identity law
    let mut logic = Solver::new("");
    let elem0 = domain.get_identity(&logic);
    let elem1 = domain.add_variable(&mut logic);
    let elem2 = domain.product(&mut logic, elem1.slice(), elem0.slice());
    let test0 = domain.equals(&mut logic, elem1.slice(), elem2.slice());
    logic.bool_add_clause1(logic.bool_not(test0));
    assert!(!logic.bool_solvable());
}

#[test]
fn monoid() {
    validate_monoid(BinaryRelations::new(SmallSet::new(3)));
    validate_monoid(UnaryOperations::new(SmallSet::new(3)));
    validate_monoid(Permutations::new(SmallSet::new(3)));
    validate_monoid(Product2::new(
        Permutations::new(SmallSet::new(2)),
        BinaryRelations::new(SmallSet::new(2)),
    ));
    validate_monoid(Power::new(
        UnaryOperations::new(SmallSet::new(2)),
        SmallSet::new(2),
    ));
}

#[test]
fn binary_relations() {
    let mut logic = Solver::new("");
    let domain = BinaryRelations::new(SmallSet::new(4));
    let elem = domain.add_variable(&mut logic);
    let test = domain.is_transitive(&mut logic, elem.slice());
    logic.bool_add_clause1(test);
    let count = logic.bool_find_num_models_method1(elem.copy_iter());
    assert_eq!(count, 3994);

    let mut logic = Solver::new("");
    let domain = BinaryRelations::new(SmallSet::new(7));
    let elem = domain.add_variable(&mut logic);
    let test = domain.is_equivalence(&mut logic, elem.slice());
    logic.bool_add_clause1(test);
    let count = logic.bool_find_num_models_method1(elem.copy_iter());
    assert_eq!(count, 877);

    let mut logic = Solver::new("");
    let domain = BinaryRelations::new(SmallSet::new(5));
    let elem = domain.add_variable(&mut logic);
    let test = domain.is_partial_order(&mut logic, elem.slice());
    logic.bool_add_clause1(test);
    let count = logic.bool_find_num_models_method1(elem.copy_iter());
    assert_eq!(count, 4231);
}

#[test]
fn unary_operations() {
    let mut logic = Solver::new("");
    let domain = UnaryOperations::new(SmallSet::new(6));
    let elem = domain.add_variable(&mut logic);
    let test = domain.is_permutation(&mut logic, elem.slice());
    logic.bool_add_clause1(test);
    let count = logic.bool_find_num_models_method1(elem.copy_iter());
    assert_eq!(count, 720);
}
