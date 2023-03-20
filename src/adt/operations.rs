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

use super::{BooleanLogic, Countable, Domain, Power, RankedDomain, Slice, SmallSet, Vector};

pub trait Operations: RankedDomain {
    /// Returns the graph of this operation, which is a relation
    /// of arity one larger than this operation.
    fn graph<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM::Slice<'_>) -> ELEM
    where
        LOGIC: BooleanLogic,
        ELEM: Vector<Item = LOGIC::Elem>;
}

impl<DOM> Operations for Power<DOM, Power<DOM, SmallSet>>
where
    DOM: Countable,
{
    fn graph<'a, LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM::Slice<'a>) -> ELEM
    where
        LOGIC: BooleanLogic,
        ELEM: Vector<Item = LOGIC::Elem>,
    {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(self.base(), self.exponent().base());
        let domain = self.base();

        let size = domain.size();
        let mut power: usize = size;
        for _ in 0..self.arity() {
            power *= size;
        }

        let mut result: ELEM = Vector::with_capacity(power);

        for part in self.part_iter(elem) {
            for index in 0..size {
                let value = domain.elem(index);
                let value: ELEM = domain.lift(logic, Vector::slice(&value));
                let value2 = value.slice();
                // result.push(domain.equals(logic, value2, part));
            }
        }

        result
    }
}
