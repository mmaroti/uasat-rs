/*
* Copyright (C) 2022, Miklos Maroti
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
    BitSlice, BooleanLattice, BooleanLogic, BoundedOrder, DirectedGraph, Domain, Indexable,
    Lattice, MeetSemilattice, Operation, PartialOrder, Slice, Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean();

pub const BOOLEAN: Boolean = Boolean();

impl Domain for Boolean {
    fn num_bits(&self) -> usize {
        1
    }

    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), 1);
        logic.bool_unit()
    }

    fn equals<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        logic.bool_equ(elem0.get(0), elem1.get(0))
    }
}

impl Indexable for Boolean {
    fn size(&self) -> usize {
        2
    }

    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(index < 2);
        let value = logic.bool_lift(index != 0);
        LOGIC::Vector::from_elem(value)
    }

    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        assert!(elem.len() == 1);
        elem.get(0) as usize
    }

    fn onehot<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        debug_assert_eq!(elem.len(), 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(2);
        let elem = elem.get(0);
        result.push(logic.bool_not(elem));
        result.push(elem);
        result
    }
}

impl DirectedGraph for Boolean {
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        logic.bool_imp(elem0.get(0), elem1.get(0))
    }
}

impl PartialOrder for Boolean {}

impl BoundedOrder for Boolean {
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        Vector::from_elem(logic.bool_unit())
    }

    fn is_top<LOGIC>(&self, _logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), 1);
        elem.get(0)
    }

    fn get_bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        Vector::from_elem(logic.bool_zero())
    }

    fn is_bottom<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), 1);
        logic.bool_not(elem.get(0))
    }
}

impl MeetSemilattice for Boolean {
    fn meet<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        Vector::from_elem(logic.bool_and(elem0.get(0), elem1.get(0)))
    }
}

impl Lattice for Boolean {
    fn join<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        Vector::from_elem(logic.bool_or(elem0.get(0), elem1.get(0)))
    }
}

impl BooleanLattice for Boolean {
    fn complement<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        debug_assert!(elem.len() == 1);
        Vector::from_elem(logic.bool_not(elem.get(0)))
    }
}

pub struct BooleanNot();

pub const BOOLEAN_NOT: BooleanNot = BooleanNot();

impl Operation for BooleanNot {
    type Domain = Boolean;

    fn domain(&self) -> &Boolean {
        &BOOLEAN
    }

    fn arity(&self) -> usize {
        1
    }

    fn evaluate<LOGIC>(&self, logic: &mut LOGIC, args: &[LOGIC::Slice<'_>]) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].len(), 1);
        Vector::from_elem(logic.bool_not(args[0].get(0)))
    }
}

pub struct BooleanAnd();

pub const BOOLEAN_AND: BooleanAnd = BooleanAnd();

impl Operation for BooleanAnd {
    type Domain = Boolean;

    fn domain(&self) -> &Boolean {
        &BOOLEAN
    }

    fn arity(&self) -> usize {
        2
    }

    fn evaluate<LOGIC>(&self, logic: &mut LOGIC, args: &[LOGIC::Slice<'_>]) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(args.len(), 2);
        assert_eq!(args[0].len(), 1);
        assert_eq!(args[1].len(), 1);
        Vector::from_elem(logic.bool_and(args[0].get(0), args[1].get(0)))
    }
}

pub struct BooleanEqu();

pub const BOOLEAN_EQU: BooleanEqu = BooleanEqu();

impl Operation for BooleanEqu {
    type Domain = Boolean;

    fn domain(&self) -> &Boolean {
        &BOOLEAN
    }

    fn arity(&self) -> usize {
        2
    }

    fn evaluate<LOGIC>(&self, logic: &mut LOGIC, args: &[LOGIC::Slice<'_>]) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(args.len(), 2);
        assert_eq!(args[0].len(), 1);
        assert_eq!(args[1].len(), 1);
        Vector::from_elem(logic.bool_equ(args[0].get(0), args[1].get(0)))
    }
}
