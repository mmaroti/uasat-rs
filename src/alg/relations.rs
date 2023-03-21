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
    BitVec, Boolean, BooleanLattice, Countable, Domain, Power, RankedDomain, SmallSet, Vector,
};

pub trait Relations: BooleanLattice + RankedDomain {
    /// Returns the relation that is true if and only if all arguments are
    /// the same. This method panics if the arity is zero.
    fn get_diagonal(&self) -> BitVec;
}

impl<DOM> Relations for Power<Boolean, Power<DOM, SmallSet>>
where
    DOM: Countable,
{
    fn get_diagonal(&self) -> BitVec {
        assert!(self.arity() >= 1);

        let num_bits = self.num_bits();
        let size = self.exponent().size();
        let mut result: BitVec = Vector::with_capacity(num_bits);

        let stride = if size >= 2 {
            (num_bits - 1) / (size - 1)
        } else {
            1
        };

        for _ in 1..size {
            result.push(true);
            for _ in 1..stride {
                result.push(false);
            }
        }
        result.push(true);

        debug_assert_eq!(result.len(), num_bits);
        result
    }
}
