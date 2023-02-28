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
    BooleanLogic, BoundedOrder, Countable, Domain, GenSlice, GenVec, Lattice, MeetSemilattice,
    PartialOrder, SliceFor, VecFor,
};

/// A small set encoded as a one-hot vector of booleans representing
/// the numbers `0..size` with the natural chain order.
#[derive(Clone)]
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

    fn contains<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        assert!(elem.len() == self.size);
        logic.bool_fold_one(elem.copy_iter())
    }

    fn equals<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        debug_assert!(elem0.len() == self.size && elem1.len() == self.size);
        let mut test = logic.bool_lift(false);
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            let v = logic.bool_and(a, b);
            test = logic.bool_or(test, v);
        }
        test
    }

    fn display_elem<ELEM>(&self, f: &mut std::fmt::Formatter<'_>, elem: ELEM) -> std::fmt::Result
    where
        ELEM: GenSlice<Item = bool>,
    {
        write!(f, "{}", self.index(elem))
    }
}

impl Countable for SmallSet {
    fn size(&self) -> usize {
        self.size
    }

    fn elem(&self, index: usize) -> VecFor<bool> {
        assert!(index < self.size);
        let mut vec: VecFor<bool> = GenVec::with_capacity(self.size);
        for i in 0..self.size {
            vec.push(i == index);
        }
        vec
    }

    fn index<ELEM>(&self, elem: ELEM) -> usize
    where
        ELEM: GenSlice<Item = bool>,
    {
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
    fn leq<ALG>(
        &self,
        alg: &mut ALG,
        elem0: SliceFor<'_, ALG::Elem>,
        elem1: SliceFor<'_, ALG::Elem>,
    ) -> ALG::Elem
    where
        ALG: BooleanLogic,
    {
        debug_assert!(elem0.len() == self.size && elem1.len() == self.size);
        alg.bool_cmp_leq(elem0.copy_iter().zip(elem1.copy_iter()))
    }
}

impl BoundedOrder for SmallSet {
    fn bottom(&self) -> VecFor<bool> {
        assert!(self.size != 0);
        self.elem(0)
    }

    fn top(&self) -> VecFor<bool> {
        assert!(self.size != 0);
        self.elem(self.size - 1)
    }
}

impl MeetSemilattice for SmallSet {
    fn meet<ALG>(
        &self,
        alg: &mut ALG,
        elem0: SliceFor<'_, ALG::Elem>,
        elem1: SliceFor<'_, ALG::Elem>,
    ) -> VecFor<ALG::Elem>
    where
        ALG: BooleanLogic,
    {
        let mut result: VecFor<ALG::Elem> = GenVec::with_capacity(self.num_bits());
        let mut looking = alg.bool_lift(true);
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            let found = alg.bool_or(a, b);
            result.push(alg.bool_and(looking, found));
            looking = alg.bool_and(looking, alg.bool_not(found));
        }
        result
    }
}

impl Lattice for SmallSet {
    fn join<ALG>(
        &self,
        alg: &mut ALG,
        elem0: SliceFor<'_, ALG::Elem>,
        elem1: SliceFor<'_, ALG::Elem>,
    ) -> VecFor<ALG::Elem>
    where
        ALG: BooleanLogic,
    {
        let mut result: VecFor<ALG::Elem> = GenVec::with_capacity(self.num_bits());
        let mut looking = alg.bool_lift(false);
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            result.push(alg.bool_maj(looking, a, b));
            looking = alg.bool_sum3(looking, a, b);
        }
        result
    }
}
