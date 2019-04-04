/*
* Copyright (C) 2019, Miklos Maroti
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

use super::Array;
use std::{alloc, ptr, usize};

pub struct Packed {
    ptr: *const u64,
    len: usize, // in bits
}

impl Drop for Packed {
    fn drop(self: &mut Self) {
        debug_assert!(self.len <= usize::MAX - 63);
        let bytes = (self.len + 63) >> 6 << 3;
        let layout = unsafe { alloc::Layout::from_size_align_unchecked(bytes, 8) };
        unsafe { alloc::dealloc(self.ptr as *mut u8, layout) };
    }
}

impl Array<bool> for Packed {
    #[allow(clippy::cast_ptr_alignment)]
    fn new(len: usize) -> Self {
        assert!(len <= usize::MAX - 63);
        let bytes = (len + 63) >> 6 << 3;
        let layout = unsafe { alloc::Layout::from_size_align_unchecked(bytes, 8) };
        let ptr = unsafe { alloc::alloc(layout) } as *const u64;
        Packed { ptr, len }
    }

    #[inline]
    fn len(self: &Self) -> usize {
        self.len
    }

    fn set_all(self: &mut Self, val: bool) {
        let value = if val { 0xFF } else { 0x00 };
        let bytes = (self.len + 63) >> 6 << 3;
        unsafe { ptr::write_bytes(self.ptr as *mut u8, value, bytes) };
    }

    fn not_assign(self: &mut Self) {
        let words = (self.len + 63) >> 6;
        let mut ptr = self.ptr as *mut u64;
        for _ in 0..words {
            unsafe {
                *ptr = !*ptr;
                ptr = ptr.add(1);
            }
        }
    }

    fn and_assign(self: &mut Self, rhs: &Self) {
        assert!(self.len == rhs.len);
        let words = (self.len + 63) >> 6;
        let mut ptr1 = self.ptr as *mut u64;
        let mut ptr2 = rhs.ptr;
        for _ in 0..words {
            unsafe {
                *ptr1 &= *ptr2;
                ptr1 = ptr1.add(1);
                ptr2 = ptr2.add(1);
            }
        }
    }

    fn or_assign(self: &mut Self, rhs: &Self) {
        assert!(self.len == rhs.len);
        let words = (self.len + 63) >> 6;
        let mut ptr1 = self.ptr as *mut u64;
        let mut ptr2 = rhs.ptr;
        for _ in 0..words {
            unsafe {
                *ptr1 |= *ptr2;
                ptr1 = ptr1.add(1);
                ptr2 = ptr2.add(1);
            }
        }
    }

    fn __slow_get__(self: &Self, index: usize) -> bool {
        assert!(index < self.len);
        let word = index >> 6;
        let mask = 1 << ((index as u32) & 63);
        unsafe {
            let ptr = self.ptr.add(word);
            (*ptr & mask) != 0
        }
    }

    fn __slow_set__(self: &Self, index: usize, elem: bool) {
        assert!(index < self.len);
        let word = index >> 6;
        let mask = 1 << ((index as u32) & 63);
        unsafe {
            let ptr = self.ptr.add(word) as *mut u64;
            if elem {
                *ptr |= mask;
            } else {
                *ptr &= !mask;
            }
        }
    }
}

impl Packed {
    pub fn count_ones(self: &Self) -> usize {
        let words = self.len >> 6;
        let mut ptr = self.ptr;
        let mut count = 0;

        for _ in 0..words {
            unsafe {
                count += (*ptr).count_ones() as usize;
                ptr = ptr.add(1);
            }
        }

        let bits = (self.len as u32) & 63;
        if bits != 0 {
            let mask = !(!0u64).wrapping_shl(bits);
            unsafe {
                count += ((*ptr) & mask).count_ones() as usize;
            }
        }

        count
    }

    pub fn count_zeros(self: &Self) -> usize {
        self.len - self.count_ones()
    }
}
