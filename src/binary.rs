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
use super::genvec::Vector as _;

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
    type Elem = genvec::VectorFor<ALG::Elem>;

    fn len(elem: &Self::Elem) -> usize {
        elem.len()
    }

    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem {
        elem.iter().map(|a| self.bool_lift(*a)).collect()
    }

    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        elem.iter().map(|a| self.bool_not(a)).collect()
    }

    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        elem1
            .iter()
            .zip(elem2.iter())
            .map(|(a, b)| self.bool_or(a, b))
            .collect()
    }

    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        elem1
            .iter()
            .zip(elem2.iter())
            .map(|(a, b)| self.bool_and(a, b))
            .collect()
    }

    fn bit_xor(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        elem1
            .iter()
            .zip(elem2.iter())
            .map(|(a, b)| self.bool_xor(a, b))
            .collect()
    }

    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        elem1
            .iter()
            .zip(elem2.iter())
            .map(|(a, b)| self.bool_equ(a, b))
            .collect()
    }

    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        elem1
            .iter()
            .zip(elem2.iter())
            .map(|(a, b)| self.bool_leq(a, b))
            .collect()
    }

    fn concat(self: &Self, elems: &[&Self::Elem]) -> Self::Elem {
        let size = elems.iter().map(|a| a.len()).sum();
        let mut result: Self::Elem = genvec::Vector::with_capacity(size);
        for elem in elems {
            result.extend(elem.iter());
        }
        result
    }

    fn num_lift(self: &Self, len: usize, elem: i64) -> Self::Elem {
        (0..len)
            .map(|i| self.bool_lift((elem >> i) & 1 != 0))
            .collect()
    }

    fn num_neg(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        let mut carry = self.bool_unit();
        elem.iter()
            .map(|a| {
                let b = self.bool_not(a);
                let c = self.bool_xor(b, carry);
                carry = self.bool_and(b, carry);
                c
            })
            .collect()
    }

    fn num_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        let mut carry = self.bool_zero();
        elem1
            .iter()
            .zip(elem2.iter())
            .map(|(a, b)| {
                let c = self.bool_sum3(a, b, carry);
                carry = self.bool_maj(a, b, carry);
                c
            })
            .collect()
    }

    fn num_sub(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        let mut carry = self.bool_unit();
        elem1
            .iter()
            .zip(elem2.iter())
            .map(|(a, b)| {
                let b = self.bool_not(b);
                let c = self.bool_sum3(a, b, carry);
                carry = self.bool_maj(a, b, carry);
                c
            })
            .collect()
    }

    fn num_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        let mut result = self.bool_unit();
        for (a, b) in elem1.iter().zip(elem2.iter()) {
            let c = self.bool_equ(a, b);
            result = self.bool_and(result, c);
        }
        genvec::Vector::from_elem1(result)
    }

    fn num_neq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let mut elem = self.num_equ(elem1, elem2);
        elem.set(0, self.bool_not(elem.get(0)));
        elem
    }

    fn num_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert_eq!(elem1.len(), elem2.len());
        let mut result = self.bool_unit();
        for (a, b) in elem1.iter().zip(elem2.iter()) {
            let a = self.bool_not(a);
            result = self.bool_maj(a, b, result);
        }
        genvec::Vector::from_elem1(result)
    }

    fn num_lth(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let mut elem = self.num_leq(elem2, elem1);
        elem.set(0, self.bool_not(elem.get(0)));
        elem
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
    fn bit_get_value(self: &Self, elem: Self::Elem) -> genvec::VectorFor<bool>;
}

impl<ALG> BinarySat for ALG
where
    ALG: boolean::BoolSat,
{
    fn bit_add_variable(self: &mut Self, len: usize) -> Self::Elem {
        // TODO: implement bulk variable addition
        (0..len).map(|_| self.bool_add_variable()).collect()
    }

    fn bit_add_clause(self: &mut Self, elem: Self::Elem) {
        // let vec: Vec<ALG::Elem> = elem.iter().collect();
        // self.bool_add_clause(elem.iter());
    }

    fn bit_find_model(self: &mut Self, elem: Self::Elem) -> bool {
        // self.bool_find_model()
        false
    }

    fn bit_get_value(self: &Self, elem: Self::Elem) -> genvec::VectorFor<bool> {
        genvec::Vector::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opers() {
        let alg = Trivial();
        let v1 = alg.num_lift(3, 13);
        assert_eq!(v1, (0..3).map(|_| ()).collect());

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
