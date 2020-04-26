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
use std::{fmt, iter};

/// Generic interface for regular and bit vectors.
pub trait Vector<ELEM>
where
    ELEM: Copy,
    Self: Default + Clone,
    Self: IntoIterator<Item = ELEM> + iter::FromIterator<ELEM>,
    Self: iter::Extend<ELEM>,
{
    /// Constructs a new empty vector. The vector will not allocate until
    /// elements are pushed onto it.
    fn new() -> Self;

    /// Constructs a new empty vector with the specified capacity. The vector
    /// will be able to hold exactly capacity elements without reallocating.
    fn with_capacity(capacity: usize) -> Self;

    /// Creates a vector with a single element.
    fn from_elem(elem: ELEM) -> Self {
        let mut vec: Self = Vector::with_capacity(1);
        vec.push(elem);
        vec
    }

    /// Clears the vector, removing all values.
    fn clear(self: &mut Self);

    /// Resizes the vector in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the the vector is extended by the
    /// difference, with each additional slot filled with `elem`.
    /// If `new_len` is less than `len`, then the vector is simply truncated.
    fn resize(self: &mut Self, new_len: usize, elem: ELEM);

    /// Reserves capacity for at least additional more bits to be inserted in
    /// the given vector. The collection may reserve more space to avoid
    /// frequent reallocations.
    fn reserve(self: &mut Self, additional: usize);

    /// Appends an element to the back of the vector.
    fn push(self: &mut Self, elem: ELEM);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(self: &mut Self) -> Option<ELEM>;

    /// Extends this vector by moving all elements from the other vector,
    /// leaving the other vector empty.
    fn append(self: &mut Self, other: &mut Self);

    /// Returns the element at the given index. Panics if the index is
    /// out of bounds.
    fn get(self: &Self, index: usize) -> ELEM;

    /// Returns the element at the given index without bound checks.
    /// # Safety
    /// Do not use this in general code, use `ranges` if possible.
    unsafe fn get_unchecked(self: &Self, index: usize) -> ELEM {
        self.get(index)
    }

    /// Sets the element at the given index to the new value. Panics if the
    /// index is out of bounds.
    fn set(self: &mut Self, index: usize, elem: ELEM);

    /// Sets the element at the given index to the new value without bound
    /// checks.
    /// # Safety
    /// Do not use this in general code.
    unsafe fn set_unchecked(self: &mut Self, index: usize, elem: ELEM) {
        self.set(index, elem);
    }

    /// Returns the number of elements in the vector.
    fn len(self: &Self) -> usize;

    /// Returns `true` if the length is zero.
    fn is_empty(self: &Self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(self: &Self) -> usize;

    /// Returns an iterator for the given range of elements.
    fn range(self: &Self, start: usize, end: usize) -> VecIter<'_, ELEM, Self> {
        VecIter::new(self, start, end)
    }

    /// Returns an iterator over the elements of the vector.
    fn iter(self: &Self) -> VecIter<'_, ELEM, Self> {
        self.range(0, self.len())
    }
}

/// Generic read only iterator over the vector.
pub struct VecIter<'a, ELEM, VEC> {
    pos: usize,
    end: usize,
    vec: &'a VEC,
    phantom: std::marker::PhantomData<ELEM>,
}

impl<'a, ELEM, VEC> VecIter<'a, ELEM, VEC>
where
    ELEM: Copy,
    VEC: Vector<ELEM>,
{
    fn new(vec: &'a VEC, start: usize, end: usize) -> Self {
        assert!(start <= end && end <= vec.len());
        VecIter {
            pos: start,
            end,
            vec,
            phantom: Default::default(),
        }
    }
}

impl<'a, ELEM, VEC> Iterator for VecIter<'a, ELEM, VEC>
where
    ELEM: Copy,
    VEC: Vector<ELEM>,
{
    type Item = ELEM;

    fn next(self: &mut Self) -> Option<Self::Item> {
        if self.pos < self.end {
            let elem = unsafe { self.vec.get_unchecked(self.pos) };
            self.pos += 1;
            Some(elem)
        } else {
            None
        }
    }

    fn size_hint(self: &Self) -> (usize, Option<usize>) {
        (self.end - self.pos, Some(self.end - self.pos))
    }

    fn count(self: Self) -> usize {
        self.end - self.pos
    }

    fn last(self: Self) -> Option<Self::Item> {
        if self.pos < self.end {
            let elem = unsafe { self.vec.get_unchecked(self.end - 1) };
            Some(elem)
        } else {
            None
        }
    }

    fn nth(self: &mut Self, n: usize) -> Option<Self::Item> {
        if self.end - self.pos < n {
            let elem = unsafe { self.vec.get_unchecked(self.pos + n) };
            self.pos += n + 1;
            Some(elem)
        } else {
            self.pos = self.end;
            None
        }
    }
}

impl<'a, ELEM, VEC> ExactSizeIterator for VecIter<'a, ELEM, VEC>
where
    ELEM: Copy,
    VEC: Vector<ELEM>,
{
    fn len(self: &Self) -> usize {
        self.end - self.pos
    }
}

impl<'a, ELEM, VEC> iter::FusedIterator for VecIter<'a, ELEM, VEC>
where
    ELEM: Copy,
    VEC: Vector<ELEM>,
{
}

/// A wrapper around standard containers to present them as generic vectors.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct VecImpl<DATA> {
    data: DATA,
}

impl<DATA> IntoIterator for VecImpl<DATA>
where
    DATA: IntoIterator,
{
    type Item = DATA::Item;

    type IntoIter = DATA::IntoIter;

    fn into_iter(self: Self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<DATA, ELEM> iter::FromIterator<ELEM> for VecImpl<DATA>
where
    DATA: iter::FromIterator<ELEM>,
{
    fn from_iter<ITER>(iter: ITER) -> Self
    where
        ITER: IntoIterator<Item = ELEM>,
    {
        VecImpl {
            data: iter::FromIterator::from_iter(iter),
        }
    }
}

impl<DATA, ELEM> iter::Extend<ELEM> for VecImpl<DATA>
where
    DATA: iter::Extend<ELEM>,
{
    fn extend<ITER>(self: &mut Self, iter: ITER)
    where
        ITER: IntoIterator<Item = ELEM>,
    {
        self.data.extend(iter);
    }
}

impl<ELEM> Vector<ELEM> for VecImpl<Vec<ELEM>>
where
    ELEM: Copy,
{
    fn new() -> Self {
        VecImpl { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        VecImpl {
            data: Vec::with_capacity(capacity),
        }
    }

    fn from_elem(elem: ELEM) -> Self {
        VecImpl { data: vec![elem] }
    }

    fn clear(self: &mut Self) {
        self.data.clear();
    }

    fn resize(self: &mut Self, new_len: usize, elem: ELEM) {
        self.data.resize(new_len, elem);
    }

    fn reserve(self: &mut Self, additional: usize) {
        self.data.reserve(additional);
    }

    fn push(self: &mut Self, elem: ELEM) {
        self.data.push(elem);
    }

    fn pop(self: &mut Self) -> Option<ELEM> {
        self.data.pop()
    }

    fn append(self: &mut Self, other: &mut Self) {
        self.data.append(&mut other.data);
    }

    fn get(self: &Self, index: usize) -> ELEM {
        self.data[index]
    }

    unsafe fn get_unchecked(self: &Self, index: usize) -> ELEM {
        *self.data.get_unchecked(index)
    }

    fn set(self: &mut Self, index: usize, elem: ELEM) {
        self.data[index] = elem;
    }

    unsafe fn set_unchecked(self: &mut Self, index: usize, elem: ELEM) {
        *self.data.get_unchecked_mut(index) = elem;
    }

    fn len(self: &Self) -> usize {
        self.data.len()
    }

    fn is_empty(self: &Self) -> bool {
        self.data.is_empty()
    }

    fn capacity(self: &Self) -> usize {
        self.data.capacity()
    }
}

impl Vector<bool> for VecImpl<bit_vec::BitVec> {
    fn new() -> Self {
        VecImpl {
            data: bit_vec::BitVec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        VecImpl {
            data: bit_vec::BitVec::with_capacity(capacity),
        }
    }

    fn clear(self: &mut Self) {
        self.data.truncate(0);
    }

    fn resize(self: &mut Self, new_len: usize, elem: bool) {
        if new_len > self.len() {
            self.data.grow(new_len - self.len(), elem);
        } else {
            self.data.truncate(new_len);
        }
    }

    fn reserve(self: &mut Self, additional: usize) {
        self.data.reserve(additional);
    }

    fn push(self: &mut Self, elem: bool) {
        self.data.push(elem);
    }

    fn pop(self: &mut Self) -> Option<bool> {
        self.data.pop()
    }

    fn append(self: &mut Self, other: &mut Self) {
        self.data.append(&mut other.data);
    }

    fn get(self: &Self, index: usize) -> bool {
        self.data.get(index).unwrap()
    }

    unsafe fn get_unchecked(self: &Self, index: usize) -> bool {
        type B = u32;
        let w = index / B::bits();
        let b = index % B::bits();
        let x = *self.data.storage().get_unchecked(w);
        let y = B::one() << b;
        (x & y) != B::zero()
    }

    fn set(self: &mut Self, index: usize, elem: bool) {
        self.data.set(index, elem);
    }

    unsafe fn set_unchecked(self: &mut Self, index: usize, elem: bool) {
        type B = u32;
        let w = index / B::bits();
        let b = index % B::bits();
        let x = self.data.storage_mut().get_unchecked_mut(w);
        let y = B::one() << b;
        if elem {
            *x |= y;
        } else {
            *x &= !y;
        }
    }

    fn len(self: &Self) -> usize {
        self.data.len()
    }

    fn is_empty(self: &Self) -> bool {
        self.data.is_empty()
    }

    fn capacity(self: &Self) -> usize {
        self.data.capacity()
    }
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

    fn size_hint(self: &Self) -> (usize, Option<usize>) {
        (self.pos, Some(self.pos))
    }

    fn count(self: Self) -> usize {
        self.pos
    }

    fn last(self: Self) -> Option<Self::Item> {
        if self.pos > 0 {
            Some(())
        } else {
            None
        }
    }

    fn nth(self: &mut Self, n: usize) -> Option<Self::Item> {
        if self.pos > n {
            self.pos -= n + 1;
            Some(())
        } else {
            self.pos = 0;
            None
        }
    }
}

impl iter::FusedIterator for UnitIter {}

/// A vector containing unit `()` elements only (just the length is stored).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnitVec {
    len: usize,
}

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
        UnitVec {
            len: iter.into_iter().count(),
        }
    }
}

impl Extend<()> for UnitVec {
    fn extend<ITER>(self: &mut Self, iter: ITER)
    where
        ITER: IntoIterator<Item = ()>,
    {
        self.len += iter.into_iter().count();
    }
}

impl Vector<()> for UnitVec {
    fn new() -> Self {
        UnitVec { len: 0 }
    }

    fn with_capacity(_capacity: usize) -> Self {
        UnitVec { len: 0 }
    }

    fn from_elem(_elem: ()) -> Self {
        UnitVec { len: 1 }
    }

    fn clear(self: &mut Self) {
        self.len = 0;
    }

    fn resize(self: &mut Self, new_len: usize, _elem: ()) {
        self.len = new_len
    }

    fn reserve(self: &mut Self, _additional: usize) {}

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

    fn append(self: &mut Self, other: &mut Self) {
        self.len += other.len;
        other.len = 0;
    }

    fn get(self: &Self, index: usize) {
        assert!(index < self.len);
    }

    unsafe fn get_unchecked(self: &Self, _index: usize) {}

    fn set(self: &mut Self, index: usize, _elem: ()) {
        assert!(index < self.len);
    }

    unsafe fn set_unchecked(self: &mut Self, _index: usize, _elem: ()) {}

    fn len(self: &Self) -> usize {
        self.len
    }

    fn capacity(self: &Self) -> usize {
        usize::max_value()
    }
}

/// A helper trait to find the right generic vector for a given element.
pub trait Element: Copy {
    /// A type that can be used for storing a vector of elements.
    type Vector: Vector<Self> + PartialEq + fmt::Debug;
}

impl Element for bool {
    type Vector = VecImpl<bit_vec::BitVec>;
}

impl Element for usize {
    type Vector = VecImpl<Vec<Self>>;
}

impl Element for solver::Literal {
    type Vector = VecImpl<Vec<Self>>;
}

impl Element for () {
    type Vector = UnitVec;
}

/// Returns the generic vector type that can hold the given element.
pub type VectorFor<ELEM> = <ELEM as Element>::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: VecImpl<Vec<bool>> = Vector::new();
        let mut v2: VectorFor<bool> = Vector::new();
        let mut v3: VectorFor<()> = Vector::new();

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
        }
    }

    #[test]
    fn iters() {
        let e1 = vec![true, false, true];
        let e2 = e1.clone();
        let v1: VectorFor<bool> = e1.into_iter().collect();
        let mut v2: VectorFor<bool> = Vector::new();
        for b in e2 {
            v2.push(b);
        }
        assert_eq!(v1, v2);

        let mut iter = Vector::range(&v1, 1, 3);
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), None);

        let e1 = [true, false];
        let v1: VectorFor<bool> = e1.iter().cloned().collect();
        let mut v2: VectorFor<bool> = Vector::new();
        for b in &e1 {
            v2.push(*b);
        }
        assert_eq!(v1, v2);

        v2.clear();
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
