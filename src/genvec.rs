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
pub trait GenVec
where
    Self: Default + Clone + fmt::Debug,
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

    /// Constructs a new vector containing the given elements.
    fn from_elems(elems: &[Self::Elem]) -> Self {
        let mut result = Self::with_capacity(elems.len());
        for elem in elems {
            result.push(*elem);
        }
        result
    }

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

    /// Extends this vector by copying all elements from the other vector.
    fn extend(self: &mut Self, other: &Self);

    /// Extends this vector by moving all elements from the other vector.
    fn append(self: &mut Self, other: &mut Self);

    /// Returns the element at the given index.
    fn get(self: &Self, index: usize) -> Self::Elem;

    /// Returns the element at the given index without bound checks.
    #[allow(non_snake_case)]
    unsafe fn __get_unchecked__(self: &Self, index: usize) -> Self::Elem {
        self.get(index)
    }

    /// Sets the element at the given index to the new value.
    fn set(self: &mut Self, index: usize, value: Self::Elem);

    /// Sets the element at the given index to the new value without bound
    /// checks.
    #[allow(non_snake_case)]
    unsafe fn __set_unchecked__(self: &mut Self, index: usize, value: Self::Elem) {
        self.set(index, value);
    }

    /// Returns the number of elements in the vector.
    fn len(self: &Self) -> usize;

    /// Returns `true` if the vector contains no elements.
    fn is_empty(self: &Self) -> bool;

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(self: &Self) -> usize;
}

impl<ELEM> GenVec for Vec<ELEM>
where
    ELEM: Copy + fmt::Debug,
{
    type Elem = ELEM;

    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> Self::Elem,
    {
        (0..len).map(op).collect()
    }

    fn clear(self: &mut Self) {
        Vec::clear(self);
    }

    fn resize(self: &mut Self, new_len: usize, value: Self::Elem) {
        Vec::resize(self, new_len, value);
    }

    fn push(self: &mut Self, value: Self::Elem) {
        Vec::push(self, value);
    }

    fn pop(self: &mut Self) -> Option<Self::Elem> {
        Vec::pop(self)
    }

    fn extend(self: &mut Self, other: &Self) {
        iter::Extend::extend(self, other.iter());
    }

    fn append(self: &mut Self, other: &mut Self) {
        Vec::append(self, other);
    }

    fn get(self: &Self, index: usize) -> Self::Elem {
        self[index]
    }

    unsafe fn __get_unchecked__(self: &Self, index: usize) -> Self::Elem {
        *self.get_unchecked(index)
    }

    fn set(self: &mut Self, index: usize, value: Self::Elem) {
        self[index] = value;
    }

    unsafe fn __set_unchecked__(self: &mut Self, index: usize, value: Self::Elem) {
        *self.get_unchecked_mut(index) = value;
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

impl GenVec for bit_vec::BitVec {
    type Elem = bool;

    fn new() -> Self {
        bit_vec::BitVec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        bit_vec::BitVec::with_capacity(capacity)
    }

    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> Self::Elem,
    {
        bit_vec::BitVec::from_fn(len, op)
    }

    fn clear(self: &mut Self) {
        bit_vec::BitVec::clear(self);
    }

    fn resize(self: &mut Self, new_len: usize, value: Self::Elem) {
        if new_len > self.len() {
            bit_vec::BitVec::grow(self, new_len - self.len(), value);
        } else if new_len < self.len() {
            bit_vec::BitVec::truncate(self, new_len);
        }
    }

    fn push(self: &mut Self, value: Self::Elem) {
        bit_vec::BitVec::push(self, value);
    }

    fn pop(self: &mut Self) -> Option<Self::Elem> {
        bit_vec::BitVec::pop(self)
    }

    fn extend(self: &mut Self, other: &Self) {
        iter::Extend::extend(self, other.iter());
    }

    fn append(self: &mut Self, other: &mut Self) {
        bit_vec::BitVec::append(self, other);
    }

    fn get(self: &Self, index: usize) -> Self::Elem {
        bit_vec::BitVec::get(self, index).unwrap()
    }

    fn set(self: &mut Self, index: usize, value: Self::Elem) {
        bit_vec::BitVec::set(self, index, value);
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

impl GenVec for usize {
    type Elem = ();

    fn new() -> Self {
        0
    }

    fn with_capacity(capacity: usize) -> Self {
        capacity
    }

    fn from_fn<F>(len: usize, _op: F) -> Self
    where
        F: FnMut(usize) -> Self::Elem,
    {
        len
    }

    fn clear(self: &mut Self) {
        *self = 0;
    }

    fn resize(self: &mut Self, new_len: usize, _value: Self::Elem) {
        *self = new_len;
    }

    fn push(self: &mut Self, _value: Self::Elem) {
        *self += 1;
    }

    fn pop(self: &mut Self) -> Option<Self::Elem> {
        if *self > 0 {
            *self -= 1;
            Some(())
        } else {
            None
        }
    }

    fn extend(self: &mut Self, other: &Self) {
        *self += *other;
    }

    fn append(self: &mut Self, other: &mut Self) {
        *self += *other;
        *other = 0;
    }

    fn get(self: &Self, index: usize) -> Self::Elem {
        assert!(index < *self);
        ()
    }

    fn set(self: &mut Self, index: usize, _value: Self::Elem) {
        assert!(index < *self);
    }

    fn len(self: &Self) -> usize {
        *self
    }

    fn is_empty(self: &Self) -> bool {
        *self == 0
    }

    fn capacity(self: &Self) -> usize {
        usize::max_value()
    }
}

/// Interface for elements whose vector container can be automatically
/// derived.
pub trait GenElem: Copy {
    /// A type that can be used for storing a vector of elements.
    type Vector: GenVec<Elem = Self>;
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
    type Vector = usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: bit_vec::BitVec = GenVec::new();
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
