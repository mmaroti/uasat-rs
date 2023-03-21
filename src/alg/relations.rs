/*
* Copyright (C) 2022-2023, Miklos Maroti
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
    BitSlice, BitVec, Boolean, BooleanLattice, BooleanLogic, Countable, Domain, Power,
    RankedDomain, Slice, SmallSet, Vector,
};

pub trait Relations: BooleanLattice + RankedDomain {
    /// Returns the relation that is true if and only if all arguments are
    /// the same. This method panics if the arity is zero.
    fn get_diagonal(&self) -> BitVec;

    /// Checks if the given element is the diagonal relation.
    fn is_diagonal<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let diag = self.lift(logic, self.get_diagonal().slice());
        self.equals(logic, elem, diag.slice())
    }

    /// Returns a unary relation containing the given element only. This
    /// method panics if the arity of this ranked domain is not one.
    fn get_singleton(&self, elem: BitSlice<'_>) -> BitVec;

    /// Checks if the given element is a singleton.
    fn is_singleton<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert!(self.arity() == 1);
        logic.bool_fold_one(elem.copy_iter())
    }
}

impl<DOM> Relations for Power<Boolean, Power<DOM, SmallSet>>
where
    DOM: Countable,
{
    fn get_diagonal(&self) -> BitVec {
        assert!(self.arity() >= 1);

        let num_bits = self.num_bits();
        let size = self.exponent().size();

        let stride = if size >= 2 {
            (num_bits - 1) / (size - 1)
        } else {
            1
        };

        let mut result: BitVec = Vector::with_values(num_bits, false);

        let mut index = 0;
        for _ in 0..size {
            result.set(index, true);
            index += stride;
        }
        debug_assert_eq!(index - stride, num_bits - 1);

        result
    }

    fn get_singleton(&self, elem: BitSlice<'_>) -> BitVec {
        assert!(self.arity() == 1);

        let num_bits = self.num_bits();
        let mut result: BitVec = Vector::with_values(num_bits, false);

        let index = self.exponent().base().index(elem);
        result.set(index, true);

        result
    }
}
