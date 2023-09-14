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
    BitSlice, BooleanLogic, BoundedOrder, DirectedGraph, Domain, Indexable, Lattice,
    MeetSemilattice, PartialOrder, Slice, Vector,
};

/// A small set encoded as a one-hot vector of booleans representing
/// the numbers `0..size` with the natural chain order. The size of
/// the domain is specified at compile time.
#[derive(Clone, PartialEq, Debug)]
pub struct FixedSet<const SIZE: usize>;

impl<const SIZE: usize> Domain for FixedSet<SIZE> {
    fn num_bits(&self) -> usize {
        SIZE
    }

    fn display_elem(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        elem: BitSlice<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.get_index(elem))
    }

    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), SIZE);
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
        debug_assert_eq!(elem0.len(), SIZE);
        debug_assert_eq!(elem1.len(), SIZE);
        let mut test = logic.bool_zero();
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            let v = logic.bool_and(a, b);
            test = logic.bool_or(test, v);
        }
        test
    }
}

impl<const SIZE: usize> Indexable for FixedSet<SIZE> {
    fn size(&self) -> usize {
        SIZE
    }

    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(index < SIZE);
        let mut vec: LOGIC::Vector = Vector::with_values(SIZE, logic.bool_zero());
        vec.set(index, logic.bool_unit());
        vec
    }

    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        assert!(elem.len() == SIZE);
        let mut index = SIZE;
        for (i, v) in elem.copy_iter().enumerate() {
            if v {
                debug_assert_eq!(index, SIZE);
                index = i;
            }
        }
        assert!(index < SIZE);
        index
    }

    fn onehot<LOGIC>(&self, _logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        elem.copy_iter().collect()
    }
}

impl<const SIZE: usize> DirectedGraph for FixedSet<SIZE> {
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        debug_assert_eq!(elem0.len(), SIZE);
        debug_assert_eq!(elem1.len(), SIZE);
        logic.bool_cmp_leq(elem0.copy_iter().zip(elem1.copy_iter()))
    }
}

impl<const SIZE: usize> PartialOrder for FixedSet<SIZE> {}

impl<const SIZE: usize> BoundedOrder for FixedSet<SIZE> {
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(SIZE != 0);
        self.get_elem(logic, SIZE - 1)
    }

    fn is_top<LOGIC>(&self, _logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert!(SIZE != 0);
        elem.get(SIZE - 1)
    }

    fn get_bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(SIZE != 0);
        self.get_elem(logic, 0)
    }

    fn is_bottom<LOGIC>(&self, _logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert!(SIZE != 0);
        elem.get(0)
    }
}

impl<const SIZE: usize> MeetSemilattice for FixedSet<SIZE> {
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
        let mut looking = logic.bool_unit();
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            let found = logic.bool_or(a, b);
            result.push(logic.bool_and(looking, found));
            looking = logic.bool_and(looking, logic.bool_not(found));
        }
        result
    }
}

impl<const SIZE: usize> Lattice for FixedSet<SIZE> {
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
        let mut looking = logic.bool_zero();
        for (a, b) in elem0.copy_iter().zip(elem1.copy_iter()) {
            result.push(logic.bool_maj(looking, a, b));
            looking = logic.bool_sum3(looking, a, b);
        }
        result
    }
}
