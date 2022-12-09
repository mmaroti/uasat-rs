/*
* Copyright (C) 2019-2022, Miklos Maroti
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

//! A simple bit vector implementation.

#![allow(unused)]

use std::iter::{ExactSizeIterator, Extend, FromIterator, FusedIterator};

#[derive(Default, Clone)]
pub struct BitVec {
    len: usize,
    data: Vec<u32>,
}

impl BitVec {
    pub fn new() -> Self {
        BitVec {
            len: 0,
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        BitVec {
            len: 0,
            data: Vec::with_capacity((capacity + 31) / 32),
        }
    }

    pub fn from_elem(elem: bool) -> Self {
        BitVec {
            len: 1,
            data: vec![u32::from(elem)],
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.data.clear();
    }

    pub fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        self.len = new_len;
        self.data.truncate((new_len + 31) / 32);
    }

    pub fn resize(&mut self, new_len: usize, elem: bool) {
        while self.len < new_len && self.len % 32 != 0 {
            self.push(elem);
        }
        self.len = new_len;
        self.data
            .resize((new_len + 31) / 32, if elem { 0xffffffff } else { 0x0 });
    }

    pub fn reserve(&mut self, additional: usize) {
        let new_len = (self.len + additional + 31) / 32;
        self.data.reserve(new_len - self.data.len());
    }

    pub fn push(&mut self, elem: bool) {
        if self.len % 32 == 0 {
            self.data.push(0);
        }
        self.len += 1;
        unsafe { self.set_unchecked(self.len - 1, elem) };
    }

    pub fn pop(&mut self) -> Option<bool> {
        if self.len == 0 {
            None
        } else {
            let a = unsafe { self.get_unchecked(self.len - 1) };
            self.len -= 1;
            Some(a)
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.reserve(other.len());
        for elem in other.copy_iter() {
            self.push(elem);
        }
        other.clear();
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.len);
        let a = self.data[index / 32];
        let b = 1 << (index % 32);
        (a & b) != 0
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> bool {
        debug_assert!(index < self.len);
        let a = self.data.get_unchecked(index / 32);
        let b = 1 << (index % 32);
        (a & b) != 0
    }

    pub fn set(&mut self, index: usize, elem: bool) {
        assert!(index < self.len);
        let a = &mut self.data[index / 32];
        let b = 1 << (index % 32);
        if elem {
            *a |= b;
        } else {
            *a &= !b;
        }
    }

    pub unsafe fn set_unchecked(&mut self, index: usize, elem: bool) {
        debug_assert!(index < self.len);
        let a = self.data.get_unchecked_mut(index / 32);
        let b = 1 << (index % 32);
        if elem {
            *a |= b;
        } else {
            *a &= !b;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity() * 32
    }

    pub fn copy_iter(&self) -> CopyIter<'_> {
        CopyIter { pos: 0, vec: self }
    }
}

impl Extend<bool> for BitVec {
    fn extend<ITER: IntoIterator<Item = bool>>(&mut self, iter: ITER) {
        let iter = iter.into_iter();
        let (min, _) = iter.size_hint();
        self.reserve(min);
        for elem in iter {
            self.push(elem)
        }
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<ITER: IntoIterator<Item = bool>>(iter: ITER) -> Self {
        let mut ret: Self = Default::default();
        ret.extend(iter);
        ret
    }
}

pub struct CopyIter<'a> {
    pos: usize,
    vec: &'a BitVec,
}

impl<'a> Iterator for CopyIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.pos < self.vec.len() {
            let elem = self.vec.get(self.pos);
            self.pos += 1;
            Some(elem)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num = self.vec.len() - self.pos;
        (num, Some(num))
    }
}

impl<'a> FusedIterator for CopyIter<'a> {}

impl<'a> ExactSizeIterator for CopyIter<'a> {}

pub struct IntoIter {
    pos: usize,
    vec: BitVec,
}

impl Iterator for IntoIter {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.pos < self.vec.len() {
            let elem = self.vec.get(self.pos);
            self.pos += 1;
            Some(elem)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num = self.vec.len() - self.pos;
        (num, Some(num))
    }
}

impl FusedIterator for IntoIter {}

impl ExactSizeIterator for IntoIter {}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter { pos: 0, vec: self }
    }
}
