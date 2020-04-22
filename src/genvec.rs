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

extern crate bit_vec;
use super::solver;
use bit_vec::BitBlock as _;
use std::iter;

/// Generic interface for regular and bit vectors.
pub trait GenVec<ELEM>
where
    ELEM: Copy,
    Self: Default + Clone,
    Self: IntoIterator<Item = ELEM> + iter::FromIterator<ELEM>,
{
    /// Constructs a new empty vector. The vector will not allocate until
    /// elements are pushed onto it.
    fn new() -> Self;

    /// Constructs a new empty vector with the specified capacity. The vector
    /// will be able to hold exactly capacity elements without reallocating.
    fn with_capacity(capacity: usize) -> Self;

    /// Constructs a new vector with the specified length where the value at
    /// each index is `op(index)`.
    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> ELEM;

    /// Creates a vector with a single element.
    fn from_elem1(elem: ELEM) -> Self {
        let mut vec: Self = GenVec::with_capacity(1);
        vec.push(elem);
        vec
    }

    /// Creates a vector with a pair of elements.
    fn from_elem2(elem1: ELEM, elem2: ELEM) -> Self {
        let mut vec: Self = GenVec::with_capacity(2);
        vec.push(elem1);
        vec.push(elem2);
        vec
    }

    /// Clears the vector, removing all values.
    fn clear(self: &mut Self);

    /// Resizes the vector in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the the vector is extended by the
    /// difference, with each additional slot filled with `elem`.
    /// If `new_len` is less than `len`, then the vector is simply truncated.
    fn resize(self: &mut Self, new_len: usize, elem: ELEM);

    /// Appends an element to the back of the vector.
    fn push(self: &mut Self, elem: ELEM);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(self: &mut Self) -> Option<ELEM>;

    /// Extends this vector by copying all elements from the other vector.
    fn extend(self: &mut Self, other: &Self);

    /// Extends this vector by moving all elements from the other vector.
    fn append(self: &mut Self, other: &mut Self);

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn get(self: &Self, index: usize) -> ELEM;

    /// Returns the element at the given index without bound checks.
    /// # Safety
    /// Do not use this in general code, use `ranges` if possible.
    unsafe fn __get_unchecked__(self: &Self, index: usize) -> ELEM {
        self.get(index)
    }

    /// Sets the element at the given index to the new value. Panics if the
    /// index is out of bounds.
    fn set(self: &mut Self, index: usize, elem: ELEM);

    /// Sets the element at the given index to the new value without bound
    /// checks.
    /// # Safety
    /// Do not use this in general code.
    unsafe fn __set_unchecked__(self: &mut Self, index: usize, elem: ELEM) {
        self.set(index, elem);
    }

    /// Returns the number of elements in the vector.
    fn len(self: &Self) -> usize;

    /// Returns `true` if the length is zero.
    fn is_empty(self: &Self) -> bool;

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(self: &Self) -> usize;

    /// Returns an iterator for the given range of elements.
    fn range(self: &Self, start: usize, end: usize) -> GenIter<ELEM, &Self> {
        assert!(start <= end && end <= self.len());
        GenIter {
            pos: start,
            len: end,
            vec: self,
            phantom: Default::default(),
        }
    }

    /// Returns an iterator over the elements of the vector.
    fn gen_iter(self: &Self) -> GenIter<ELEM, &Self> {
        self.range(0, self.len())
    }
}

/// Generic read only iterator over the vector.
pub struct GenIter<ELEM, VEC> {
    pos: usize,
    len: usize,
    vec: VEC,
    phantom: std::marker::PhantomData<ELEM>,
}

impl<'a, ELEM, VEC> Iterator for GenIter<ELEM, &'a VEC>
where
    ELEM: Copy,
    VEC: GenVec<ELEM>,
{
    type Item = ELEM;

    fn next(self: &mut Self) -> Option<Self::Item> {
        if self.pos < self.len {
            let elem = unsafe { self.vec.__get_unchecked__(self.pos) };
            self.pos += 1;
            Some(elem)
        } else {
            None
        }
    }
}

impl<'a, ELEM, VEC> iter::FusedIterator for GenIter<ELEM, &'a VEC>
where
    ELEM: Copy,
    VEC: GenVec<ELEM>,
{
}

impl<ELEM> GenVec<ELEM> for Vec<ELEM>
where
    ELEM: Copy,
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

    fn from_elem1(elem: ELEM) -> Self {
        vec![elem]
    }

    fn from_elem2(elem1: ELEM, elem2: ELEM) -> Self {
        vec![elem1, elem2]
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
        bit_vec::BitVec::truncate(self, 0);
    }

    fn resize(self: &mut Self, new_len: usize, elem: bool) {
        if new_len > self.len() {
            bit_vec::BitVec::grow(self, new_len - self.len(), elem);
        } else {
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

    unsafe fn __get_unchecked__(self: &Self, index: usize) -> bool {
        type B = u32;
        let w = index / B::bits();
        let b = index % B::bits();
        let x = *self.storage().get_unchecked(w);
        let y = B::one() << b;
        (x & y) != B::zero()
    }

    fn set(self: &mut Self, index: usize, elem: bool) {
        bit_vec::BitVec::set(self, index, elem);
    }

    unsafe fn __set_unchecked__(self: &mut Self, index: usize, elem: bool) {
        type B = u32;
        let w = index / B::bits();
        let b = index % B::bits();
        let x = self.storage_mut().get_unchecked_mut(w);
        let y = B::one() << b;
        if elem {
            *x |= y;
        } else {
            *x &= !y;
        }
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

/// A vector containing unit `()` elements only (just the length is stored).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnitVec {
    len: usize,
}

/// The iterator for unit vectors.
pub struct UnitIter {
    pos: usize,
}

impl Iterator for UnitIter {
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

impl iter::FusedIterator for UnitIter {}

impl IntoIterator for UnitVec {
    type Item = ();
    type IntoIter = UnitIter;

    fn into_iter(self: Self) -> Self::IntoIter {
        UnitIter { pos: self.len }
    }
}

impl iter::FromIterator<()> for UnitVec {
    fn from_iter<ITER>(iter: ITER) -> Self
    where
        ITER: IntoIterator<Item = ()>,
    {
        let mut len = 0;
        for _ in iter {
            len += 1;
        }
        UnitVec { len }
    }
}

impl GenVec<()> for UnitVec {
    fn new() -> Self {
        UnitVec { len: 0 }
    }

    fn with_capacity(_capacity: usize) -> Self {
        UnitVec { len: 0 }
    }

    fn from_fn<F>(len: usize, _op: F) -> Self
    where
        F: FnMut(usize) -> (),
    {
        UnitVec { len }
    }

    fn from_elem1(_elem: ()) -> Self {
        UnitVec { len: 1 }
    }

    fn from_elem2(_elem1: (), _elem2: ()) -> Self {
        UnitVec { len: 2 }
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

    fn get(self: &Self, index: usize) {
        assert!(index < self.len);
    }

    unsafe fn __get_unchecked__(self: &Self, _index: usize) {}

    fn set(self: &mut Self, index: usize, _elem: ()) {
        assert!(index < self.len);
    }

    unsafe fn __set_unchecked__(self: &mut Self, _index: usize, _elem: ()) {}

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

/// A helper trait to find the right generic vector for a given element.
pub trait GenElem: Copy {
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
    type Vector = UnitVec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: bit_vec::BitVec = GenVec::new();
        let mut v2: Vec<bool> = GenVec::new();
        let mut v3: UnitVec = GenVec::new();

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

    #[test]
    fn iters() {
        let e1 = vec![true, false, true];
        let e2 = e1.clone();
        let v1: <bool as GenElem>::Vector = e1.into_iter().collect();
        let mut v2: <bool as GenElem>::Vector = GenVec::new();
        for b in e2 {
            GenVec::push(&mut v2, b);
        }
        assert_eq!(v1, v2);

        let mut iter = GenVec::range(&v1, 1, 3);
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), None);

        let e1 = [true, false];
        let v1: <bool as GenElem>::Vector = e1.iter().cloned().collect();
        let mut v2: <bool as GenElem>::Vector = GenVec::new();
        for b in e1.iter() {
            GenVec::push(&mut v2, *b);
        }
        assert_eq!(v1, v2);

        GenVec::clear(&mut v2);
        for j in 0..100 {
            GenVec::push(&mut v2, j % 5 == 0 || j % 3 == 0);
        }
        assert_eq!(v2.len(), 100);
        for j in 0..100 {
            let b1 = unsafe { GenVec::__get_unchecked__(&v2, j) };
            let b2 = GenVec::get(&v2, j);
            let b3 = v2.get(j).unwrap();
            let b4 = j % 5 == 0 || j % 3 == 0;
            assert_eq!(b1, b4);
            assert_eq!(b2, b4);
            assert_eq!(b3, b4);

            let b5 = j % 7 == 0;
            unsafe { GenVec::__set_unchecked__(&mut v2, j, b5) };
            assert_eq!(v2.get(j).unwrap(), b5);
        }
    }
}
