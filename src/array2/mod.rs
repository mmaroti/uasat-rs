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

mod packed;
pub use packed::Packed;

#[allow(clippy::len_without_is_empty)]
pub trait Array<ELEM: Copy> {
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
    fn not_assign(self: &mut Self);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_set_all() {
        for num in 0..100 {
            let mut v = Packed::new(num);
            assert_eq!(v.len(), num);
            v.set_all(true);
            assert_eq!(v.count_ones(), num);
            assert_eq!(v.count_zeros(), 0);
            v.set_all(false);
            assert_eq!(v.count_ones(), 0);
            v.not_assign();
            assert_eq!(v.count_ones(), num);
        }
    }

    #[test]
    fn packed_slow_set() {
        for num in 1..100 {
            let mut v = Packed::new(num);
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
