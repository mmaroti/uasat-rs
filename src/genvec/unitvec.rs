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

use super::{GenElem, GenSlice, GenVec};
use std::iter::{Extend, FromIterator, FusedIterator};

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

    type Iter<'a> = UnitIter;

    fn copy_iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }

    type Slice<'a> = UnitVec;

    fn slice(&self) -> Self::Slice<'_> {
        *self
    }
}

impl GenSlice<()> for UnitVec {
    fn len(self) -> usize {
        self.len
    }

    fn get(self, index: usize) {
        assert!(index < self.len);
    }

    unsafe fn get_unchecked(self, _index: usize) {}

    fn slice(self, start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { len: end - start }
    }

    type Iter = UnitIter;

    fn copy_iter(self) -> Self::Iter {
        self.into_iter()
    }
}

impl GenElem for () {
    type Vec = UnitVec;
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

impl ExactSizeIterator for UnitIter {}

impl DoubleEndedIterator for UnitIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
