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

use super::boolalg::BoolAlg;
use super::genvec::{GenElem, GenVec};

/// Boolean array algebra.
pub trait BoolVecAlg {
    type Elem: Clone;

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
    fn bit_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the are the logical equivalence
    /// of the original elements.
    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the are the logical implication
    /// of the original elements.
    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Creates a new vector of the given length representing the given binary
    /// number.
    fn num_lift(self: &Self, len: usize, elem: i64) -> Self::Elem;
}

pub type Checker = ();

fn checker_binop(elem1: &usize, elem2: &usize) -> usize {
    assert!(*elem1 == *elem2);
    *elem1
}

impl BoolVecAlg for Checker {
    type Elem = usize;

    fn len(elem: &Self::Elem) -> usize {
        *elem
    }

    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem {
        elem.len()
    }

    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        *elem
    }

    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        checker_binop(elem1, elem2)
    }

    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        checker_binop(elem1, elem2)
    }

    fn bit_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        checker_binop(elem1, elem2)
    }

    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        checker_binop(elem1, elem2)
    }

    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        checker_binop(elem1, elem2)
    }

    fn num_lift(self: &Self, len: usize, _elem: i64) -> Self::Elem {
        len
    }
}

fn bvec_binop<ALG, OP>(
    alg: &mut ALG,
    elem1: &<ALG::Elem as GenElem>::Vector,
    elem2: &<ALG::Elem as GenElem>::Vector,
    mut op: OP,
) -> <ALG::Elem as GenElem>::Vector
where
    ALG: BoolAlg,
    ALG::Elem: GenElem,
    OP: FnMut(&mut ALG, ALG::Elem, ALG::Elem) -> ALG::Elem,
{
    assert!(elem1.len() == elem2.len());
    GenVec::from_fn(elem1.len(), |i| op(alg, elem1.get(i), elem2.get(i)))
}

impl<ALG> BoolVecAlg for ALG
where
    ALG: BoolAlg,
    ALG::Elem: GenElem,
{
    type Elem = <ALG::Elem as GenElem>::Vector;

    fn len(elem: &Self::Elem) -> usize {
        elem.len()
    }

    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem {
        GenVec::from_fn(elem.len(), |i| self.bool_lift(elem[i]))
    }

    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        GenVec::from_fn(elem.len(), |i| self.bool_not(elem.get(i)))
    }

    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        bvec_binop(self, elem1, elem2, BoolAlg::bool_or)
    }

    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        bvec_binop(self, elem1, elem2, BoolAlg::bool_and)
    }

    fn bit_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        bvec_binop(self, elem1, elem2, BoolAlg::bool_add)
    }

    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        bvec_binop(self, elem1, elem2, BoolAlg::bool_equ)
    }

    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        bvec_binop(self, elem1, elem2, BoolAlg::bool_leq)
    }

    fn num_lift(self: &Self, len: usize, elem: i64) -> Self::Elem {
        GenVec::from_fn(len, |i| self.bool_lift((elem >> i) & 1 != 0))
    }
}

#[cfg(test)]
mod tests {
    use super::super::boolalg::Boolean;
    use super::*;

    #[test]
    fn num_lift() {
        let alg = Boolean::new();
        let v1 = alg.bit_lift(&[true, false, true, true]);
        let v2 = alg.num_lift(4, 13);
        assert_eq!(v1, v2);

        let v3 = alg.num_lift(4, -3);
        assert_eq!(v1, v3);
    }
}
