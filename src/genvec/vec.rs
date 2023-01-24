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

use super::{GenElem, GenIterable, GenSlice, GenVec};
use crate::core::Literal;

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

impl<'a, ELEM> GenIterable<'a, ELEM> for Vec<ELEM>
where
    ELEM: Copy + 'a,
{
    type Slice = &'a [ELEM];

    fn gen_slice_impl(&'a self) -> Self::Slice {
        self
    }

    type Iter = std::iter::Copied<std::slice::Iter<'a, ELEM>>;

    fn gen_iter_impl(&'a self) -> Self::Iter {
        self.iter().copied()
    }
}

impl<'a, ELEM> GenSlice<ELEM> for &'a [ELEM]
where
    ELEM: Copy,
{
    fn len(self) -> usize {
        <[ELEM]>::len(self)
    }

    fn is_empty(self) -> bool {
        <[ELEM]>::is_empty(self)
    }

    fn get(self, index: usize) -> ELEM {
        self[index]
    }

    unsafe fn get_unchecked(self, index: usize) -> ELEM {
        *<[ELEM]>::get_unchecked(self, index)
    }

    fn get_slice(self, start: usize, end: usize) -> Self {
        &self[start..end]
    }
}

impl GenElem for usize {
    type Vec = Vec<Self>;
}

impl GenElem for Literal {
    type Vec = Vec<Self>;
}
