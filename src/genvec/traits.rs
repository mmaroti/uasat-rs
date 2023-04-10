/*
* Copyright (C) 2019-2023, Miklos Maroti
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

use std::iter::{Extend, FromIterator, FusedIterator};

/// A unifying interface for regular and bit vectors.
pub trait Vector
where
    Self: Default + Clone,
    Self: IntoIterator,
    Self: FromIterator<Self::Item>,
    Self: Extend<Self::Item>,
{
    /// Constructs a new empty vector. The vector will not allocate until
    /// elements are pushed onto it.
    fn new() -> Self;

    /// Constructs a new empty vector with the specified capacity. The vector
    /// will be able to hold exactly capacity elements without reallocating.
    fn with_capacity(capacity: usize) -> Self;

    /// Creates a new vector of the given length filled with the specified
    /// element.
    fn with_values(len: usize, elem: Self::Item) -> Self;

    /// Concatenates the given vectors into a new one.
    fn concat(parts: Vec<Self>) -> Self {
        let len = parts.iter().map(|a| a.len()).sum();
        let mut result: Self = Vector::with_capacity(len);
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
            let mut vec: Self = Vector::with_capacity(len);
            for _ in 0..len {
                vec.push(iter.next().unwrap());
            }
            result.push(vec);
        }
        result
    }

    /// Creates a vector with a single element.
    fn from_elem(elem: Self::Item) -> Self {
        let mut vec: Self = Vector::with_capacity(1);
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
    fn resize(&mut self, new_len: usize, elem: Self::Item);

    /// Reserves capacity for at least additional more elements to be inserted
    /// in the given vector. The collection may reserve more space to avoid
    /// frequent reallocations.
    fn reserve(&mut self, additional: usize);

    /// Appends an element to the back of the vector.
    fn push(&mut self, elem: Self::Item);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(&mut self) -> Option<Self::Item>;

    /// Extends this vector by moving all elements from the other vector,
    /// leaving the other vector empty.
    fn append(&mut self, other: &mut Self);

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn get(&self, index: usize) -> Self::Item;

    /// Returns the element at the given index without bound checks.
    /// # Safety
    /// Do not use this in general code, use `ranges` if possible.
    unsafe fn get_unchecked(&self, index: usize) -> Self::Item {
        self.get(index)
    }

    /// Sets the element at the given index to the new value. Panics if the
    /// index is out of bounds.
    fn set(&mut self, index: usize, elem: Self::Item);

    /// Sets the element at the given index to the new value without bound
    /// checks.
    /// # Safety
    /// Do not use this in general code.
    unsafe fn set_unchecked(&mut self, index: usize, elem: Self::Item) {
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

    /// The type of slice structure that can be further sliced.
    type Slice<'a>: Slice<'a, Item = Self::Item, Vector = Self>
    where
        Self: 'a;

    /// Returns a slice object covering all elements of this vector.
    fn slice(&self) -> Self::Slice<'_>;

    /// Creates an iterator that returns the elements copied values.
    fn copy_iter(&self) -> <Self::Slice<'_> as Slice>::Iter {
        self.slice().copy_iter()
    }
}

pub trait Slice<'a>
where
    Self: Sized + Copy,
{
    /// The item type for this slice.
    type Item: Copy;

    /// The iterator type for this slice.
    type Iter: Iterator<Item = Self::Item> + FusedIterator + ExactSizeIterator + DoubleEndedIterator;

    /// A type of vector than can hold elements.
    type Vector: Vector<Item = Self::Item>;

    /// Returns the number of elements in the slice.
    fn len(self) -> usize;

    /// Returns `true` if the length is zero.
    fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn get(self, index: usize) -> Self::Item;

    /// Returns the element at the given index without bound checks.
    /// # Safety
    /// Do not use this in general code, use `ranges` if possible.
    unsafe fn get_unchecked(self, index: usize) -> Self::Item {
        self.get(index)
    }

    /// Returns an iterator over this slice.
    fn copy_iter(self) -> Self::Iter;

    /// Returns a slice containing the selected range of elements.
    fn range(self, start: usize, end: usize) -> Self;

    /// Returns a head slice of elements.
    fn head(self, end: usize) -> Self {
        self.range(0, end)
    }

    /// Returns a tail slice of elements.
    fn tail(self, start: usize) -> Self {
        self.range(start, self.len())
    }
}
