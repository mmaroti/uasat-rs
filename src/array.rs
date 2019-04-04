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

use std::{cmp, usize};

#[allow(clippy::len_without_is_empty)]
pub trait Array<ELEM: Copy + Default>: Sized {
    /**
     * Returns the length of the array.
     */
    fn len(self: &Self) -> usize;

    /**
     * Creates an array with the given length filled with the given element.
     */
    fn constant(len: usize, elem: ELEM) -> Self;

    /**
     * Creates an array with the given length filled with elements produced
     * by the given generator.
     */
    fn generate(len: usize, mut gen: impl FnMut(usize) -> ELEM) -> Self {
        let mut vec: Self = Array::constant(len, Default::default());
        for idx in 0..len {
            vec.__slow_set__(idx, gen(idx));
        }
        vec
    }

    /**
     * Creates the logical negation of the given array.
     */
    fn not(self: &Self) -> Self;

    /**
     * Creates the logical and of two arrays of the same length.
     */
    fn and(self: &Self, other: &Self) -> Self;

    /**
     * Returns the element at the given index.
     */
    #[allow(non_snake_case)]
    fn __slow_get__(self: &Self, index: usize) -> ELEM;

    /**
     * Sets the element at the given index to a new value.
     */
    #[allow(non_snake_case)]
    fn __slow_set__(self: &mut Self, index: usize, elem: ELEM);
}

#[derive(Debug)]
pub struct Bits {
    vec: Vec<u32>,
    len: usize,
}

impl Array<bool> for Bits {
    fn len(self: &Self) -> usize {
        self.len
    }

    fn constant(len: usize, elem: bool) -> Self {
        assert!(len <= usize::MAX - 31);
        let mut vec = Vec::new();
        vec.resize((len + 31) >> 5, if elem { !0 } else { 0 });
        Bits { vec, len }
    }

    fn generate(len: usize, mut gen: impl FnMut(usize) -> bool) -> Self {
        assert!(len <= usize::MAX - 31);
        let mut vec = Vec::with_capacity((len + 31) >> 5);
        let mut idx = 0;
        while idx < len {
            let mut word = 0;
            for bit in 0..cmp::min(32, len - idx) {
                word |= (gen(idx) as u32) << bit;
                idx += 1;
            }
            vec.push(word);
        }
        debug_assert!(vec.len() == (len + 31) >> 5);
        Bits { vec, len }
    }

    fn not(self: &Self) -> Self {
        let mut vec = Vec::with_capacity(self.vec.len());
        for word in self.vec.iter() {
            vec.push(!*word);
        }
        let len = self.len;
        debug_assert!(vec.len() == (len + 31) >> 5);
        Bits { vec, len }
    }

    fn and(self: &Self, other: &Self) -> Self {
        assert!(self.len == other.len);
        let mut vec = Vec::with_capacity(self.vec.len());
        for (word1, word2) in self.vec.iter().zip(other.vec.iter()) {
            vec.push(*word1 & *word2);
        }
        let len = self.len;
        debug_assert!(vec.len() == (len + 31) >> 5);
        Bits { vec, len }
    }

    fn __slow_get__(self: &Self, index: usize) -> bool {
        assert!(index < self.len);
        let word = self.vec[index >> 5];
        let bit = 1 << (index & 31);
        (word & bit) != 0
    }

    fn __slow_set__(self: &mut Self, index: usize, elem: bool) {
        assert!(index < self.len);
        let word = &mut self.vec[index >> 5];
        let bit = 1 << (index & 31);
        if elem {
            *word |= bit;
        } else {
            *word &= !bit;
        }
    }
}

impl Bits {
    pub fn count_ones(self: &Self) -> usize {
        let mut word: u32 = 0;
        let mut count = 0;
        for word2 in self.vec.iter() {
            count += word.count_ones() as usize;
            word = *word2;
        }
        let mask = (!0u32).wrapping_shr(32 - ((self.len as u32) & 31));
        count + (word & mask).count_ones() as usize
    }

    pub fn count_zeros(self: &Self) -> usize {
        self.len - self.count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_slow_set() {
        for num in 1..100 {
            let mut v = Bits::constant(num, true);
            for bit in 0..num {
                assert_eq!(v.count_ones(), num - bit);
                assert_eq!(v.not().count_ones(), bit);
                assert_eq!(v.__slow_get__(bit), true);
                v.__slow_set__(bit, false);
                assert_eq!(v.__slow_get__(bit), false);
            }
            assert_eq!(v.count_ones(), 0);
        }
    }

    #[test]
    fn bits_generate() {
        for num in 0..100 {
            let v = Bits::generate(num, |idx| idx % 5 == 0);
            assert_eq!(v.count_ones(), (num + 4) / 5);
            assert_eq!(v.len(), num);
        }
    }
}
