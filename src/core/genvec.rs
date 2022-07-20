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

use bit_vec::{BitBlock as _, BitVec};
use std::iter::{Extend, FromIterator, FusedIterator};

use super::Literal;

/// A unifying interface for regular and bit vectors.
pub trait GenericVector<ELEM>
where
    ELEM: Copy,
    Self: Default + Clone,
    Self: IntoIterator<Item = ELEM>,
    Self: FromIterator<ELEM>,
    Self: Extend<ELEM>,
{
    /// Constructs a new empty vector. The vector will not allocate until
    /// elements are pushed onto it.
    fn gen_new() -> Self;

    /// Constructs a new empty vector with the specified capacity. The vector
    /// will be able to hold exactly capacity elements without reallocating.
    fn gen_with_capacity(capacity: usize) -> Self;

    /// Concatenates the given vectors into a new one.
    fn gen_concat(parts: Vec<Self>) -> Self {
        let len = parts.iter().map(|a| a.gen_len()).sum();
        let mut result: Self = GenericVector::gen_with_capacity(len);
        for elem in parts.into_iter() {
            result.extend(elem.into_iter());
        }
        result
    }

    /// Splits this vector into equal sized vectors.
    /// TODO: implement more efficient specialized versions
    fn gen_split(self, len: usize) -> Vec<Self> {
        if self.gen_len() == 0 {
            return Vec::new();
        }
        assert_ne!(len, 0);
        let count = self.gen_len() / len;
        let mut result: Vec<Self> = Vec::with_capacity(count);
        let mut iter = self.into_iter();
        for _ in 0..count {
            let mut vec: Self = GenericVector::gen_with_capacity(len);
            for _ in 0..len {
                vec.gen_push(iter.next().unwrap());
            }
            result.push(vec);
        }
        result
    }

    /// Creates a vector with a single element.
    fn gen_from_elem(elem: ELEM) -> Self {
        let mut vec: Self = GenericVector::gen_with_capacity(1);
        vec.gen_push(elem);
        vec
    }

    /// Clears the vector, removing all values.
    fn gen_clear(&mut self);

    /// Shortens the vector, keeping the first `new_len` many elements and
    /// dropping the rest. This method panics if the current `len` is smaller
    /// than `new_len`.
    fn gen_truncate(&mut self, new_len: usize);

    /// Resizes the vector in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the the vector is extended by the
    /// difference, with each additional slot filled with `elem`.
    /// If `new_len` is less than `len`, then the vector is simply truncated.
    fn gen_resize(&mut self, new_len: usize, elem: ELEM);

    /// Reserves capacity for at least additional more elements to be inserted
    /// in the given vector. The collection may reserve more space to avoid
    /// frequent reallocations.
    fn gen_reserve(&mut self, additional: usize);

    /// Appends an element to the back of the vector.
    fn gen_push(&mut self, elem: ELEM);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn gen_pop(&mut self) -> Option<ELEM>;

    /// Extends this vector by moving all elements from the other vector,
    /// leaving the other vector empty.
    fn gen_append(&mut self, other: &mut Self);

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn gen_get(&self, index: usize) -> ELEM;

    /// Returns the element at the given index without bound checks.
    /// # Safety
    /// Do not use this in general code, use `ranges` if possible.
    unsafe fn gen_get_unchecked(&self, index: usize) -> ELEM {
        self.gen_get(index)
    }

    /// Sets the element at the given index to the new value. Panics if the
    /// index is out of bounds.
    fn gen_set(&mut self, index: usize, elem: ELEM);

    /// Sets the element at the given index to the new value without bound
    /// checks.
    /// # Safety
    /// Do not use this in general code.
    unsafe fn gen_set_unchecked(&mut self, index: usize, elem: ELEM) {
        self.gen_set(index, elem);
    }

    /// Returns the number of elements in the vector.
    fn gen_len(&self) -> usize;

    /// Returns `true` if the length is zero.
    fn gen_is_empty(&self) -> bool {
        self.gen_len() == 0
    }

    /// Returns the number of elements the vector can hold without reallocating.
    fn gen_capacity(&self) -> usize;

    /// Returns an iterator over copied elements of the vector.
    fn gen_iter<'a>(&'a self) -> <Self as CopyIterable<'a, ELEM>>::Iter
    where
        Self: CopyIterable<'a, ELEM>,
    {
        self.iter_copy()
    }
}

impl<ELEM> GenericVector<ELEM> for Vec<ELEM>
where
    ELEM: Copy,
{
    fn gen_new() -> Self {
        Vec::new()
    }

    fn gen_with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn gen_from_elem(elem: ELEM) -> Self {
        vec![elem]
    }

    fn gen_clear(&mut self) {
        Vec::clear(self);
    }

    fn gen_truncate(&mut self, new_len: usize) {
        assert!(new_len <= Vec::len(self));
        Vec::truncate(self, new_len);
    }

    fn gen_resize(&mut self, new_len: usize, elem: ELEM) {
        Vec::resize(self, new_len, elem);
    }

    fn gen_reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional);
    }

    fn gen_push(&mut self, elem: ELEM) {
        Vec::push(self, elem);
    }

    fn gen_pop(&mut self) -> Option<ELEM> {
        Vec::pop(self)
    }

    fn gen_append(&mut self, other: &mut Self) {
        Vec::append(self, other);
    }

    fn gen_get(&self, index: usize) -> ELEM {
        self[index]
    }

    unsafe fn gen_get_unchecked(&self, index: usize) -> ELEM {
        *<[ELEM]>::get_unchecked(self, index)
    }

    fn gen_set(&mut self, index: usize, elem: ELEM) {
        self[index] = elem;
    }

    unsafe fn gen_set_unchecked(&mut self, index: usize, elem: ELEM) {
        *<[ELEM]>::get_unchecked_mut(self, index) = elem;
    }

    fn gen_len(&self) -> usize {
        Vec::len(self)
    }

    fn gen_is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn gen_capacity(&self) -> usize {
        Vec::capacity(self)
    }
}
impl GenericVector<bool> for BitVec {
    fn gen_new() -> Self {
        BitVec::new()
    }

    fn gen_with_capacity(capacity: usize) -> Self {
        BitVec::with_capacity(capacity)
    }

    fn gen_clear(&mut self) {
        BitVec::truncate(self, 0);
    }

    fn gen_truncate(&mut self, new_len: usize) {
        assert!(new_len <= BitVec::len(self));
        BitVec::truncate(self, new_len);
    }

    fn gen_resize(&mut self, new_len: usize, elem: bool) {
        if new_len > BitVec::len(self) {
            BitVec::grow(self, new_len - self.gen_len(), elem);
        } else {
            BitVec::truncate(self, new_len);
        }
    }

    fn gen_reserve(&mut self, additional: usize) {
        BitVec::reserve(self, additional);
    }

    fn gen_push(&mut self, elem: bool) {
        BitVec::push(self, elem);
    }

    fn gen_pop(&mut self) -> Option<bool> {
        BitVec::pop(self)
    }

    fn gen_append(&mut self, other: &mut Self) {
        BitVec::append(self, other);
    }

    fn gen_get(&self, index: usize) -> bool {
        BitVec::get(self, index).unwrap()
    }

    unsafe fn gen_get_unchecked(&self, index: usize) -> bool {
        type B = u32;
        let w = index / B::bits();
        let b = index % B::bits();
        let x = *BitVec::storage(self).get_unchecked(w);
        let y = B::one() << b;
        (x & y) != B::zero()
    }

    fn gen_set(&mut self, index: usize, elem: bool) {
        BitVec::set(self, index, elem);
    }

    unsafe fn gen_set_unchecked(&mut self, index: usize, elem: bool) {
        type B = u32;
        let w = index / B::bits();
        let b = index % B::bits();
        let x = BitVec::storage_mut(self).get_unchecked_mut(w);
        let y = B::one() << b;
        if elem {
            *x |= y;
        } else {
            *x &= !y;
        }
    }

    fn gen_len(&self) -> usize {
        BitVec::len(self)
    }

    fn gen_is_empty(&self) -> bool {
        BitVec::is_empty(self)
    }

    fn gen_capacity(&self) -> usize {
        BitVec::capacity(self)
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

impl GenericVector<()> for UnitVec {
    fn gen_new() -> Self {
        UnitVec { len: 0 }
    }

    fn gen_with_capacity(_capacity: usize) -> Self {
        UnitVec { len: 0 }
    }

    fn gen_split(self, len: usize) -> Vec<Self> {
        if self.len == 0 {
            return Vec::new();
        }
        assert_ne!(len, 0);
        std::iter::repeat(UnitVec { len })
            .take(self.len / len)
            .collect()
    }

    fn gen_from_elem(_elem: ()) -> Self {
        UnitVec { len: 1 }
    }

    fn gen_clear(&mut self) {
        self.len = 0;
    }

    fn gen_truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        self.len = new_len;
    }

    fn gen_resize(&mut self, new_len: usize, _elem: ()) {
        self.len = new_len;
    }

    fn gen_reserve(&mut self, _additional: usize) {}

    fn gen_push(&mut self, _elem: ()) {
        self.len += 1;
    }

    fn gen_pop(&mut self) -> Option<()> {
        if self.len > 0 {
            self.len -= 1;
            Some(())
        } else {
            None
        }
    }

    fn gen_append(&mut self, other: &mut Self) {
        self.len += other.len;
        other.len = 0;
    }

    fn gen_get(&self, index: usize) {
        assert!(index < self.len);
    }

    unsafe fn gen_get_unchecked(&self, _index: usize) {}

    fn gen_set(&mut self, index: usize, _elem: ()) {
        assert!(index < self.len);
    }

    unsafe fn gen_set_unchecked(&mut self, _index: usize, _elem: ()) {}

    fn gen_len(&self) -> usize {
        self.len
    }

    fn gen_capacity(&self) -> usize {
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
    type Iter = bit_vec::Iter<'a>;

    fn iter_copy(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a> CopyIterable<'a, ()> for UnitVec {
    type Iter = UnitIter;

    fn iter_copy(&'a self) -> Self::Iter {
        self.into_iter()
    }
}

/// A trait for elements that can be stored in a generic vector.
pub trait GenericElem: Copy {
    /// A type that can be used for storing a vector of elements.
    type Vector: GenericVector<Self> + PartialEq + std::fmt::Debug + for<'a> CopyIterable<'a, Self>;
}

impl GenericElem for bool {
    type Vector = BitVec;
}

impl GenericElem for usize {
    type Vector = Vec<Self>;
}

impl GenericElem for Literal {
    type Vector = Vec<Self>;
}

impl GenericElem for () {
    type Vector = UnitVec;
}

/// Returns the generic vector type that can hold the given element.
pub(crate) type GenericVec<ELEM> = <ELEM as GenericElem>::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: Vec<bool> = GenericVector::gen_new();
        let mut v2: GenericVec<bool> = GenericVector::gen_new();
        let mut v3: GenericVec<()> = GenericVector::gen_new();

        for i in 0..50 {
            let b = i % 2 == 0;

            for _ in 0..90 {
                v1.push(b);
                v3.gen_push(());
                assert_eq!(v1.len(), v3.gen_len());
            }
            v2.gen_resize(v2.gen_len() + 90, b);

            assert_eq!(v1.len(), v2.gen_len());
            for j in 0..v1.len() {
                assert_eq!(v1.gen_get(j), v2.gen_get(j));
            }
        }

        for _ in 0..50 {
            for _ in 0..77 {
                v1.pop();
            }
            v2.gen_resize(v2.gen_len() - 77, false);

            assert_eq!(v1.len(), v2.gen_len());
            for j in 0..v1.len() {
                assert_eq!(v1.gen_get(j), v2.gen_get(j));
            }
        }
    }

    #[test]
    fn iters() {
        let e1 = vec![true, false, true];
        let e2 = e1.clone();
        let v1: GenericVec<bool> = e1.into_iter().collect();
        let mut v2: GenericVec<bool> = GenericVector::gen_new();
        for b in e2 {
            v2.gen_push(b);
        }
        assert_eq!(v1, v2);

        let mut iter = v1.gen_iter().skip(1);
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), None);

        let e1 = [true, false];
        let v1: GenericVec<bool> = e1.iter().copied().collect();
        let mut v2: GenericVec<bool> = GenericVector::gen_new();
        for b in &e1 {
            v2.gen_push(*b);
        }
        assert_eq!(v1, v2);

        v2.gen_clear();
        assert_eq!(v2.gen_len(), 0);
        for j in 0..100 {
            v2.gen_push(j % 5 == 0 || j % 3 == 0);
        }
        assert_eq!(v2.gen_len(), 100);
        for j in 0..100 {
            let b1 = unsafe { v2.gen_get_unchecked(j) };
            let b2 = v2.gen_get(j);
            let b3 = j % 5 == 0 || j % 3 == 0;
            assert_eq!(b1, b3);
            assert_eq!(b2, b3);

            let b4 = j % 7 == 0;
            unsafe { v2.gen_set_unchecked(j, b4) };
            assert_eq!(v2.gen_get(j), b4);
        }
    }
}
