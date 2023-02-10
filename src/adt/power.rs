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
    BooleanAlgebra, BoundedOrder, Countable, Domain, GenSlice, GenVec, PartialOrder, SliceFor,
    VecFor,
};

use std::iter::{ExactSizeIterator, Extend, FusedIterator};

/// A helper iterator to go through the parts of an element.
struct PartIter<SLICE, ELEM>
where
    SLICE: GenSlice<ELEM>,
    ELEM: Copy,
{
    slice: SLICE,
    step: usize,
    phantom: std::marker::PhantomData<ELEM>,
}

impl<SLICE, ELEM> Iterator for PartIter<SLICE, ELEM>
where
    SLICE: GenSlice<ELEM>,
    ELEM: Copy,
{
    type Item = SLICE;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let next = self.slice.head(self.step);
            self.slice = self.slice.tail(self.step);
            Some(next)
        }
    }
}

impl<SLICE, ELEM> FusedIterator for PartIter<SLICE, ELEM>
where
    SLICE: GenSlice<ELEM>,
    ELEM: Copy,
{
}

impl<SLICE, ELEM> ExactSizeIterator for PartIter<SLICE, ELEM>
where
    SLICE: GenSlice<ELEM>,
    ELEM: Copy,
{
    fn len(&self) -> usize {
        self.slice.len() / self.step
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
    fn part_iter<SLICE, ELEM>(&self, elem: SLICE) -> PartIter<SLICE, ELEM>
    where
        SLICE: GenSlice<ELEM>,
        ELEM: Copy,
    {
        debug_assert!(elem.len() == self.num_bits());
        PartIter {
            slice: elem,
            step: self.base.num_bits(),
            phantom: Default::default(),
        }
    }

    /// Returns the part of an element at the given index.
    pub fn part<SLICE, ELEM>(&self, elem: SLICE, index: usize) -> SLICE
    where
        SLICE: GenSlice<ELEM>,
        ELEM: Copy,
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

    fn contains<ALG>(&self, alg: &mut ALG, elem: SliceFor<'_, ALG::Elem>) -> ALG::Elem
    where
        ALG: BooleanAlgebra,
    {
        let mut valid = alg.bool_lift(true);
        for part in self.part_iter(elem) {
            let v = self.base.contains(alg, part);
            valid = alg.bool_and(valid, v);
        }
        valid
    }

    fn display_elem<'a>(
        &self,
        f: &mut std::fmt::Formatter<'a>,
        elem: SliceFor<'_, bool>,
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

    fn elem(&self, index: usize) -> VecFor<bool> {
        let mut index = index;
        let base_size = self.base.size();
        let mut result: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            let other = self.base.elem(index % base_size);
            result.extend(other);
            index /= base_size;
        }
        assert!(index == 0 && result.len() == self.num_bits());
        result
    }

    fn index(&self, elem: SliceFor<'_, bool>) -> usize {
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
    fn leq<ALG>(
        &self,
        alg: &mut ALG,
        elem0: SliceFor<'_, ALG::Elem>,
        elem1: SliceFor<'_, ALG::Elem>,
    ) -> ALG::Elem
    where
        ALG: BooleanAlgebra,
    {
        let mut valid = alg.bool_lift(true);
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            let v = self.base.leq(alg, part0, part1);
            valid = alg.bool_and(valid, v);
        }
        valid
    }
}

impl<BASE, EXP> BoundedOrder for Power<BASE, EXP>
where
    BASE: BoundedOrder,
    EXP: Countable,
{
    fn top(&self) -> VecFor<bool> {
        let part = self.base.top();
        let mut elem: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            elem.extend(part.copy_iter());
        }
        elem
    }

    fn bottom(&self) -> VecFor<bool> {
        let part = self.base.bottom();
        let mut elem: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            elem.extend(part.copy_iter());
        }
        elem
    }
}

#[cfg(test)]
mod tests {
    use super::super::SmallSet;
    use super::*;
    use crate::core::{BooleanSolver, Bools, Solver};

    #[test]
    fn size() {
        let domain = Power::new(SmallSet::new(5), SmallSet::new(2));

        let mut solver = Solver::new("");
        let elem = domain.add_variable(&mut solver);
        let test = domain.contains(&mut solver, &elem);
        solver.bool_add_clause(&[test]);

        let num = solver.bool_find_num_models_method1(elem.iter().copied());
        assert_eq!(num, 25);
    }

    #[test]
    fn index() {
        let mut alg = Bools();
        let domain = Power::new(SmallSet::new(5), SmallSet::new(2));
        assert!(domain.size() == 25);

        for idx in 0..domain.size() {
            let elem = domain.elem(idx);
            assert!(domain.contains(&mut alg, elem.slice()));
            assert!(domain.index(elem.slice()) == idx);
        }
    }
}
