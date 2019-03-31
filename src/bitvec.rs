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

use std::{alloc, ptr, usize};

#[allow(clippy::len_without_is_empty)]
pub trait BoolArray<ELEM: Copy> {
    /**
     * Creates an array with the given length.
     */
    fn new(len: usize) -> Self;

    /**
     * Returns the length of the array.
     */
    fn len(self: &Self) -> usize;

    /**
     * Sets all elements in the array to the given value.
     */
    fn set_all(self: &mut Self, elem: ELEM);

    /**
     * Negates all elements of the array.
     */
    fn neg_assign(self: &mut Self);

    /**
     * Updates this array in place with using the bitwise and operation.
     */
    fn and_assign(self: &mut Self, rhs: &Self);

    /**
     * Updates this array in place with using the bitwise or operation.
     */
    fn or_assign(self: &mut Self, rhs: &Self);

    /**
     * Returns the element at the given index.
     */
    #[allow(non_snake_case)]
    fn __slow_get__(self: &Self, index: usize) -> ELEM;

    /**
     * Sets the element at the given index to a new value.
     */
    #[allow(non_snake_case)]
    fn __slow_set__(self: &Self, index: usize, elem: ELEM);
}

pub struct BitVec {
    ptr: *const u64,
    len: usize, // in bits
}

impl Drop for BitVec {
    fn drop(self: &mut Self) {
        debug_assert!(self.len <= usize::MAX - 63);
        let bytes = (self.len + 63) >> 6 << 3;
        let layout = unsafe { alloc::Layout::from_size_align_unchecked(bytes, 8) };
        unsafe { alloc::dealloc(self.ptr as *mut u8, layout) };
    }
}

impl BoolArray<bool> for BitVec {
    #[allow(clippy::cast_ptr_alignment)]
    fn new(len: usize) -> Self {
        assert!(len <= usize::MAX - 63);
        let bytes = (len + 63) >> 6 << 3;
        let layout = unsafe { alloc::Layout::from_size_align_unchecked(bytes, 8) };
        let ptr = unsafe { alloc::alloc(layout) } as *const u64;
        BitVec { ptr, len }
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

    fn neg_assign(self: &mut Self) {
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

impl BitVec {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_all() {
        for num in 0..100 {
            let mut v = BitVec::new(num);
            assert_eq!(v.len(), num);
            v.set_all(true);
            assert_eq!(v.count_ones(), num);
            assert_eq!(v.count_zeros(), 0);
            v.set_all(false);
            assert_eq!(v.count_ones(), 0);
            v.neg_assign();
            assert_eq!(v.count_ones(), num);
        }
    }

    #[test]
    fn test_slow_set() {
        for num in 1..100 {
            let mut v = BitVec::new(num);
            v.set_all(false);
            for bit in 0..num {
                assert_eq!(v.__slow_get__(bit), false);
                v.__slow_set__(bit, true);
                assert_eq!(v.__slow_get__(bit), true);
                assert_eq!(v.count_ones(), bit + 1);
            }
        }
    }
}
