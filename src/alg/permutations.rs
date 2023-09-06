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

use super::{BinaryRelations, BooleanLogic, Countable, Domain};

#[derive(Debug, Clone, PartialEq)]
pub struct Permutations<DOM>(BinaryRelations<DOM>)
where
    DOM: Countable;

impl<DOM> Permutations<DOM>
where
    DOM: Countable,
{
    /// Creates a class of permutations of the given domain.
    pub fn new(dom: DOM) -> Self {
        let rels = BinaryRelations::new(dom);
        Self(rels)
    }

    /// Returns the underlying domain of this class of permutations.
    pub fn domain(&self) -> &DOM {
        self.0.domain()
    }
}

impl<DOM> Domain for Permutations<DOM>
where
    DOM: Countable,
{
    fn num_bits(&self) -> usize {
        self.0.num_bits()
    }

    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_permutation(logic, elem)
    }

    fn equals<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: crate::core::BooleanLogic,
    {
        self.0.equals(logic, elem0, elem1)
    }
}
