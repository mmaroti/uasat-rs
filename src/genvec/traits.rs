/*
* Copyright (C) 2019-2020, Miklos Maroti
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

use std::iter::{Extend, FromIterator};

/// A unifying interface for regular and bit vectors.
pub trait GenVec<ELEM>
where
    ELEM: Copy,
    Self: Default + Clone,
    Self: IntoIterator<Item = ELEM>,
    Self: FromIterator<ELEM>,
    Self: Extend<ELEM>,
    Self: PartialEq,
    Self: for<'a> GenIterable<'a, ELEM>,
{
    /// Constructs a new empty vector. The vector will not allocate until
    /// elements are pushed onto it.
    fn new() -> Self;

    /// Constructs a new empty vector with the specified capacity. The vector
    /// will be able to hold exactly capacity elements without reallocating.
    fn with_capacity(capacity: usize) -> Self;

    /// Concatenates the given vectors into a new one.
    fn concat(parts: Vec<Self>) -> Self {
        let len = parts.iter().map(|a| a.len()).sum();
        let mut result: Self = GenVec::with_capacity(len);
        for elem in parts.into_iter() {
            result.extend(elem.into_iter());
        }
        result
    }

    /// Splits this vector into equal sized vectors.
    /// TODO: implement more efficient specialized versions
    fn split(self, len: usize) -> Vec<Self> {
        if self.len() == 0 {
            return Vec::new();
        }
        assert_ne!(len, 0);
        let count = self.len() / len;
        let mut result: Vec<Self> = Vec::with_capacity(count);
        let mut iter = self.into_iter();
        for _ in 0..count {
            let mut vec: Self = GenVec::with_capacity(len);
            for _ in 0..len {
                vec.push(iter.next().unwrap());
            }
            result.push(vec);
        }
        result
    }

    /// Creates a vector with a single element.
    fn from_elem(elem: ELEM) -> Self {
        let mut vec: Self = GenVec::with_capacity(1);
        vec.push(elem);
        vec
    }

    /// Clears the vector, removing all values.
    fn clear(&mut self);

    /// Shortens the vector, keeping the first `new_len` many elements and
    /// dropping the rest. This method panics if the current `len` is smaller
    /// than `new_len`.
    fn truncate(&mut self, new_len: usize);

    /// Resizes the vector in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the the vector is extended by the
    /// difference, with each additional slot filled with `elem`.
    /// If `new_len` is less than `len`, then the vector is simply truncated.
    fn resize(&mut self, new_len: usize, elem: ELEM);

    /// Reserves capacity for at least additional more elements to be inserted
    /// in the given vector. The collection may reserve more space to avoid
    /// frequent reallocations.
    fn reserve(&mut self, additional: usize);

    /// Appends an element to the back of the vector.
    fn push(&mut self, elem: ELEM);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(&mut self) -> Option<ELEM>;

    /// Extends this vector by moving all elements from the other vector,
    /// leaving the other vector empty.
    fn append(&mut self, other: &mut Self);

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn get(&self, index: usize) -> ELEM;

    /// Returns the element at the given index without bound checks.
    /// # Safety
    /// Do not use this in general code, use `ranges` if possible.
    unsafe fn get_unchecked(&self, index: usize) -> ELEM {
        self.get(index)
    }

    /// Sets the element at the given index to the new value. Panics if the
    /// index is out of bounds.
    fn set(&mut self, index: usize, elem: ELEM);

    /// Sets the element at the given index to the new value without bound
    /// checks.
    /// # Safety
    /// Do not use this in general code.
    unsafe fn set_unchecked(&mut self, index: usize, elem: ELEM) {
        self.set(index, elem);
    }

    /// Returns the number of elements in the vector.
    fn len(&self) -> usize;

    /// Returns `true` if the length is zero.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(&self) -> usize;

    /// The type of iterator that can return the elements as copied values.
    type Iter<'a>: Iterator<Item = ELEM>
    where
        Self: 'a;

    /// Creates an iterator that returns the elements copied values.
    fn copy_iter(&self) -> Self::Iter<'_>;
}

/// A helper trait to find the right slice and iterator type for the vector.
pub trait GenIterable<'a, ELEM>
where
    ELEM: Copy,
{
    type Slice: GenSlice<ELEM>;

    /// Returns a slice object covering all elements of this vector.
    fn slice(&'a self) -> Self::Slice;
}

pub trait GenSlice<ELEM>
where
    Self: Sized + Copy,
    ELEM: Copy,
{
    /// The iterator type for this slice.
    type Iter: Iterator<Item = ELEM>;

    /// Returns the number of elements in the slice.
    fn len(self) -> usize;

    /// Returns `true` if the length is zero.
    fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn get(self, index: usize) -> ELEM;

    /// Returns the element at the given index without bound checks.
    /// # Safety
    /// Do not use this in general code, use `ranges` if possible.
    unsafe fn get_unchecked(self, index: usize) -> ELEM {
        self.get(index)
    }

    /// Returns a slice containing the selected range of elements.
    fn slice(self, start: usize, end: usize) -> Self;

    /// Returns an iterator over this slice.
    fn iter(self) -> Self::Iter;
}

/// A trait for elements that can be stored in a generic vector.
pub trait GenElem: Copy {
    /// A type that can be used for storing a vector of elements.
    type Vec: GenVec<Self> + std::fmt::Debug;
}

pub type VecFor<ELEM> = <ELEM as GenElem>::Vec;
pub type SliceFor<'a, ELEM> = <<ELEM as GenElem>::Vec as GenIterable<'a, ELEM>>::Slice;
