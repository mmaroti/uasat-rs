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

use super::{BooleanAlgebra, Countable, Domain, GenSlice, GenVec, SliceFor, VecFor};

#[derive(Debug, Clone)]
pub struct Boolean();

pub const BOOLEAN: Boolean = Boolean();

impl Domain for Boolean {
    fn num_bits(&self) -> usize {
        1
    }

    fn contains<ALG>(&self, alg: &mut ALG, elem: SliceFor<'_, ALG::Elem>) -> ALG::Elem
    where
        ALG: BooleanAlgebra,
    {
        assert!(elem.len() == 1);
        alg.bool_lift(true)
    }
}

impl Countable for Boolean {
    fn size(&self) -> usize {
        2
    }

    fn elem(&self, index: usize) -> VecFor<bool> {
        assert!(index < 2);
        let mut vec: VecFor<bool> = GenVec::with_capacity(1);
        vec.push(index != 0);
        vec
    }

    fn index(&self, elem: SliceFor<'_, bool>) -> usize {
        assert!(elem.len() == 1);
        elem.get(0) as usize
    }
}
