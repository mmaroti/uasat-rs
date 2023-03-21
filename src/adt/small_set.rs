/*
* Copyright (C) 2022, Miklos Maroti
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
    BitSlice, BitVec, BooleanLogic, BoundedOrder, Countable, Domain, Lattice, MeetSemilattice,
    PartialOrder, Slice, Vector,
};

/// A small set encoded as a one-hot vector of booleans representing
/// the numbers `0..size` with the natural chain order.
#[derive(Clone, PartialEq, Debug)]
pub struct SmallSet {
    size: usize,
}

pub const ZERO: SmallSet = SmallSet::new(0);
pub const ONE: SmallSet = SmallSet::new(1);
pub const TWO: SmallSet = SmallSet::new(2);
pub const THREE: SmallSet = SmallSet::new(3);
pub const FOUR: SmallSet = SmallSet::new(4);
pub const FIVE: SmallSet = SmallSet::new(5);
pub const SIX: SmallSet = SmallSet::new(6);
pub const SEVEN: SmallSet = SmallSet::new(7);
pub const EIGHT: SmallSet = SmallSet::new(8);
pub const NINE: SmallSet = SmallSet::new(9);
pub const TEN: SmallSet = SmallSet::new(10);

impl SmallSet {
    /// Creates a new small set of the given size.
    pub const fn new(size: usize) -> Self {
        Self { size }
    }
}

impl Domain for SmallSet {
    fn num_bits(&self) -> usize {
        self.size
    }

    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), self.size);
        logic.bool_fold_one(elem.copy_iter())
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
        assert_eq!(elem0.len(), self.size);
        assert_eq!(elem1.len(), self.size);
        let mut test = logic.bool_lift(false);
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            let v = logic.bool_and(a, b);
            test = logic.bool_or(test, v);
        }
        test
    }

    fn display_elem(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        elem: BitSlice<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.index(elem))
    }
}

impl Countable for SmallSet {
    fn size(&self) -> usize {
        self.size
    }

    fn elem(&self, index: usize) -> BitVec {
        assert!(index < self.size);
        let mut vec: BitVec = Vector::with_capacity(self.size);
        for i in 0..self.size {
            vec.push(i == index);
        }
        vec
    }

    fn index(&self, elem: BitSlice<'_>) -> usize {
        assert!(elem.len() == self.size);
        let mut index = self.size;
        for (i, v) in elem.copy_iter().enumerate() {
            if v {
                debug_assert!(index == self.size);
                index = i;
            }
        }
        assert!(index < self.size);
        index
    }
}

impl PartialOrder for SmallSet {
    fn leq<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        debug_assert!(elem0.len() == self.size && elem1.len() == self.size);
        logic.bool_cmp_leq(elem0.copy_iter().zip(elem1.copy_iter()))
    }
}

impl BoundedOrder for SmallSet {
    fn bottom(&self) -> BitVec {
        assert!(self.size != 0);
        self.elem(0)
    }

    fn top(&self) -> BitVec {
        assert!(self.size != 0);
        self.elem(self.size - 1)
    }
}

impl MeetSemilattice for SmallSet {
    fn meet<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut result: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        let mut looking = logic.bool_lift(true);
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            let found = logic.bool_or(a, b);
            result.push(logic.bool_and(looking, found));
            looking = logic.bool_and(looking, logic.bool_not(found));
        }
        result
    }
}

impl Lattice for SmallSet {
    fn join<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut result: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        let mut looking = logic.bool_lift(false);
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            result.push(logic.bool_maj(looking, a, b));
            looking = logic.bool_sum3(looking, a, b);
        }
        result
    }
}
