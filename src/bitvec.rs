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

//! A minimalistic bit vector for specializing the generic vector for booleans.

/// Generic interface for regular and bit vectors.
pub trait GenVec<T: Copy>
where
    Self: Default,
{
    /// Constructs a new, empty vector. The vector will not allocate until
    /// elements are pushed onto it.
    fn new() -> Self;

    /// Constructs a new, empty vector with the specified capacity. The vector
    /// will be able to hold exactly capacity elements without reallocating.
    fn with_capacity(capacity: usize) -> Self;

    /// Clears the vector, removing all values.
    fn clear(self: &mut Self);

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `value`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    fn resize(self: &mut Self, new_len: usize, value: T);

    /// Appends an element to the back of a collection.
    fn push(self: &mut Self, value: T);

    /// Removes the last element from a vector and returns it, or `None` if
    /// it is empty.
    fn pop(self: &mut Self) -> Option<T>;

    /// Returns the element at the given index.
    fn get(self: &Self, index: usize) -> T;

    /// Sets the element at the given index to the new value.
    fn set(self: &mut Self, index: usize, value: T);

    /// Returns the number of elements in the vector.
    fn len(self: &Self) -> usize;

    /// Returns `true` if the vector contains no elements.
    fn is_empty(self: &Self) -> bool;

    /// Returns the number of elements the vector can hold without reallocating.
    fn capacity(self: &Self) -> usize;
}

impl<T: Copy> GenVec<T> for Vec<T> {
    #[inline]
    fn new() -> Self {
        Vec::new()
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    #[inline]
    fn clear(self: &mut Self) {
        Vec::clear(self);
    }

    #[inline]
    fn resize(self: &mut Self, new_len: usize, value: T) {
        Vec::resize(self, new_len, value);
    }

    #[inline]
    fn push(self: &mut Self, value: T) {
        Vec::push(self, value);
    }

    #[inline]
    fn pop(self: &mut Self) -> Option<T> {
        Vec::pop(self)
    }

    #[inline]
    fn get(self: &Self, index: usize) -> T {
        self[index]
    }

    #[inline]
    fn set(self: &mut Self, index: usize, value: T) {
        self[index] = value;
    }

    #[inline]
    fn len(self: &Self) -> usize {
        Vec::len(self)
    }

    #[inline]
    fn is_empty(self: &Self) -> bool {
        Vec::is_empty(self)
    }

    #[inline]
    fn capacity(self: &Self) -> usize {
        Vec::capacity(self)
    }
}

/// A vector containing bools.
#[derive(Debug)]
pub struct BitVec {
    vec: Vec<u32>,
    len: usize,
}

impl GenVec<bool> for BitVec {
    fn new() -> Self {
        BitVec {
            vec: Vec::new(),
            len: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        let c = (capacity + 31) >> 5;
        BitVec {
            vec: Vec::with_capacity(c),
            len: 0,
        }
    }

    fn clear(self: &mut Self) {
        self.vec.clear();
        self.len = 0;
    }

    fn resize(self: &mut Self, new_len: usize, value: bool) {
        if (self.len & 31) != 0 {
            let m = !0 << (self.len & 31);
            if value {
                self.vec[self.len >> 5] |= m;
            } else {
                self.vec[self.len >> 5] &= !m;
            }
        }

        self.vec
            .resize((new_len + 31) >> 5, if value { !0 } else { 0 });

        self.len = new_len;
    }

    #[allow(clippy::verbose_bit_mask)]
    fn push(self: &mut Self, value: bool) {
        if (self.len & 31) == 0 {
            self.vec.push(0);
        }
        if value {
            self.vec[self.len >> 5] |= 1 << (self.len & 31);
        } else {
            self.vec[self.len >> 5] &= !(1 << (self.len & 31));
        }
        self.len += 1;
    }

    #[allow(clippy::verbose_bit_mask)]
    fn pop(self: &mut Self) -> Option<bool> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let b = self.vec[self.len >> 5] & (1 << (self.len & 31)) != 0;
            if (self.len & 31) == 0 {
                self.vec.pop();
            }
            Some(b)
        }
    }

    fn get(self: &Self, index: usize) -> bool {
        assert!(index < self.len);
        self.vec[index >> 5] & (1 << (index & 31)) != 0
    }

    fn set(self: &mut Self, index: usize, value: bool) {
        assert!(index < self.len);
        if value {
            self.vec[index >> 5] |= 1 << (index & 31);
        } else {
            self.vec[index >> 5] &= !(1 << (index & 31));
        }
    }

    fn len(self: &Self) -> usize {
        self.len
    }

    fn is_empty(self: &Self) -> bool {
        self.len == 0
    }

    fn capacity(self: &Self) -> usize {
        self.vec.capacity() * 32
    }
}

impl Default for BitVec {
    fn default() -> Self {
        GenVec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitvec_resize() {
        let mut v1: BitVec = GenVec::new();
        let mut v2: BitVec = GenVec::new();

        for i in 0..50 {
            let b = i % 2 == 0;

            for _ in 0..90 {
                v1.push(b);
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
}
