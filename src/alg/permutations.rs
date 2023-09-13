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
    BinaryRelations, BitSlice, BooleanLogic, Domain, Indexable, Monoid, Semigroup, Slice, Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Permutations<DOM>(BinaryRelations<DOM>)
where
    DOM: Indexable;

impl<DOM> Permutations<DOM>
where
    DOM: Indexable,
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
    DOM: Indexable,
{
    #[inline]
    fn num_bits(&self) -> usize {
        self.0.num_bits()
    }

    #[inline]
    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_permutation(logic, elem)
    }

    #[inline]
    fn equals<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.equals(logic, elem0, elem1)
    }
}

impl<DOM> Indexable for Permutations<DOM>
where
    DOM: Indexable,
{
    fn size(&self) -> usize {
        let mut size = 1;
        for i in 0..self.domain().size() {
            size *= i + 1;
        }
        size
    }

    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let count = self.domain().size();
        let mut used = vec![false; count];

        let mut stride = self.size();
        assert!(index < stride);
        let mut index = index;

        let mut result: LOGIC::Vector = Vector::with_values(count * count, logic.bool_zero());
        for i in 0..count {
            stride /= count - i;
            let mut r = index / stride;
            index %= stride;
            for (j, u) in used.iter_mut().enumerate() {
                if !*u {
                    if r == 0 {
                        *u = true;
                        result.set(i * count + j, logic.bool_unit());
                        break;
                    }
                    r -= 1;
                }
            }
        }
        result
    }

    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        let count = self.domain().size();
        assert_eq!(elem.len(), count * count);
        let mut used = vec![false; count];

        let mut index = 0;
        for i in 0..count {
            index *= count - i;
            let mut r = 0;
            for (j, u) in used.iter_mut().enumerate() {
                if !*u {
                    if elem.get(i * count + j) {
                        *u = true;
                        assert!(r < count - i);
                        index += r;
                        break;
                    }
                    r += 1;
                }
            }
        }
        index
    }
}

impl<DOM> Semigroup for Permutations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn product<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.product(logic, elem0, elem1)
    }
}

impl<DOM> Monoid for Permutations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn get_identity<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.get_identity(logic)
    }

    #[inline]
    fn is_identity<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_identity(logic, elem)
    }
}
