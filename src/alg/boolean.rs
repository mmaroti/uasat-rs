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
    Base, Lattice, MeetSemilattice, PartialOrder, Slice, Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean();

pub const BOOLEAN: Boolean = Boolean();

impl Base for Boolean {
    fn num_bits(&self) -> usize {
        1
    }
}

impl<LOGIC> Domain<LOGIC> for Boolean
where
    LOGIC: BooleanLogic,
{
    fn contains(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        assert!(elem.len() == 1);
        logic.bool_unit()
    }

    fn equals(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem {
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
}

impl<LOGIC> DirectedGraph<LOGIC> for Boolean
where
    LOGIC: BooleanLogic,
{
    fn is_edge(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        logic.bool_imp(elem0.get(0), elem1.get(0))
    }
}

impl<LOGIC> PartialOrder<LOGIC> for Boolean where LOGIC: BooleanLogic {}

impl<LOGIC> BoundedOrder<LOGIC> for Boolean
where
    LOGIC: BooleanLogic,
{
    fn top(&self, logic: &LOGIC) -> LOGIC::Vector {
        Vector::from_elem(logic.bool_unit())
    }

    fn is_top(&self, _logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        assert_eq!(elem.len(), 1);
        elem.get(0)
    }

    fn bottom(&self, logic: &LOGIC) -> LOGIC::Vector {
        Vector::from_elem(logic.bool_zero())
    }

    fn is_bottom(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        assert_eq!(elem.len(), 1);
        logic.bool_not(elem.get(0))
    }
}

impl<LOGIC> MeetSemilattice<LOGIC> for Boolean
where
    LOGIC: BooleanLogic,
{
    fn meet(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        Vector::from_elem(logic.bool_and(elem0.get(0), elem1.get(0)))
    }
}

impl<LOGIC> Lattice<LOGIC> for Boolean
where
    LOGIC: BooleanLogic,
{
    fn join(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector {
        debug_assert!(elem0.len() == 1 && elem1.len() == 1);
        Vector::from_elem(logic.bool_or(elem0.get(0), elem1.get(0)))
    }
}

impl<LOGIC> BooleanLattice<LOGIC> for Boolean
where
    LOGIC: BooleanLogic,
{
    fn complement(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector {
        debug_assert!(elem.len() == 1);
        Vector::from_elem(logic.bool_not(elem.get(0)))
    }
}
