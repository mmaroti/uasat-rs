/*
* Copyright (C) 2020, Miklos Maroti
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

use super::boolean;
use super::genvec;
use super::genvec::GenVec as _;

pub use boolean::{Boolean, Solver, Trivial};

/// Boolean array algebra representing bit vectors and binary numbers.
pub trait BinaryAlg {
    type Elem;

    /// Returns the length of the array.
    fn len(elem: &Self::Elem) -> usize;

    /// Creates a new vector of the given length containing the element.
    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem;

    /// Returns the element wise negation of the vector.
    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are disjunctions of the original
    /// elements.
    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are conjunction of the original
    /// elements.
    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the exclusive or of the
    /// original elements.
    fn bit_xor(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the are the logical equivalence
    /// of the original elements.
    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the are the logical implication
    /// of the original elements.
    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Concatenates the given vectors into a single one.
    fn concat(self: &Self, elems: &[&Self::Elem]) -> Self::Elem;

    /// Creates a new vector of the given length representing the given binary
    /// number.
    fn num_lift(self: &Self, len: usize, elem: i64) -> Self::Elem;

    /// Returns the negative of the given binary number in two's complement.
    fn num_neg(self: &mut Self, elem: &Self::Elem) -> Self::Elem;

    /// Returns the sum of the two binary numbers of the same length in
    /// two's complement.
    fn num_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns the difference of the two binary numbers of the same length in
    /// two's complement.
    fn num_sub(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns whether the first binary number is equal to the second one
    /// as a 1-element vector.
    fn num_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns whether the first binary number is not equal to the second one
    /// as a 1-element vector.
    fn num_neq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let temp = self.num_equ(elem1, elem2);
        self.bit_not(&temp)
    }

    /// Returns whether the first unsigned binary number is less than or equal
    /// to the second one as a 1-element vector.
    fn num_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns whether the first unsigned binary number is less than the
    /// second one as a 1-element vector.
    fn num_lth(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let temp = self.num_leq(elem2, elem1);
        self.bit_not(&temp)
    }
}

impl<ALG> BinaryAlg for ALG
where
    ALG: boolean::BoolAlg,
{
    type Elem = <ALG::Elem as genvec::GenElem>::Vector;

    fn len(elem: &Self::Elem) -> usize {
        elem.len()
    }

    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem {
        genvec::GenVec::from_fn(elem.len(), |i| self.bool_lift(elem[i]))
    }

    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        genvec::GenVec::from_fn(elem.len(), |i| self.bool_not(elem.get(i)))
    }

    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        genvec::GenVec::from_fn(elem1.len(), |i| self.bool_or(elem1.get(i), elem2.get(i)))
    }

    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        genvec::GenVec::from_fn(elem1.len(), |i| self.bool_and(elem1.get(i), elem2.get(i)))
    }

    fn bit_xor(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        genvec::GenVec::from_fn(elem1.len(), |i| self.bool_xor(elem1.get(i), elem2.get(i)))
    }

    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        genvec::GenVec::from_fn(elem1.len(), |i| self.bool_equ(elem1.get(i), elem2.get(i)))
    }

    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        genvec::GenVec::from_fn(elem1.len(), |i| self.bool_leq(elem1.get(i), elem2.get(i)))
    }

    fn concat(self: &Self, elems: &[&Self::Elem]) -> Self::Elem {
        let size = elems.iter().fold(0, |sum, elem| sum + elem.len());
        let mut result: Self::Elem = genvec::GenVec::with_capacity(size);
        for elem in elems {
            result.extend(elem);
        }
        result
    }

    fn num_lift(self: &Self, len: usize, elem: i64) -> Self::Elem {
        genvec::GenVec::from_fn(len, |i| self.bool_lift((elem >> i) & 1 != 0))
    }

    fn num_neg(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        let mut carry = self.bool_unit();
        let mut result: Self::Elem = genvec::GenVec::with_capacity(elem.len());
        for i in 0..elem.len() {
            let not_elem = self.bool_not(elem.get(i));
            result.push(self.bool_xor(not_elem, carry));
            carry = self.bool_and(not_elem, carry);
        }
        result
    }

    fn num_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        let mut carry = self.bool_zero();
        let mut result: Self::Elem = genvec::GenVec::with_capacity(elem1.len());
        for i in 0..elem1.len() {
            result.push(self.bool_ad3(elem1.get(i), elem2.get(i), carry));
            carry = self.bool_maj(elem1.get(i), elem2.get(i), carry);
        }
        result
    }

    fn num_sub(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        let mut carry = self.bool_unit();
        let mut result: Self::Elem = genvec::GenVec::with_capacity(elem1.len());
        for i in 0..elem1.len() {
            let not_elem2 = self.bool_not(elem2.get(i));
            result.push(self.bool_ad3(elem1.get(i), not_elem2, carry));
            carry = self.bool_maj(elem1.get(i), not_elem2, carry);
        }
        result
    }

    fn num_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        let mut result = self.bool_unit();
        for i in 0..elem1.len() {
            let temp = self.bool_equ(elem1.get(i), elem2.get(i));
            result = self.bool_and(result, temp);
        }
        let mut vec: Self::Elem = genvec::GenVec::with_capacity(1);
        vec.push(result);
        vec
    }

    fn num_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        let mut result = self.bool_unit();
        for i in 0..elem1.len() {
            let not_elem1 = self.bool_not(elem1.get(i));
            result = self.bool_maj(not_elem1, elem2.get(i), result);
        }
        let mut vec: Self::Elem = genvec::GenVec::with_capacity(1);
        vec.push(result);
        vec
    }
}

/// Constraint solving over a boolean algebra.
pub trait BinarySat: BinaryAlg {
    /// Adds a new bit vector variable to the solver
    fn bit_add_variable(self: &mut Self, len: usize) -> Self::Elem;

    /// Adds the given (disjunctive) clause of bits to the solver.
    fn bit_add_clause(self: &mut Self, elem: Self::Elem);

    /// Runs the solver and finds a model where the given bit assumptions
    /// are all true.
    fn bit_find_model(self: &mut Self, elem: Self::Elem) -> bool;

    /// Returns the logical value of the element in the found model.
    fn bit_get_value(self: &Self, elem: Self::Elem) -> <bool as genvec::GenElem>::Vector;
}

impl<ALG> BinarySat for ALG
where
    ALG: boolean::BoolSat,
{
    fn bit_add_variable(self: &mut Self, len: usize) -> Self::Elem {
        // TODO: implement bulk variable addition
        genvec::GenVec::from_fn(len, |_| self.bool_add_variable())
    }

    fn bit_add_clause(self: &mut Self, elem: Self::Elem) {
        // let vec: Vec<ALG::Elem> = elem.iter().collect();
        // self.bool_add_clause(elem.iter());
    }

    fn bit_find_model(self: &mut Self, elem: Self::Elem) -> bool {
        // self.bool_find_model()
        false
    }

    fn bit_get_value(self: &Self, elem: Self::Elem) -> <bool as genvec::GenElem>::Vector {
        genvec::GenVec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opers() {
        let alg = Trivial();
        let v1 = alg.num_lift(3, 13);
        assert_eq!(v1, 3);

        let mut alg = Boolean();
        for a1 in 0..15 {
            let a2 = alg.num_lift(4, a1);
            assert_eq!(a2, alg.num_lift(4, a1 - 16));
            assert_eq!(alg.bit_not(&a2), alg.num_lift(4, !a1));
            assert_eq!(alg.num_neg(&a2), alg.num_lift(4, -a1));
            assert_eq!(alg.concat(&[&a2]), a2);

            for b1 in 0..15 {
                let b2 = alg.num_lift(4, b1);
                assert_eq!(alg.bit_and(&a2, &b2), alg.num_lift(4, a1 & b1));
                assert_eq!(alg.bit_or(&a2, &b2), alg.num_lift(4, a1 | b1));
                assert_eq!(alg.bit_xor(&a2, &b2), alg.num_lift(4, a1 ^ b1));
                assert_eq!(alg.bit_equ(&a2, &b2), alg.num_lift(4, !a1 ^ b1));
                assert_eq!(alg.bit_leq(&a2, &b2), alg.num_lift(4, !a1 | b1));

                assert_eq!(alg.num_add(&a2, &b2), alg.num_lift(4, a1 + b1));
                assert_eq!(alg.num_sub(&a2, &b2), alg.num_lift(4, a1 - b1));
                assert_eq!(alg.num_equ(&a2, &b2), alg.bit_lift(&[a1 == b1]));
                assert_eq!(alg.num_neq(&a2, &b2), alg.bit_lift(&[a1 != b1]));
                assert_eq!(alg.num_leq(&a2, &b2), alg.bit_lift(&[a1 <= b1]));
                assert_eq!(alg.num_lth(&a2, &b2), alg.bit_lift(&[a1 < b1]));

                assert_eq!(alg.concat(&[&a2, &b2]), alg.num_lift(8, a1 + 16 * b1));
            }
        }
    }
}
