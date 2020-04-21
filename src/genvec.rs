/*
* Copyright (C) 2019, Miklos Maroti
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

//! A generic vector trait to work with regular and bit vectors.

extern crate bit_vec;
use super::solver;
use std::{fmt, iter};

/// Generic interface for regular and bit vectors.
pub trait GenVec<ELEM>
where
    Self: Default + Clone + fmt::Debug,
    ELEM: Copy + fmt::Debug,
    Self: IntoIterator<Item = ELEM>,
    Self: iter::FromIterator<ELEM>,
{
    /// Constructs a new, empty vector. The vector will not allocate until
    /// elements are pushed onto it.
    fn new() -> Self;

    /// Constructs a new, empty vector with the specified capacity. The vector
    /// will be able to hold exactly capacity elements without reallocating.
    fn with_capacity(capacity: usize) -> Self;

    /// Constructs a new vector with the specified length where the value at
    /// each index is `op(index)`.
    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> ELEM;

    /// Constructs a new vector containing the given elements.
    fn from_iter<'a, ITER>(iter: ITER) -> Self
    where
        ITER: Iterator<Item = &'a ELEM>,
        ELEM: 'a,
    {
        let mut result = Self::with_capacity(iter.size_hint().0);
        for elem in iter {
            result.push(*elem);
        }
        result
    }

    /// Clears the vector, removing all values.
    fn clear(self: &mut Self);

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `elem`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    fn resize(self: &mut Self, new_len: usize, elem: ELEM);

    /// Appends an element to the back of a collection.
    fn push(self: &mut Self, elem: ELEM);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(self: &mut Self) -> Option<ELEM>;

    /// Extends this vector by copying all elements from the other vector.
    fn extend(self: &mut Self, other: &Self);

    /// Extends this vector by moving all elements from the other vector.
    fn append(self: &mut Self, other: &mut Self);

    /// Returns the element at the given index.
    fn get(self: &Self, index: usize) -> ELEM;

    /// Returns the element at the given index without bound checks.
    #[allow(non_snake_case)]
    unsafe fn __get_unchecked__(self: &Self, index: usize) -> ELEM {
        self.get(index)
    }

    /// Sets the element at the given index to the new value.
    fn set(self: &mut Self, index: usize, elem: ELEM);

    /// Sets the element at the given index to the new value without bound
    /// checks.
    #[allow(non_snake_case)]
    unsafe fn __set_unchecked__(self: &mut Self, index: usize, elem: ELEM) {
        self.set(index, elem);
    }

    /// Returns the number of elements in the vector.
    fn len(self: &Self) -> usize;

    /// Returns `true` if the vector contains no elements.
    fn is_empty(self: &Self) -> bool;

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(self: &Self) -> usize;
}

impl<ELEM> GenVec<ELEM> for Vec<ELEM>
where
    ELEM: Copy + fmt::Debug,
{
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> ELEM,
    {
        (0..len).map(op).collect()
    }

    fn clear(self: &mut Self) {
        Vec::clear(self);
    }

    fn resize(self: &mut Self, new_len: usize, elem: ELEM) {
        Vec::resize(self, new_len, elem);
    }

    fn push(self: &mut Self, elem: ELEM) {
        Vec::push(self, elem);
    }

    fn pop(self: &mut Self) -> Option<ELEM> {
        Vec::pop(self)
    }

    fn extend(self: &mut Self, other: &Self) {
        iter::Extend::extend(self, other.iter());
    }

    fn append(self: &mut Self, other: &mut Self) {
        Vec::append(self, other);
    }

    fn get(self: &Self, index: usize) -> ELEM {
        self[index]
    }

    unsafe fn __get_unchecked__(self: &Self, index: usize) -> ELEM {
        *self.get_unchecked(index)
    }

    fn set(self: &mut Self, index: usize, elem: ELEM) {
        self[index] = elem;
    }

    unsafe fn __set_unchecked__(self: &mut Self, index: usize, elem: ELEM) {
        *self.get_unchecked_mut(index) = elem;
    }

    fn len(self: &Self) -> usize {
        Vec::len(self)
    }

    fn is_empty(self: &Self) -> bool {
        Vec::is_empty(self)
    }

    fn capacity(self: &Self) -> usize {
        Vec::capacity(self)
    }
}

impl GenVec<bool> for bit_vec::BitVec {
    fn new() -> Self {
        bit_vec::BitVec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        bit_vec::BitVec::with_capacity(capacity)
    }

    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> bool,
    {
        bit_vec::BitVec::from_fn(len, op)
    }

    fn clear(self: &mut Self) {
        bit_vec::BitVec::clear(self);
    }

    fn resize(self: &mut Self, new_len: usize, elem: bool) {
        if new_len > self.len() {
            bit_vec::BitVec::grow(self, new_len - self.len(), elem);
        } else if new_len < self.len() {
            bit_vec::BitVec::truncate(self, new_len);
        }
    }

    fn push(self: &mut Self, elem: bool) {
        bit_vec::BitVec::push(self, elem);
    }

    fn pop(self: &mut Self) -> Option<bool> {
        bit_vec::BitVec::pop(self)
    }

    fn extend(self: &mut Self, other: &Self) {
        iter::Extend::extend(self, other.iter());
    }

    fn append(self: &mut Self, other: &mut Self) {
        bit_vec::BitVec::append(self, other);
    }

    fn get(self: &Self, index: usize) -> bool {
        bit_vec::BitVec::get(self, index).unwrap()
    }

    fn set(self: &mut Self, index: usize, elem: bool) {
        bit_vec::BitVec::set(self, index, elem);
    }

    fn len(self: &Self) -> usize {
        bit_vec::BitVec::len(self)
    }

    fn is_empty(self: &Self) -> bool {
        bit_vec::BitVec::is_empty(self)
    }

    fn capacity(self: &Self) -> usize {
        bit_vec::BitVec::capacity(self)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TrivialVec {
    len: usize,
}

pub struct TrivialIter {
    pos: usize,
}

impl Iterator for TrivialIter {
    type Item = ();

    fn next(self: &mut Self) -> Option<Self::Item> {
        if self.pos > 0 {
            self.pos -= 1;
            Some(())
        } else {
            None
        }
    }
}

impl IntoIterator for TrivialVec {
    type Item = ();
    type IntoIter = TrivialIter;

    fn into_iter(self: Self) -> Self::IntoIter {
        TrivialIter { pos: self.len }
    }
}

impl IntoIterator for &TrivialVec {
    type Item = ();
    type IntoIter = TrivialIter;

    fn into_iter(self: Self) -> Self::IntoIter {
        TrivialIter { pos: self.len }
    }
}

impl iter::FromIterator<()> for TrivialVec {
    fn from_iter<ITER>(iter: ITER) -> Self
    where
        ITER: IntoIterator<Item = ()>,
    {
        let mut len = 0;
        for _ in iter {
            len += 1;
        }
        TrivialVec { len: len }
    }
}

impl GenVec<()> for TrivialVec {
    fn new() -> Self {
        TrivialVec { len: 0 }
    }

    fn with_capacity(_capacity: usize) -> Self {
        TrivialVec { len: 0 }
    }

    fn from_fn<F>(len: usize, _op: F) -> Self
    where
        F: FnMut(usize) -> (),
    {
        TrivialVec { len: len }
    }

    fn clear(self: &mut Self) {
        self.len = 0;
    }

    fn resize(self: &mut Self, new_len: usize, _elem: ()) {
        self.len = new_len;
    }

    fn push(self: &mut Self, _elem: ()) {
        self.len += 1;
    }

    fn pop(self: &mut Self) -> Option<()> {
        if self.len > 0 {
            self.len -= 1;
            Some(())
        } else {
            None
        }
    }

    fn extend(self: &mut Self, other: &Self) {
        self.len += other.len;
    }

    fn append(self: &mut Self, other: &mut Self) {
        self.len += other.len;
        other.len = 0;
    }

    fn get(self: &Self, index: usize) -> () {
        assert!(index < self.len);
        ()
    }

    fn set(self: &mut Self, index: usize, _elem: ()) {
        assert!(index < self.len);
    }

    fn len(self: &Self) -> usize {
        self.len
    }

    fn is_empty(self: &Self) -> bool {
        self.len == 0
    }

    fn capacity(self: &Self) -> usize {
        usize::max_value()
    }
}

/// Interface for elements whose vector container can be automatically
/// derived.
pub trait GenElem: Copy + fmt::Debug {
    /// A type that can be used for storing a vector of elements.
    type Vector: GenVec<Self>;
}

impl GenElem for bool {
    type Vector = bit_vec::BitVec;
}

impl GenElem for usize {
    type Vector = Vec<Self>;
}

impl GenElem for solver::Literal {
    type Vector = Vec<Self>;
}

impl GenElem for () {
    type Vector = TrivialVec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: bit_vec::BitVec = GenVec::new();
        let mut v2: Vec<bool> = GenVec::new();
        let mut v3: TrivialVec = GenVec::new();

        for i in 0..50 {
            let b = i % 2 == 0;

            for _ in 0..90 {
                v1.push(b);
                v3.push(());
                assert_eq!(v1.len(), v3.len());
            }
            v2.resize(v2.len() + 90, b);

            assert_eq!(v1.len(), v2.len());
            for j in 0..v1.len() {
                assert_eq!(GenVec::get(&v1, j), v2.get(j));
            }
        }

        for _ in 0..50 {
            for _ in 0..77 {
                v1.pop();
            }
            v2.resize(v2.len() - 77, false);

            assert_eq!(v1.len(), v2.len());
            for j in 0..v1.len() {
                assert_eq!(GenVec::get(&v1, j), v2.get(j));
            }
        }
    }
}
