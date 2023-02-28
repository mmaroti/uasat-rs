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
    BitVec, BooleanLattice, BooleanLogic, BoundedOrder, Countable, Domain, GenSlice, GenVec,
    Lattice, MeetSemilattice, PartialOrder,
};

use std::iter::{ExactSizeIterator, Extend, FusedIterator};

/// A helper iterator to go through the parts of an element.
struct PartIter<ELEM>
where
    ELEM: GenSlice,
{
    elem: ELEM,
    step: usize,
}

impl<ELEM> Iterator for PartIter<ELEM>
where
    ELEM: GenSlice,
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

impl<ELEM> FusedIterator for PartIter<ELEM> where ELEM: GenSlice {}

impl<ELEM> ExactSizeIterator for PartIter<ELEM>
where
    ELEM: GenSlice,
{
    fn len(&self) -> usize {
        self.elem.len() / self.step
    }
}

/// The product of a list of domains.
#[derive(Clone)]
pub struct Power<BASE, EXP>
where
    BASE: Domain,
    EXP: Countable,
{
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

    /// Returns the part of an element at the given index.
    fn part_iter<ELEM>(&self, elem: ELEM) -> PartIter<ELEM>
    where
        ELEM: GenSlice,
    {
        debug_assert!(elem.len() == self.num_bits());
        PartIter {
            elem,
            step: self.base.num_bits(),
        }
    }

    /// Returns the part of an element at the given index.
    pub fn part<ELEM>(&self, elem: ELEM, index: usize) -> ELEM
    where
        ELEM: GenSlice,
    {
        debug_assert!(elem.len() == self.num_bits());
        let step = self.base().num_bits();
        let start = index * step;
        elem.slice(start, start + step)
    }
}

impl<PART, EXP> Domain for Power<PART, EXP>
where
    PART: Domain,
    EXP: Countable,
{
    fn num_bits(&self) -> usize {
        self.base.num_bits() * self.exponent.size()
    }

    fn contains<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let mut valid = logic.bool_lift(true);
        for part in self.part_iter(elem) {
            let v = self.base.contains(logic, part);
            valid = logic.bool_and(valid, v);
        }
        valid
    }

    fn equals<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let mut valid = logic.bool_lift(true);
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            let v = self.base.equals(logic, part0, part1);
            valid = logic.bool_and(valid, v);
        }
        valid
    }

    fn display_elem<ELEM>(&self, f: &mut std::fmt::Formatter<'_>, elem: ELEM) -> std::fmt::Result
    where
        ELEM: GenSlice<Item = bool>,
    {
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

    fn elem(&self, index: usize) -> BitVec {
        let mut index = index;
        let base_size = self.base.size();
        let mut result: BitVec = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            let other = self.base.elem(index % base_size);
            result.extend(other);
            index /= base_size;
        }
        assert!(index == 0 && result.len() == self.num_bits());
        result
    }

    fn index<ELEM>(&self, elem: ELEM) -> usize
    where
        ELEM: GenSlice<Item = bool>,
    {
        let mut index = 0;
        let base_size = self.base.size();
        let mut power = 1;

        for part in self.part_iter(elem) {
            index += self.base.index(part) * power;
            power *= base_size;
        }

        index
    }
}

impl<BASE, EXP> PartialOrder for Power<BASE, EXP>
where
    BASE: PartialOrder,
    EXP: Countable,
{
    fn leq<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let mut valid = logic.bool_lift(true);
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            let v = self.base.leq(logic, part0, part1);
            valid = logic.bool_and(valid, v);
        }
        valid
    }
}

impl<BASE, EXP> BoundedOrder for Power<BASE, EXP>
where
    BASE: BoundedOrder,
    EXP: Countable,
{
    fn top(&self) -> BitVec {
        let part = self.base.top();
        let mut elem: BitVec = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            elem.extend(part.copy_iter());
        }
        elem
    }

    fn bottom(&self) -> BitVec {
        let part = self.base.bottom();
        let mut elem: BitVec = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            elem.extend(part.copy_iter());
        }
        elem
    }
}

impl<BASE, EXP> MeetSemilattice for Power<BASE, EXP>
where
    BASE: MeetSemilattice,
    EXP: Countable,
{
    fn meet<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let mut elem: ELEM::Vec = GenVec::with_capacity(self.num_bits());
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
    fn join<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let mut elem: ELEM::Vec = GenVec::with_capacity(self.num_bits());
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
    fn complement<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let mut result: ELEM::Vec = GenVec::with_capacity(self.num_bits());
        for part in self.part_iter(elem) {
            result.extend(self.base.complement(logic, part));
        }
        result
    }
}
