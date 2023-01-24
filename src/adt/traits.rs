/*
* Copyright (C) 2022, Miklos Maroti
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

use super::{BooleanAlgebra, BooleanSolver, GenVec, VecFor};

/// An arbitrary set of elements that can be representable by bit vectors.
pub trait Domain: Clone {
    /// Returns the number of bits used to represent the elements of this
    /// domain.
    fn num_bits(&self) -> usize;

    /// Verifies that the given bit vector is encoding a valid element of
    /// this domain.
    /// TODO: switch the elem to a generic vec, but we need generic slices.
    fn contains<ALG>(&self, alg: &mut ALG, elem: &[ALG::Elem]) -> ALG::Elem
    where
        ALG: BooleanAlgebra;

    /// Adds a new variable to the given solver, which is just a list of
    /// fresh literals.
    fn add_variable<ALG>(&self, alg: &mut ALG) -> VecFor<ALG::Elem>
    where
        ALG: BooleanSolver,
    {
        let mut elem: VecFor<ALG::Elem> = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.num_bits() {
            elem.push(alg.bool_add_variable());
        }
        elem
    }
}

/// A domain where the elements can be counted and indexed.
pub trait Countable: Domain {
    /// Returns the number of elements of the domain.
    fn count(&self) -> usize;

    /// Returns the given element of the domain.
    fn elem(&self, index: usize) -> VecFor<bool>;

    /// Returns the index of the given element.
    fn index(&self, elem: &VecFor<bool>) -> usize;
}
