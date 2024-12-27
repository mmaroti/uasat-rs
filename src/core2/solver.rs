/*
* Copyright (C) 2022-2024, Miklos Maroti
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

use std::cmp::{max, min};
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::num::NonZeroI32;

use super::{BitSlice, BitVec, Vector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bool(NonZeroI32);

pub const TRUE: Bool = Bool(NonZeroI32::new(1).unwrap());
pub const FALSE: Bool = Bool(NonZeroI32::new(-1).unwrap());

pub trait BoolIter<'a>: Iterator<Item = Bool> + FusedIterator + ExactSizeIterator + 'a {}

#[allow(dead_code)]
pub struct BoolRange<'a, VEC>
where
    VEC: BoolVec + ?Sized,
{
    vector: &'a VEC,
    start: usize,
    length: usize,
}

pub trait BoolVec: Debug {
    /// Returns a reference to the underlying logic object.
    fn logic(&self) -> &dyn BoolLogic;

    /// Clears the vector, removing all values.
    fn clear(&mut self) {
        self.truncate(0);
    }

    /// Shortens the vector, keeping the first `new_len` many elements and
    /// dropping the rest. This method panics if the current `len` is smaller
    /// than `new_len`.
    fn truncate(&mut self, new_len: usize);

    /// Resizes the vector in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the the vector is extended by the
    /// difference, with each additional slot filled with `elem`.
    /// If `new_len` is less than `len`, then the vector is simply truncated.
    fn resize(&mut self, new_len: usize, elem: Bool);

    /// Reserves capacity for at least additional more elements to be inserted
    /// in the given vector. The collection may reserve more space to avoid
    /// frequent reallocations.
    fn reserve(&mut self, additional: usize);

    /// Appends an element to the back of the vector.
    fn push(&mut self, elem: Bool);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(&mut self) -> Option<Bool>;

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn get(&self, index: usize) -> Bool;

    /// Sets the element at the given index to the new value. Panics if the
    /// index is out of bounds.
    fn set(&mut self, index: usize, elem: Bool);

    /// Returns the number of elements in the vector.
    fn len(&self) -> usize;

    /// Returns `true` if the length is zero.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(&self) -> usize;

    /// Returns an iterator over this slice.
    fn copy_iter(&self) -> Box<dyn BoolIter<'_> + '_>;

    fn iter(&self) -> BoolRange<'_, Self> {
        BoolRange {
            vector: self,
            start: 0,
            length: self.len(),
        }
    }
}

pub trait BoolLogic: Debug {
    /// Returns either the unit or zero element depending of the argument.
    fn bool_lift(&self, elem: bool) -> Bool {
        if elem {
            TRUE
        } else {
            FALSE
        }
    }

    /// Return the logical negation of the element.
    fn bool_not(&self, elem: Bool) -> Bool;

    /// Returns the logical or (lattice join) of a pair of elements.
    fn bool_or(&mut self, elem1: Bool, elem2: Bool) -> Bool;

    /// Returns the exclusive or (boolean addition) of a pair of elements.
    fn bool_xor(&mut self, elem1: Bool, elem2: Bool) -> Bool;

    /// Returns the logical and (lattice meet) of a pair of elements.
    fn bool_and(&mut self, elem1: Bool, elem2: Bool) -> Bool {
        let tmp1 = self.bool_not(elem1);
        let tmp2 = self.bool_not(elem2);
        let tmp3 = self.bool_or(tmp1, tmp2);
        self.bool_not(tmp3)
    }

    /// Returns the logical equivalence of a pair of elements.
    fn bool_equ(&mut self, elem1: Bool, elem2: Bool) -> Bool {
        let tmp = self.bool_not(elem1);
        self.bool_xor(tmp, elem2)
    }

    /// Returns the logical implication of a pair of elements.
    fn bool_imp(&mut self, elem1: Bool, elem2: Bool) -> Bool {
        let tmp = self.bool_not(elem1);
        self.bool_or(tmp, elem2)
    }

    /// Returns the boolean sum of three values.
    fn bool_sum3(&mut self, elem1: Bool, elem2: Bool, elem3: Bool) -> Bool {
        let tmp = self.bool_xor(elem1, elem2);
        self.bool_xor(tmp, elem3)
    }

    /// Returns the majority of the given values.
    fn bool_maj(&mut self, elem1: Bool, elem2: Bool, elem3: Bool) -> Bool {
        let tmp1 = self.bool_and(elem1, elem2);
        let tmp2 = self.bool_and(elem1, elem3);
        let tmp3 = self.bool_and(elem2, elem3);
        let tmp4 = self.bool_or(tmp1, tmp2);
        self.bool_or(tmp3, tmp4)
    }

    /// Computes the conjunction of the elements.
    fn bool_fold_all(&mut self, elems: &mut dyn BoolIter) -> Bool {
        let mut result = TRUE;
        for elem in elems {
            result = self.bool_and(result, elem);
        }
        result
    }

    /// Computes the disjunction of the elements.
    fn bool_fold_any(&mut self, elems: &mut dyn BoolIter) -> Bool {
        let mut result = FALSE;
        for elem in elems {
            result = self.bool_or(result, elem);
        }
        result
    }

    /// Computes the boolean sum of the elements.
    fn bool_fold_sum(&mut self, elems: &mut dyn BoolIter) -> Bool {
        let mut result = FALSE;
        for elem in elems {
            result = self.bool_xor(result, elem);
        }
        result
    }

    /// Computes the exactly one predicate over the given elements.
    fn bool_fold_one(&mut self, elems: &mut dyn BoolIter) -> Bool {
        let mut min1 = FALSE;
        let mut min2 = FALSE;
        for elem in elems {
            let tmp = self.bool_and(min1, elem);
            min2 = self.bool_or(min2, tmp);
            min1 = self.bool_or(min1, elem);
        }
        min2 = self.bool_not(min2);
        self.bool_and(min1, min2)
    }

    /// Computes the at most one predicate over the given elements.
    fn bool_fold_amo(&mut self, elems: &mut dyn BoolIter) -> Bool {
        let mut min1 = FALSE;
        let mut min2 = FALSE;
        for elem in elems {
            let tmp = self.bool_and(min1, elem);
            min2 = self.bool_or(min2, tmp);
            min1 = self.bool_or(min1, elem);
        }
        self.bool_not(min2)
    }

    /// Returns true if the two sequences are equal.
    fn bool_cmp_equ(&mut self, elems1: &mut dyn BoolIter, elems2: &mut dyn BoolIter) -> Bool {
        assert_eq!(elems1.len(), elems2.len());
        let mut result = TRUE;
        for (a, b) in elems1.zip(elems2) {
            let c = self.bool_equ(a, b);
            result = self.bool_and(result, c);
        }
        result
    }

    /// Returns true if the two sequences are not equal.
    fn bool_cmp_neq(&mut self, elems1: &mut dyn BoolIter, elems2: &mut dyn BoolIter) -> Bool {
        let result = self.bool_cmp_equ(elems1, elems2);
        self.bool_not(result)
    }

    /// Returns true if the first sequence is lexicographically smaller
    /// than or equal to the second one.
    fn bool_cmp_leq(&mut self, elems1: &mut dyn BoolIter, elems2: &mut dyn BoolIter) -> Bool {
        assert_eq!(elems1.len(), elems2.len());
        let mut result = TRUE;
        for (a, b) in elems1.zip(elems2) {
            let a = self.bool_not(a);
            result = self.bool_maj(a, b, result);
        }
        result
    }

    /// Returns true if the first sequence is lexicographically smaller
    /// than the second one.
    fn bool_cmp_ltn(&mut self, elems1: &mut dyn BoolIter, elems2: &mut dyn BoolIter) -> Bool {
        assert_eq!(elems1.len(), elems2.len());
        let mut result = FALSE;
        for (a, b) in elems1.zip(elems2) {
            let a = self.bool_not(a);
            result = self.bool_maj(a, b, result);
        }
        result
    }
}

#[derive(Debug)]
pub struct BitLogic();

pub const BITLOGIC: BitLogic = BitLogic();

impl BoolLogic for BitLogic {
    fn bool_not(&self, elem: Bool) -> Bool {
        debug_assert!(elem == TRUE || elem == FALSE);
        Bool(-elem.0)
    }

    fn bool_and(&mut self, elem1: Bool, elem2: Bool) -> Bool {
        debug_assert!(elem1 == TRUE || elem1 == FALSE);
        debug_assert!(elem2 == TRUE || elem2 == FALSE);
        Bool(min(elem1.0, elem1.0))
    }

    fn bool_or(&mut self, elem1: Bool, elem2: Bool) -> Bool {
        debug_assert!(elem1 == TRUE || elem1 == FALSE);
        debug_assert!(elem2 == TRUE || elem2 == FALSE);
        Bool(max(elem1.0, elem1.0))
    }

    fn bool_xor(&mut self, elem1: Bool, elem2: Bool) -> Bool {
        debug_assert!(elem1 == TRUE || elem1 == FALSE);
        debug_assert!(elem2 == TRUE || elem2 == FALSE);
        if elem1 != elem2 {
            TRUE
        } else {
            FALSE
        }
    }
}

impl BoolVec for BitVec {
    fn logic(&self) -> &dyn BoolLogic {
        &BITLOGIC
    }

    fn truncate(&mut self, new_len: usize) {
        Vector::truncate(self, new_len)
    }

    fn resize(&mut self, new_len: usize, elem: Bool) {
        debug_assert!(elem == TRUE || elem == FALSE);
        Vector::resize(self, new_len, elem == TRUE)
    }

    fn reserve(&mut self, additional: usize) {
        Vector::reserve(self, additional);
    }

    fn push(&mut self, elem: Bool) {
        debug_assert!(elem == TRUE || elem == FALSE);
        Vector::push(self, elem == TRUE)
    }

    fn pop(&mut self) -> Option<Bool> {
        Vector::pop(self).map(|b| BITLOGIC.bool_lift(b))
    }

    fn get(&self, index: usize) -> Bool {
        BITLOGIC.bool_lift(Vector::get(self, index))
    }

    fn set(&mut self, index: usize, elem: Bool) {
        Vector::set(self, index, elem == TRUE)
    }

    fn len(&self) -> usize {
        Vector::len(self)
    }

    fn capacity(&self) -> usize {
        Vector::capacity(self)
    }

    fn copy_iter(&self) -> Box<dyn BoolIter<'_> + '_> {
        let iter = Vector::copy_iter(self);
        Box::new(BitIt(iter))
    }
}

#[allow(dead_code)]
struct BitIt<'a>(BitSlice<'a>);

// pub trait BoolIter: Iterator<Item = Bool> + FusedIterator + ExactSizeIterator {}

impl Iterator for BitIt<'_> {
    type Item = Bool;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl FusedIterator for BitIt<'_> {}

impl ExactSizeIterator for BitIt<'_> {}

impl<'a> BoolIter<'a> for BitIt<'a> {}

pub struct CaDiCaL {
    pub solver: cadical::Solver,
    pub num_vars: u32,
}

pub fn test(_solver: &dyn BoolLogic) {}
