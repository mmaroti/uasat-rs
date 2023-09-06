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
    BitSlice, BooleanLattice, BooleanLogic, BoundedOrder, Countable, DirectedGraph, Domain,
    Lattice, MeetSemilattice, PartialOrder, Slice, Vector,
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
pub struct Power<BASE, EXP> {
    base: BASE,
    exponent: EXP,
}

impl<BASE, EXP> Power<BASE, EXP>
where
    BASE: Domain,
    EXP: Countable,
{
    /// Creates the product domain from the given list of domains.
    pub fn new(base: BASE, exponent: EXP) -> Self {
        Self { base, exponent }
    }

    /// Returns the base domain of the power domain.
    pub fn base(&self) -> &BASE {
        &self.base
    }

    /// Returns the base domain of the power domain.
    pub fn exponent(&self) -> &EXP {
        &self.exponent
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

impl<BASE, EXP> Domain for Power<BASE, EXP>
where
    BASE: Domain,
    EXP: Countable,
{
    fn num_bits(&self) -> usize {
        self.base.num_bits() * self.exponent.size()
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

impl<BASE, EXP> Countable for Power<BASE, EXP>
where
    BASE: Countable,
    EXP: Countable,
{
    fn size(&self) -> usize {
        let mut result = 1;
        let base_size = self.base.size();
        for _ in 0..self.exponent.size() {
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
        for _ in 0..self.exponent.size() {
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

impl<BASE, EXP> DirectedGraph for Power<BASE, EXP>
where
    BASE: DirectedGraph,
    EXP: Countable,
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

impl<BASE, EXP> PartialOrder for Power<BASE, EXP>
where
    BASE: PartialOrder,
    EXP: Countable,
{
}

impl<BASE, EXP> BoundedOrder for Power<BASE, EXP>
where
    BASE: BoundedOrder,
    EXP: Countable,
{
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let part = self.base.get_top(logic);
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
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
        for _ in 0..self.exponent.size() {
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

impl<BASE, EXP> MeetSemilattice for Power<BASE, EXP>
where
    BASE: MeetSemilattice,
    EXP: Countable,
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

impl<BASE, EXP> Lattice for Power<BASE, EXP>
where
    BASE: Lattice,
    EXP: Countable,
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

impl<BASE, EXP> BooleanLattice for Power<BASE, EXP>
where
    BASE: BooleanLattice,
    EXP: Countable,
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
