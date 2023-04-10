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
    BitSlice, BitVec, BooleanLattice, BooleanLogic, BoundedOrder, Countable, DirectedGraph, Domain,
    Lattice, MeetSemilattice, PartialOrder, Slice, Vector,
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
        assert!(elem.len() == 1);
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

impl Countable for Boolean {
    fn size(&self) -> usize {
        2
    }

    fn elem(&self, index: usize) -> BitVec {
        assert!(index < 2);
        Vector::from_elem(index != 0)
    }

    fn index(&self, elem: BitSlice<'_>) -> usize {
        assert!(elem.len() == 1);
        elem.get(0) as usize
    }

    fn onehot<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut result: LOGIC::Vector = Vector::with_capacity(2);
        result.push(logic.bool_not(elem.get(0)));
        result.push(elem.get(0));
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
    fn top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
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

    fn bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
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
