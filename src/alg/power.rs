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
    BitSlice, BitVec, BooleanLattice, BooleanLogic, BoundedOrder, Countable, Domain, Lattice,
    MeetSemilattice, PartialOrder, RankedDomain, Slice, SmallSet, Vector,
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
    pub fn part_iter<'a, ELEM>(&self, elem: ELEM) -> PartIter<'a, ELEM>
    where
        ELEM: Slice<'a>,
    {
        debug_assert!(elem.len() == self.num_bits());
        PartIter {
            elem,
            step: self.base.num_bits(),
            phantom: Default::default(),
        }
    }

    /// Returns the part of an element at the given index.
    pub fn part<'a, ELEM>(&self, elem: ELEM, index: usize) -> ELEM
    where
        ELEM: Slice<'a>,
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

    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), self.num_bits());
        let mut valid = logic.bool_lift(true);
        for part in self.part_iter(elem) {
            let v = self.base.contains(logic, part);
            valid = logic.bool_and(valid, v);
        }
        valid
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
        let mut valid = logic.bool_lift(true);
        for (part0, part1) in self.part_iter(elem0).zip(self.part_iter(elem1)) {
            let v = self.base.equals(logic, part0, part1);
            valid = logic.bool_and(valid, v);
        }
        valid
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
        let mut result: BitVec = Vector::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            let other = self.base.elem(index % base_size);
            result.extend(other);
            index /= base_size;
        }
        assert!(index == 0 && result.len() == self.num_bits());
        result
    }

    fn index(&self, elem: BitSlice<'_>) -> usize {
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
    fn leq<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
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
        let mut elem: BitVec = Vector::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            elem.extend(part.copy_iter());
        }
        elem
    }

    fn bottom(&self) -> BitVec {
        let part = self.base.bottom();
        let mut elem: BitVec = Vector::with_capacity(self.num_bits());
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

impl<DOM0, DOM1> RankedDomain for Power<DOM0, Power<DOM1, SmallSet>>
where
    DOM0: Domain,
    DOM1: Countable,
{
    fn arity(&self) -> usize {
        self.exponent().exponent().size()
    }

    fn new_arity(&self, arity: usize) -> Self {
        Power::new(
            self.base.clone(),
            Power::new(self.exponent.base.clone(), SmallSet::new(arity)),
        )
    }

    fn polymer<'a, ELEM>(&self, elem: ELEM, arity: usize, mapping: &[usize]) -> ELEM::Vec
    where
        ELEM: Slice<'a>,
    {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(mapping.len(), self.exponent().exponent().size());

        let mut strides: Vec<(usize, usize, usize)> = vec![(0, 0, 0); arity];
        let size = self.exponent().base().size();
        let mut power: usize = 1;
        for &i in mapping {
            assert!(i < arity);
            strides[i].0 += power;
            power *= size;
        }

        power = 1;
        for s in strides.iter_mut() {
            s.2 = size * s.0;
            power *= size;
        }

        let mut result: ELEM::Vec = Vector::with_capacity(self.base.num_bits() * power);
        let mut index = 0;
        'outer: loop {
            result.extend(self.part(elem, index).copy_iter());

            for stride in strides.iter_mut() {
                index += stride.0;
                stride.1 += 1;
                if stride.1 >= size {
                    stride.1 = 0;
                    index -= stride.2;
                } else {
                    continue 'outer;
                }
            }

            break;
        }

        debug_assert_eq!(result.len(), self.base.num_bits() * power);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BitVec, Domain, Logic, Vector};
    use super::*;

    #[test]
    fn polymer() {
        let dom0 = SmallSet::new(2);
        let dom1 = SmallSet::new(3);
        let op0 = Power::new(dom0.clone(), Power::new(dom1.clone(), SmallSet::new(1)));
        let op1 = Power::new(dom0.clone(), Power::new(dom1.clone(), SmallSet::new(2)));

        assert_eq!(op0.arity(), 1);
        assert_eq!(op1.arity(), 2);

        let mut logic = Logic();

        let elem1: BitVec = vec![false, true, true, false, false, true]
            .into_iter()
            .collect();
        assert!(op0.contains(&mut logic, elem1.slice()));

        let elem2: BitVec = vec![
            false, true, true, false, true, false, false, true, true, false, true, false, false,
            true, false, true, false, true,
        ]
        .into_iter()
        .collect();
        assert!(op1.contains(&mut logic, elem2.slice()));

        let elem3: BitVec = vec![
            false, true, false, true, false, true, true, false, true, false, false, true, true,
            false, true, false, false, true,
        ]
        .into_iter()
        .collect();
        assert!(op1.contains(&mut logic, elem3.slice()));

        let elem4 = op1.polymer(elem2.slice(), 2, &[1, 0]);
        assert_eq!(elem3, elem4);

        let elem5 = op1.polymer(elem2.slice(), 1, &[0, 0]);
        assert_eq!(elem1, elem5);
    }
}