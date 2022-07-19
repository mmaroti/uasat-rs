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

use std::iter::{Extend, FromIterator, FusedIterator};

#[derive(Default, Clone)]
pub struct BitVec {
    len: usize,
    data: Vec<u32>,
}

impl Extend<bool> for BitVec {}

impl IntoIterator for BitVec {
    type Item = bool;
}

impl FromIterator<bool> for BitVec {}

impl BitVec {
    fn gen_new() -> Self {
        BitVec {
            len: 0,
            data: Vec::new(),
        }
    }

    fn gen_with_capacity(capacity: usize) -> Self {
        BitVec {
            len: 0,
            data: Vec::with_capacity((capacity + 31) / 32),
        }
    }

    fn gen_from_elem(elem: bool) -> Self {
        BitVec {
            len: 1,
            data: vec![if elem { 1 } else { 0 }],
        }
    }

    fn gen_clear(&mut self) {
        self.len = 0;
        self.data.clear();
    }

    fn gen_truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        self.len = new_len;
        self.data.truncate((new_len + 31) / 32);
    }

    fn gen_resize(&mut self, new_len: usize, elem: bool) {
        Vec::resize(self, new_len, elem);
    }

    fn gen_reserve(&mut self, additional: usize) {
        let new_len = (self.len + additional + 31) / 32;
        self.data.reserve(new_len - self.data.len());
    }

    fn gen_push(&mut self, elem: bool) {
        let b = self.len % 32;
        if b == 0 {
            self.data.push(if elem { 1 } else { 0 });
        } else {
            let &mut a = unsafe { self.data.get_unchecked_mut(self.len / 32) };
            let b = 1 << b;
            if elem {
                a |= b;
            } else {
                a &= !b;
            }
        }
        self.len += 1;
    }

    fn gen_pop(&mut self) -> Option<bool> {
        Vec::pop(self)
    }

    fn gen_append(&mut self, other: &mut Self) {
        Vec::append(self, other);
    }

    fn gen_get(&self, index: usize) -> bool {
        assert!(index < self.len);
        let a = self.data[index / 32];
        let b = 1 << (index % 32);
        (a & b) != 0
    }

    unsafe fn gen_get_unchecked(&self, index: usize) -> bool {
        debug_assert!(index < self.len);
        let a = self.data.get_unchecked(index / 32);
        let b = 1 << (index % 32);
        (a & b) != 0
    }

    fn gen_set(&mut self, index: usize, elem: bool) {
        assert!(index < self.len);
        let &mut a = &mut self.data[index / 32];
        let b = 1 << (index % 32);
        if elem {
            a |= b;
        } else {
            a &= !b;
        }
    }

    unsafe fn gen_set_unchecked(&mut self, index: usize, elem: bool) {
        debug_assert!(index < self.len);
        let &mut a = self.data.get_unchecked_mut(index / 32);
        let b = 1 << (index % 32);
        if elem {
            a |= b;
        } else {
            a &= !b;
        }
    }

    fn gen_len(&self) -> usize {
        self.len
    }

    fn gen_is_empty(&self) -> bool {
        self.len == 0
    }

    fn gen_capacity(&self) -> usize {
        self.data.capacity() * 32
    }
}
