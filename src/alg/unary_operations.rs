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
    BinaryRelations, BitSlice, BooleanLogic, Indexable, Domain, Monoid, Power, Semigroup, SmallSet,
};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperations<DOM>(BinaryRelations<DOM>)
where
    DOM: Indexable;

impl<DOM> UnaryOperations<DOM>
where
    DOM: Indexable,
{
    /// Creates domain of binary relations over the given domain.
    #[inline]
    pub fn new(dom: DOM) -> Self {
        Self(BinaryRelations::new(dom))
    }

    /// Returns the underlying domain of this class of relations.
    #[inline]
    pub fn domain(&self) -> &DOM {
        self.0.domain()
    }

    /// Returns true if the given element is a permutation (unary and surjective).
    pub fn is_permutation<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_permutation(logic, elem)
    }
}

impl<DOM> Domain for UnaryOperations<DOM>
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
        self.0.is_operation(logic, elem)
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

impl<DOM> Indexable for UnaryOperations<DOM>
where
    DOM: Indexable,
{
    fn size(&self) -> usize {
        let mut power = 1;
        let size = self.domain().size();
        for _ in 0..size {
            power *= size;
        }
        power
    }

    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let size = self.domain().size();
        let dom = Power::new(SmallSet::new(size), SmallSet::new(size));
        dom.get_elem(logic, index)
    }

    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        let size = self.domain().size();
        let dom = Power::new(SmallSet::new(size), SmallSet::new(size));
        dom.get_index(elem)
    }
}

impl<DOM> Semigroup for UnaryOperations<DOM>
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

impl<DOM> Monoid for UnaryOperations<DOM>
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
