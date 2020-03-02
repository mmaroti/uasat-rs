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

use super::solver::Literal;
pub use bit_vec::BitVec;
use std::fmt::Debug;

/// Generic interface for regular and bit vectors.
pub trait GenVec
where
    Self: Default + Clone + Debug,
{
    /// The element type of the vector.
    type Elem: Copy;

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
        F: FnMut(usize) -> Self::Elem;

    /// Clears the vector, removing all values.
    fn clear(self: &mut Self);

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `value`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    fn resize(self: &mut Self, new_len: usize, value: Self::Elem);

    /// Appends an element to the back of a collection.
    fn push(self: &mut Self, value: Self::Elem);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(self: &mut Self) -> Option<Self::Elem>;

    /// Returns the element at the given index.
    fn get(self: &Self, index: usize) -> Self::Elem;

    /// Sets the element at the given index to the new value.
    fn set(self: &mut Self, index: usize, value: Self::Elem);

    /// Returns the number of elements in the vector.
    fn len(self: &Self) -> usize;

    /// Returns `true` if the vector contains no elements.
    fn is_empty(self: &Self) -> bool;

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(self: &Self) -> usize;
}

impl<ELEM> GenVec for Vec<ELEM>
where
    ELEM: Copy + Debug,
{
    type Elem = ELEM;

    #[inline]
    fn new() -> Self {
        Vec::new()
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    #[inline]
    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> Self::Elem,
    {
        (0..len).map(op).collect()
    }

    #[inline]
    fn clear(self: &mut Self) {
        Vec::clear(self);
    }

    #[inline]
    fn resize(self: &mut Self, new_len: usize, value: Self::Elem) {
        Vec::resize(self, new_len, value);
    }

    #[inline]
    fn push(self: &mut Self, value: Self::Elem) {
        Vec::push(self, value);
    }

    #[inline]
    fn pop(self: &mut Self) -> Option<Self::Elem> {
        Vec::pop(self)
    }

    #[inline]
    fn get(self: &Self, index: usize) -> Self::Elem {
        self[index]
    }

    #[inline]
    fn set(self: &mut Self, index: usize, value: Self::Elem) {
        self[index] = value;
    }

    #[inline]
    fn len(self: &Self) -> usize {
        Vec::len(self)
    }

    #[inline]
    fn is_empty(self: &Self) -> bool {
        Vec::is_empty(self)
    }

    #[inline]
    fn capacity(self: &Self) -> usize {
        Vec::capacity(self)
    }
}

impl GenVec for BitVec {
    type Elem = bool;

    #[inline]
    fn new() -> Self {
        BitVec::new()
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        BitVec::with_capacity(capacity)
    }

    #[inline]
    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> Self::Elem,
    {
        BitVec::from_fn(len, op)
    }

    #[inline]
    fn clear(self: &mut Self) {
        BitVec::clear(self);
    }

    #[inline]
    fn resize(self: &mut Self, new_len: usize, value: Self::Elem) {
        if new_len > self.len() {
            BitVec::grow(self, new_len - self.len(), value);
        } else if new_len < self.len() {
            BitVec::truncate(self, new_len);
        }
    }

    #[inline]
    fn push(self: &mut Self, value: Self::Elem) {
        BitVec::push(self, value);
    }

    #[inline]
    fn pop(self: &mut Self) -> Option<Self::Elem> {
        BitVec::pop(self)
    }

    #[inline]
    fn get(self: &Self, index: usize) -> Self::Elem {
        BitVec::get(self, index).unwrap()
    }

    #[inline]
    fn set(self: &mut Self, index: usize, value: Self::Elem) {
        BitVec::set(self, index, value);
    }

    #[inline]
    fn len(self: &Self) -> usize {
        BitVec::len(self)
    }

    #[inline]
    fn is_empty(self: &Self) -> bool {
        BitVec::is_empty(self)
    }

    #[inline]
    fn capacity(self: &Self) -> usize {
        BitVec::capacity(self)
    }
}

/// Interface for elements whose vector container can be automatically
/// derived.
pub trait GenElem: Copy {
    /// A type that can be used for storing a vector of elements.
    type Vector: GenVec<Elem = Self>;
}

impl GenElem for bool {
    type Vector = BitVec;
}

impl GenElem for usize {
    type Vector = Vec<Self>;
}

impl GenElem for Literal {
    type Vector = Vec<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: BitVec = GenVec::new();
        let mut v2: Vec<bool> = GenVec::new();

        for i in 0..50 {
            let b = i % 2 == 0;

            for _ in 0..90 {
                v1.push(b);
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
