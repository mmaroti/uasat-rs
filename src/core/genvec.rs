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

use super::bitvec::BitVec;
use std::iter::{Extend, FromIterator, FusedIterator};

use super::Literal;

/// A unifying interface for regular and bit vectors.
pub trait GenVec<ELEM>
where
    ELEM: Copy,
    Self: Default + Clone,
    Self: IntoIterator<Item = ELEM>,
    Self: FromIterator<ELEM>,
    Self: Extend<ELEM>,
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

    /// Returns an iterator over copied elements of the vector.
    fn gen_iter<'a>(&'a self) -> <Self as CopyIterable<'a, ELEM>>::Iter
    where
        Self: CopyIterable<'a, ELEM>,
    {
        self.iter_copy()
    }
}

impl<ELEM> GenVec<ELEM> for Vec<ELEM>
where
    ELEM: Copy,
{
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn from_elem(elem: ELEM) -> Self {
        vec![elem]
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= Vec::len(self));
        self.truncate(new_len);
    }

    fn resize(&mut self, new_len: usize, elem: ELEM) {
        self.resize(new_len, elem);
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn push(&mut self, elem: ELEM) {
        self.push(elem);
    }

    fn pop(&mut self) -> Option<ELEM> {
        self.pop()
    }

    fn append(&mut self, other: &mut Self) {
        self.append(other)
    }

    fn get(&self, index: usize) -> ELEM {
        self[index]
    }

    unsafe fn get_unchecked(&self, index: usize) -> ELEM {
        *<[ELEM]>::get_unchecked(self, index)
    }

    fn set(&mut self, index: usize, elem: ELEM) {
        self[index] = elem;
    }

    unsafe fn set_unchecked(&mut self, index: usize, elem: ELEM) {
        *<[ELEM]>::get_unchecked_mut(self, index) = elem;
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}

impl GenVec<bool> for BitVec {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn clear(&mut self) {
        Self::clear(self);
    }

    fn truncate(&mut self, new_len: usize) {
        self.truncate(new_len);
    }

    fn resize(&mut self, new_len: usize, elem: bool) {
        self.resize(new_len, elem);
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn push(&mut self, elem: bool) {
        self.push(elem);
    }

    fn pop(&mut self) -> Option<bool> {
        self.pop()
    }

    fn append(&mut self, other: &mut Self) {
        self.append(other);
    }

    fn get(&self, index: usize) -> bool {
        self.get(index)
    }

    unsafe fn get_unchecked(&self, index: usize) -> bool {
        self.get_unchecked(index)
    }

    fn set(&mut self, index: usize, elem: bool) {
        self.set(index, elem);
    }

    unsafe fn set_unchecked(&mut self, index: usize, elem: bool) {
        self.set_unchecked(index, elem);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}

/// The iterator for unit vectors.
pub struct UnitIter {
    pos: usize,
}

impl Iterator for UnitIter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > 0 {
            self.pos -= 1;
            Some(())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.pos, Some(self.pos))
    }

    fn count(self) -> usize {
        self.pos
    }

    fn last(self) -> Option<Self::Item> {
        if self.pos > 0 {
            Some(())
        } else {
            None
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.pos > n {
            self.pos -= n + 1;
            Some(())
        } else {
            self.pos = 0;
            None
        }
    }
}

impl FusedIterator for UnitIter {}

/// A vector containing unit `()` elements only (just the length is stored).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnitVec {
    len: usize,
}

impl IntoIterator for UnitVec {
    type Item = ();
    type IntoIter = UnitIter;

    fn into_iter(self) -> Self::IntoIter {
        UnitIter { pos: self.len }
    }
}

impl FromIterator<()> for UnitVec {
    fn from_iter<ITER>(iter: ITER) -> Self
    where
        ITER: IntoIterator<Item = ()>,
    {
        UnitVec {
            len: iter.into_iter().count(),
        }
    }
}

impl Extend<()> for UnitVec {
    fn extend<ITER>(&mut self, iter: ITER)
    where
        ITER: IntoIterator<Item = ()>,
    {
        self.len += iter.into_iter().count();
    }
}

impl GenVec<()> for UnitVec {
    fn new() -> Self {
        UnitVec { len: 0 }
    }

    fn with_capacity(_capacity: usize) -> Self {
        UnitVec { len: 0 }
    }

    fn split(self, len: usize) -> Vec<Self> {
        if self.len == 0 {
            return Vec::new();
        }
        assert_ne!(len, 0);
        std::iter::repeat(UnitVec { len })
            .take(self.len / len)
            .collect()
    }

    fn from_elem(_elem: ()) -> Self {
        UnitVec { len: 1 }
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        self.len = new_len;
    }

    fn resize(&mut self, new_len: usize, _elem: ()) {
        self.len = new_len;
    }

    fn reserve(&mut self, _additional: usize) {}

    fn push(&mut self, _elem: ()) {
        self.len += 1;
    }

    fn pop(&mut self) -> Option<()> {
        if self.len > 0 {
            self.len -= 1;
            Some(())
        } else {
            None
        }
    }

    fn append(&mut self, other: &mut Self) {
        self.len += other.len;
        other.len = 0;
    }

    fn get(&self, index: usize) {
        assert!(index < self.len);
    }

    unsafe fn get_unchecked(&self, _index: usize) {}

    fn set(&mut self, index: usize, _elem: ()) {
        assert!(index < self.len);
    }

    unsafe fn set_unchecked(&mut self, _index: usize, _elem: ()) {}

    fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize {
        usize::max_value()
    }
}

/// A helper trait to find the right iterator that returns elements and not
/// references.
pub trait CopyIterable<'a, ELEM> {
    type Iter: Iterator<Item = ELEM>;

    fn iter_copy(&'a self) -> Self::Iter;
}

impl<'a, ELEM: 'a + Copy> CopyIterable<'a, ELEM> for Vec<ELEM> {
    type Iter = std::iter::Copied<std::slice::Iter<'a, ELEM>>;

    fn iter_copy(&'a self) -> Self::Iter {
        self.iter().copied()
    }
}

impl<'a> CopyIterable<'a, bool> for BitVec {
    type Iter = super::bitvec::CopyIter<'a>;

    fn iter_copy(&'a self) -> Self::Iter {
        self.iter_copy()
    }
}

impl<'a> CopyIterable<'a, ()> for UnitVec {
    type Iter = UnitIter;

    fn iter_copy(&'a self) -> Self::Iter {
        self.into_iter()
    }
}

/// A trait for elements that can be stored in a generic vector.
pub trait GenElem: Copy {
    /// A type that can be used for storing a vector of elements.
    type Vec: GenVec<Self> + PartialEq + std::fmt::Debug + for<'a> CopyIterable<'a, Self>;
}

impl GenElem for bool {
    type Vec = BitVec;
}

impl GenElem for usize {
    type Vec = Vec<Self>;
}

impl GenElem for Literal {
    type Vec = Vec<Self>;
}

impl GenElem for () {
    type Vec = UnitVec;
}

pub type VecFor<ELEM> = <ELEM as GenElem>::Vec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: Vec<bool> = GenVec::new();
        let mut v2: VecFor<bool> = GenVec::new();
        let mut v3: VecFor<()> = GenVec::new();
        let mut v4: VecFor<bool> = GenVec::new();

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
                assert_eq!(v1.get(j), v2.get(j));
            }

            v4.clear();
            v4.extend(v1.clone());
            assert_eq!(v2, v4);

            v4.set(v4.len() - 1, !v4.get(v4.len() - 1));
            assert!(v2 != v4);
        }

        for _ in 0..50 {
            for _ in 0..77 {
                v1.pop();
            }
            v2.resize(v2.len() - 77, false);

            assert_eq!(v1.len(), v2.len());
            for j in 0..v1.len() {
                assert_eq!(v1.get(j), v2.get(j));
            }

            v4.clear();
            v4.extend(v1.clone());
            assert_eq!(v2, v4);

            v4.set(v4.len() - 1, !v4.get(v4.len() - 1));
            assert!(v2 != v4);
        }
    }

    #[test]
    fn iters() {
        let e1 = vec![true, false, true];
        let e2 = e1.clone();
        let v1: VecFor<bool> = e1.into_iter().collect();
        let mut v2: VecFor<bool> = GenVec::new();
        for b in e2 {
            v2.push(b);
        }
        assert_eq!(v1, v2);

        let mut iter = v1.gen_iter().skip(1);
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), None);

        let e1 = [true, false];
        let v1: VecFor<bool> = e1.iter().copied().collect();
        let mut v2: VecFor<bool> = GenVec::new();
        for b in &e1 {
            v2.push(*b);
        }
        assert_eq!(v1, v2);

        v2.clear();
        assert_eq!(v2.len(), 0);
        for j in 0..100 {
            v2.push(j % 5 == 0 || j % 3 == 0);
        }
        assert_eq!(v2.len(), 100);
        for j in 0..100 {
            let b1 = unsafe { v2.get_unchecked(j) };
            let b2 = v2.get(j);
            let b3 = j % 5 == 0 || j % 3 == 0;
            assert_eq!(b1, b3);
            assert_eq!(b2, b3);

            let b4 = j % 7 == 0;
            unsafe { v2.set_unchecked(j, b4) };
            assert_eq!(v2.get(j), b4);
        }
    }
}
