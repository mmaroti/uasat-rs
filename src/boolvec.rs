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

use super::boolalg::{BoolAlg, BoolSat};
use super::genvec::{GenElem, GenVec};

/// Boolean array algebra.
pub trait BoolVecAlg {
    type Bool: Copy;
    type Elem: Clone;

    /// Returns the length of the array.
    fn bvec_len(elem: &Self::Elem) -> usize;

    /// Returns the element wise negation of the vector.
    fn bvec_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem;
}

pub struct Checker();

impl BoolVecAlg for Checker {
    type Bool = bool;
    type Elem = usize;

    fn bvec_len(elem: &Self::Elem) -> usize {
        *elem
    }

    fn bvec_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        *elem
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
    type Bool = ALG::Elem;
    type Elem = <ALG::Elem as GenElem>::Vector;

    fn bvec_len(elem: &Self::Elem) -> usize {
        elem.len()
    }

    fn bvec_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        GenVec::from_fn(elem.len(), |i| self.bool_not(elem.get(i)))
    }
}
