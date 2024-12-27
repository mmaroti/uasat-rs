/*
* Copyright (C) 2022-2024, Miklos Maroti
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
    BitSlice, BitVec, BooleanLogic, Domain, Function, Indexable, LitSlice, LitVec, Slice, Vector,
    LOGIC,
};

#[derive(Debug)]
pub struct Boolean();

pub const BOOLEAN: Boolean = Boolean();

impl Domain for Boolean {
    fn num_bits(&self) -> usize {
        1
    }
}

impl Indexable for Boolean {
    fn size(&self) -> usize {
        2
    }

    fn get_elem(&self, index: usize) -> BitVec {
        assert!(index < 2);
        let value = LOGIC.bool_lift(index != 0);
        BitVec::from_elem(value)
    }

    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        assert!(elem.len() == 1);
        elem.get(0) as usize
    }
}

#[derive(Debug)]
pub struct BooleanNot();

pub const BOOLEAN_NOT: BooleanNot = BooleanNot();

impl Function for BooleanNot {
    fn arity(&self) -> usize {
        1
    }

    fn domains(&self) -> &[&dyn Domain] {
        &[&BOOLEAN as &dyn Domain]
    }

    fn codomain(&self) -> &dyn Domain {
        &BOOLEAN as &dyn Domain
    }

    fn evaluate1(&self, elems: &[BitSlice<'_>]) -> BitVec {
        assert_eq!(elems.len(), 1);
        assert_eq!(elems[0].len(), 1);
        BitVec::from_elem(LOGIC.bool_not(elems[0].get(0)))
    }

    fn evaluate2(&self, logic: &mut crate::core::Solver, elems: &[LitSlice<'_>]) -> LitVec {
        assert_eq!(elems.len(), 1);
        assert_eq!(elems[0].len(), 1);
        Vec::from_elem(logic.bool_not(Slice::get(elems[0], 0)))
    }
}
