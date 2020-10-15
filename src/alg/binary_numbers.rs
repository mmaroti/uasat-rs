/*
* Copyright (C) 2020, Miklos Maroti
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

use super::{BooleanAlgebra, Domain};

/// The ring of residue classes of integers modulo a two-power number. The elements are represented
/// as vectors of boolean values backed by the underlying logic. The ring operations wrap around, 
/// the elements are ordered as a chain with unsigned values, thus `0` is the smallest element.
#[derive(Debug)]
pub struct BinaryNumbers<'a, L>
where
    L: BooleanAlgebra,
{
    length: usize,
    logic: &'a L,
}

impl<'a, L> BinaryNumbers<'a, L>
where
    L: BooleanAlgebra,
{
    pub fn new(length: usize, logic: &'a L) -> Self {
        Self { length, logic }
    }
}

impl<'a, L> Domain for BinaryNumbers<'a, L>
where
    L: BooleanAlgebra,
{
    type Elem = Vec<L::Elem>;

    type Logic = L;

    fn logic(&self) -> &Self::Logic {
        &self.logic
    }

    fn contains(&self, elem: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        if elem.len() == self.length {
            self.logic.top()
        } else {
            self.logic.bot()
        }
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.equ(a, b))
            .fold(self.logic.top(), |a, b| self.logic.meet(&a, &b))
    }
}
