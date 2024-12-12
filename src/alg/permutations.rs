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
    BinaryRelations, BitSlice, BooleanLogic, Domain, Group, Indexable, Monoid, Semigroup, Slice,
    Vector,
};

/// The class of all permutations of the given indexable domain.
#[derive(Debug, Clone, PartialEq)]
pub struct SymmetricGroup<DOM>(BinaryRelations<DOM>)
where
    DOM: Indexable;

impl<DOM> SymmetricGroup<DOM>
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

    /// Returns true if the parity of the permutation is odd.
    pub fn is_odd_permutation<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_odd_permutation(logic, elem)
    }

    /// Returns true if the parity of the permutation is even.
    pub fn is_even_permutation<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_even_permutation(logic, elem)
    }
}

impl<DOM> Domain for SymmetricGroup<DOM>
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

impl<DOM> Indexable for SymmetricGroup<DOM>
where
    DOM: Indexable,
{
    fn size(&self) -> usize {
        let mut size = 1;
        for i in 1..self.domain().size() {
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

impl<DOM> Semigroup for SymmetricGroup<DOM>
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
        Semigroup::product(&self.0, logic, elem0, elem1)
    }
}

impl<DOM> Monoid for SymmetricGroup<DOM>
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

impl<DOM> Group for SymmetricGroup<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn inverse<LOGIC>(&self, _logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.converse(elem)
    }
}

/// The class of all even permutations of the given indexable domain.
#[derive(Debug, Clone, PartialEq)]
pub struct AlternatingGroup<DOM>(BinaryRelations<DOM>)
where
    DOM: Indexable;

impl<DOM> AlternatingGroup<DOM>
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

impl<DOM> Domain for AlternatingGroup<DOM>
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
        let test0 = self.0.is_permutation(logic, elem);
        let test1 = self.0.is_even_permutation(logic, elem);
        logic.bool_and(test0, test1)
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

impl<DOM> Indexable for AlternatingGroup<DOM>
where
    DOM: Indexable,
{
    fn size(&self) -> usize {
        let mut size = 1;
        for i in 2..self.domain().size() {
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

        let mut stride = if count < 2 { 1 } else { 2 * self.size() };
        let mut index = 2 * index;
        assert!(index < stride);
        let mut parity = false;

        let mut result: LOGIC::Vector = Vector::with_values(count * count, logic.bool_zero());
        for i in 0..count {
            if stride == 2 && parity {
                index += 1;
            }
            stride /= count - i;
            let mut r = index / stride;
            index %= stride;
            parity ^= r % 2 != 0;
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
        index / 2
    }
}

impl<DOM> Semigroup for AlternatingGroup<DOM>
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
        Semigroup::product(&self.0, logic, elem0, elem1)
    }
}

impl<DOM> Monoid for AlternatingGroup<DOM>
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

impl<DOM> Group for AlternatingGroup<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn inverse<LOGIC>(&self, _logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.converse(elem)
    }
}
