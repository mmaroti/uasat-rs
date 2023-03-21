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
    BitVec, BooleanLattice, BooleanLogic, BoundedOrder, Countable, Domain, Lattice,
    MeetSemilattice, PartialOrder, Slice, Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean();

pub const BOOLEAN: Boolean = Boolean();

impl Domain for Boolean {
    fn num_bits(&self) -> usize {
        1
    }

    fn contains<'a, LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: Slice<'a, Item = LOGIC::Elem>,
    {
        assert!(elem.len() == 1);
        logic.bool_lift(true)
    }

    fn equals<'a, 'b, LOGIC, ELEM0, ELEM1>(
        &self,
        logic: &mut LOGIC,
        elem0: ELEM0,
        elem1: ELEM1,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM0: Slice<'a, Item = LOGIC::Elem>,
        ELEM1: Slice<'b, Item = LOGIC::Elem>,
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
        let mut elem: BitVec = Vector::with_capacity(1);
        elem.push(index != 0);
        elem
    }

    fn index<'a, ELEM>(&self, elem: ELEM) -> usize
    where
        ELEM: Slice<'a, Item = bool>,
    {
        assert!(elem.len() == 1);
        elem.get(0) as usize
    }
}

impl PartialOrder for Boolean {
    fn leq<'a, LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: Slice<'a, Item = LOGIC::Elem>,
    {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        logic.bool_imp(elem0.get(0), elem1.get(0))
    }
}

impl BoundedOrder for Boolean {
    fn top(&self) -> BitVec {
        let mut elem: BitVec = Vector::with_capacity(1);
        elem.push(true);
        elem
    }

    fn bottom(&self) -> BitVec {
        let mut elem: BitVec = Vector::with_capacity(1);
        elem.push(false);
        elem
    }
}

impl MeetSemilattice for Boolean {
    fn meet<'a, LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: Slice<'a, Item = LOGIC::Elem>,
    {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        let mut elem: ELEM::Vec = Vector::with_capacity(1);
        elem.push(logic.bool_and(elem0.get(0), elem1.get(0)));
        elem
    }
}

impl Lattice for Boolean {
    fn join<'a, LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: Slice<'a, Item = LOGIC::Elem>,
    {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        let mut elem: ELEM::Vec = Vector::with_capacity(1);
        elem.push(logic.bool_or(elem0.get(0), elem1.get(0)));
        elem
    }
}

impl BooleanLattice for Boolean {
    fn complement<'a, LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: Slice<'a, Item = LOGIC::Elem>,
    {
        debug_assert!(elem.len() == 1);
        let mut elem: ELEM::Vec = Vector::with_capacity(1);
        elem.push(logic.bool_not(elem.get(0)));
        elem
    }
}
