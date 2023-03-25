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
    BinaryRelations, BitSlice, Boolean, BooleanLogic, Countable, Domain, Base, Power,
    RankedDomain, Relations, Slice, SmallSet, Vector,
};

impl<DOM, LOGIC> Relations<LOGIC> for Power<Boolean, Power<DOM, SmallSet>>
where
    DOM: Domain<LOGIC> + Countable,
    LOGIC: BooleanLogic,
{
    fn get_diagonal(&self, logic: &LOGIC) -> LOGIC::Vector {
        assert!(self.arity() >= 1);

        let num_bits = self.num_bits();
        let size = self.exponent().base().size();

        let stride = if size >= 2 {
            (num_bits - 1) / (size - 1)
        } else {
            1
        };

        let mut result: LOGIC::Vector = Vector::with_values(num_bits, logic.bool_zero());

        let mut index = 0;
        for _ in 0..size {
            result.set(index, logic.bool_unit());
            index += stride;
        }
        debug_assert_eq!(index - stride, num_bits - 1);

        result
    }

    fn is_diagonal(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        let diag = self.get_diagonal(logic);
        self.equals(logic, elem, diag.slice())
    }

    fn get_singleton(&self, logic: &LOGIC, elem: &[BitSlice<'_>]) -> LOGIC::Vector {
        assert!(self.arity() == elem.len());

        let domain = self.exponent().base();
        let size = domain.size();

        let mut index = 0;
        for value in elem.iter().rev() {
            index *= size;
            index += domain.index(*value);
        }

        let mut result: LOGIC::Vector = Vector::with_values(self.num_bits(), logic.bool_zero());
        result.set(index, logic.bool_unit());
        result
    }

    fn is_singleton(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        assert!(self.arity() == 1);
        logic.bool_fold_one(elem.copy_iter())
    }

    fn fold_all(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector {
        let dom = self.change(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_all(part.copy_iter()));
        }
        result
    }

    fn fold_any(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector {
        let dom = self.change(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_any(part.copy_iter()));
        }
        result
    }
}

impl<DOM, LOGIC> BinaryRelations<LOGIC> for Power<Boolean, Power<DOM, SmallSet>>
where
    DOM: Domain<LOGIC> + Countable,
    LOGIC: BooleanLogic,
{
}
