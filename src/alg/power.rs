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

use super::{
    BitSlice, BooleanLattice, BooleanLogic, BoundedOrder, DirectedGraph, Domain, Indexable,
    Lattice, MeetSemilattice, Monoid, PartialOrder, Semigroup, Slice, Vector,
};

use std::iter::{ExactSizeIterator, Extend, FusedIterator};

/// A helper iterator to go through the parts of an element.
pub struct PartIter<'a, ELEM>
where
    ELEM: Slice<'a>,
{
    elem: ELEM,
    step: usize,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, ELEM> PartIter<'a, ELEM>
where
    ELEM: Slice<'a>,
{
    pub fn new(elem: ELEM, step: usize) -> Self {
        Self {
            elem,
            step,
            phantom: Default::default(),
        }
    }
}

impl<'a, ELEM> Iterator for PartIter<'a, ELEM>
where
    ELEM: Slice<'a>,
{
    type Item = ELEM;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elem.is_empty() {
            None
        } else {
            let next = self.elem.head(self.step);
            self.elem = self.elem.tail(self.step);
            Some(next)
        }
    }
}

impl<'a, ELEM> FusedIterator for PartIter<'a, ELEM> where ELEM: Slice<'a> {}

impl<'a, ELEM> ExactSizeIterator for PartIter<'a, ELEM>
where
    ELEM: Slice<'a>,
{
    fn len(&self) -> usize {
        self.elem.len() / self.step
    }
}

/// The product of a list of domains.
#[derive(Clone, PartialEq, Debug)]
pub struct Power<BASE> {
    base: BASE,
    exponent: usize,
}

impl<BASE> Power<BASE>
where
    BASE: Domain,
{
    /// Creates the product domain from the given list of domains.
    pub fn new(base: BASE, exponent: usize) -> Self {
        Self { base, exponent }
    }

    /// Returns the base domain of the power domain.
    pub fn base(&self) -> &BASE {
        &self.base
    }

    /// Returns the exponent of the power domain.
    pub fn exponent(&self) -> usize {
        self.exponent
    }

    /// Returns the part of an element at consequtive indices.
    pub fn part_iter<'a, ELEM>(&self, elem: ELEM) -> PartIter<'a, ELEM>
    where
        ELEM: Slice<'a>,
    {
        assert_eq!(elem.len(), self.num_bits());
        PartIter::new(elem, self.base().num_bits())
    }

    /// Returns the part of an element at the given index.
    pub fn part<'a, ELEM>(&self, elem: ELEM, index: usize) -> ELEM
    where
        ELEM: Slice<'a>,
    {
        assert_eq!(elem.len(), self.num_bits());
        let step = self.base().num_bits();
        let start = index * step;
        elem.range(start, start + step)
    }
}

impl<BASE> Domain for Power<BASE>
where
    BASE: Domain,
{
    fn num_bits(&self) -> usize {
        self.base.num_bits() * self.exponent
    }

    fn display_elem(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        elem: BitSlice<'_>,
    ) -> std::fmt::Result {
        let mut first = true;
        write!(f, "[")?;
        for part in self.part_iter(elem) {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }
            self.base.display_elem(f, part)?;
        }
        write!(f, "]")
    }

    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let mut result = logic.bool_unit();
        for part in self.part_iter(elem) {
            let v = self.base.contains(logic, part);
            result = logic.bool_and(result, v);
        }
        result
    }

    fn equals<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let mut result = logic.bool_unit();
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            let v = self.base.equals(logic, part0, part1);
            result = logic.bool_and(result, v);
        }
        result
    }
}

impl<BASE> Indexable for Power<BASE>
where
    BASE: Indexable,
{
    fn size(&self) -> usize {
        let mut result = 1;
        let base_size = self.base.size();
        for _ in 0..self.exponent {
            result *= base_size;
        }
        result
    }

    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut index = index;
        let base_size = self.base.size();
        let mut result: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for _ in 0..self.exponent {
            let other = self.base.get_elem(logic, index % base_size);
            result.extend(other);
            index /= base_size;
        }
        assert!(index == 0 && result.len() == self.num_bits());
        result
    }

    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        let mut index = 0;
        let base_size = self.base.size();
        let mut power = 1;

        for part in self.part_iter(elem) {
            index += self.base.get_index(part) * power;
            power *= base_size;
        }

        index
    }

    fn onehot<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut result: LOGIC::Vector = Vector::with_capacity(self.size());
        let mut temp: LOGIC::Vector = Vector::new();

        result.push(logic.bool_unit());
        for part in self.part_iter(elem) {
            temp.clear();
            temp.append(&mut result);
            debug_assert!(result.is_empty());

            let part = self.base.onehot(logic, part);
            for v1 in part.copy_iter() {
                for v0 in temp.copy_iter() {
                    result.push(logic.bool_and(v0, v1));
                }
            }
        }

        debug_assert_eq!(result.len(), self.size());
        result
    }
}

impl<BASE> DirectedGraph for Power<BASE>
where
    BASE: DirectedGraph,
{
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let mut result = logic.bool_unit();
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            let v = self.base.is_edge(logic, part0, part1);
            result = logic.bool_and(result, v);
        }
        result
    }
}

impl<BASE> PartialOrder for Power<BASE> where BASE: PartialOrder {}

impl<BASE> BoundedOrder for Power<BASE>
where
    BASE: BoundedOrder,
{
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let part = self.base.get_top(logic);
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for _ in 0..self.exponent {
            elem.extend(part.copy_iter());
        }
        elem
    }

    fn is_top<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let mut result = logic.bool_unit();
        for part in self.part_iter(elem) {
            let v = self.base.is_top(logic, part);
            result = logic.bool_and(result, v);
        }
        result
    }

    fn get_bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let part = self.base.get_bottom(logic);
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for _ in 0..self.exponent {
            elem.extend(part.copy_iter());
        }
        elem
    }

    fn is_bottom<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let mut result = logic.bool_unit();
        for part in self.part_iter(elem) {
            let v = self.base.is_bottom(logic, part);
            result = logic.bool_and(result, v);
        }
        result
    }
}

impl<BASE> MeetSemilattice for Power<BASE>
where
    BASE: MeetSemilattice,
{
    fn meet<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            elem.extend(self.base.meet(logic, part0, part1));
        }
        elem
    }
}

impl<BASE> Lattice for Power<BASE>
where
    BASE: Lattice,
{
    fn join<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            elem.extend(self.base.join(logic, part0, part1));
        }
        elem
    }
}

impl<BASE> BooleanLattice for Power<BASE>
where
    BASE: BooleanLattice,
{
    fn complement<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut result: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for part in self.part_iter(elem) {
            result.extend(self.base.complement(logic, part));
        }
        result
    }
}

impl<BASE> Semigroup for Power<BASE>
where
    BASE: Semigroup,
{
    fn product<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            elem.extend(Semigroup::product(&self.base, logic, part0, part1));
        }
        elem
    }
}

impl<BASE> Monoid for Power<BASE>
where
    BASE: Monoid,
{
    fn get_identity<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let part = self.base.get_identity(logic);
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for _ in 0..self.exponent {
            elem.extend(part.copy_iter());
        }
        elem
    }

    fn is_identity<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let mut result = logic.bool_unit();
        for part in self.part_iter(elem) {
            let v = self.base.is_identity(logic, part);
            result = logic.bool_and(result, v);
        }
        result
    }
}
