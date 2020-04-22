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
        F: FnMut(usize) -> ELEM,
    {
        (0..len).map(op).collect()
    }

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
    /// TODO: implement it for iterator
    fn extend(self: &mut Self, other: &Self);

    /// Extends this vector by moving all elements from the other vector.
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
            let elem = unsafe { self.vec.get_unchecked(self.pos) };
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Wrapper<DATA> {
    data: DATA,
}

impl<DATA> IntoIterator for Wrapper<DATA>
where
    DATA: IntoIterator,
{
    type Item = DATA::Item;

    type IntoIter = DATA::IntoIter;

    fn into_iter(self: Self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<DATA, ITEM> iter::FromIterator<ITEM> for Wrapper<DATA>
where
    DATA: iter::FromIterator<ITEM>,
{
    fn from_iter<ITER>(iter: ITER) -> Self
    where
        ITER: IntoIterator<Item = ITEM>,
    {
        Wrapper {
            data: iter::FromIterator::from_iter(iter),
        }
    }
}

impl<ELEM> GenVec<ELEM> for Wrapper<Vec<ELEM>>
where
    ELEM: Copy,
{
    fn new() -> Self {
        Wrapper { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Wrapper {
            data: Vec::with_capacity(capacity),
        }
    }

    fn from_elem1(elem: ELEM) -> Self {
        Wrapper { data: vec![elem] }
    }

    fn from_elem2(elem1: ELEM, elem2: ELEM) -> Self {
        Wrapper {
            data: vec![elem1, elem2],
        }
    }

    fn clear(self: &mut Self) {
        self.data.clear();
    }

    fn resize(self: &mut Self, new_len: usize, elem: ELEM) {
        self.data.resize(new_len, elem);
    }

    fn push(self: &mut Self, elem: ELEM) {
        self.data.push(elem);
    }

    fn pop(self: &mut Self) -> Option<ELEM> {
        self.data.pop()
    }

    fn extend(self: &mut Self, other: &Self) {
        self.data.extend(other.data.iter());
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

impl GenVec<bool> for Wrapper<bit_vec::BitVec> {
    fn new() -> Self {
        Wrapper {
            data: bit_vec::BitVec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Wrapper {
            data: bit_vec::BitVec::with_capacity(capacity),
        }
    }

    fn from_fn<F>(len: usize, op: F) -> Self
    where
        F: FnMut(usize) -> bool,
    {
        Wrapper {
            data: bit_vec::BitVec::from_fn(len, op),
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

    fn push(self: &mut Self, elem: bool) {
        self.data.push(elem);
    }

    fn pop(self: &mut Self) -> Option<bool> {
        self.data.pop()
    }

    fn extend(self: &mut Self, other: &Self) {
        self.data.extend(other.data.iter());
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

/// A helper trait to find the right generic vector for a given element.
pub trait GenElem: Copy {
    /// A type that can be used for storing a vector of elements.
    type Vector: GenVec<Self>;
}

impl GenElem for bool {
    type Vector = Wrapper<bit_vec::BitVec>;
}

impl GenElem for usize {
    type Vector = Wrapper<Vec<Self>>;
}

impl GenElem for solver::Literal {
    type Vector = Wrapper<Vec<Self>>;
}

impl GenElem for () {
    type Vector = Wrapper<Vec<Self>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        let mut v1: Wrapper<bit_vec::BitVec> = GenVec::new();
        let mut v2: Wrapper<Vec<bool>> = GenVec::new();
        let mut v3: Wrapper<Vec<()>> = GenVec::new();

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
        let v1: <bool as GenElem>::Vector = e1.into_iter().collect();
        let mut v2: <bool as GenElem>::Vector = GenVec::new();
        for b in e2 {
            v2.push(b);
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
