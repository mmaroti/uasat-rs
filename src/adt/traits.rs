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

use super::{BooleanAlgebra, BooleanSolver};

/// An arbitrary set of elements that can be representable by bit vectors.
pub trait Domain: Clone {
    /// Returns the number of bits used to represent the elements of this
    /// domain.
    fn num_bits(&self) -> usize;

    /// Verifies that the given bit vector is encoding a valid element of
    /// this domain.
    fn contains<ALG>(&self, alg: &mut ALG, elem: &[ALG::Elem]) -> ALG::Elem
    where
        ALG: BooleanAlgebra;

    /// Adds a new variable to the given solver, which is just a list of
    /// fresh literals.
    fn add_variable<ALG>(&self, alg: &mut ALG) -> Box<[ALG::Elem]>
    where
        ALG: BooleanSolver,
    {
        let mut elem = Vec::with_capacity(self.num_bits());
        for _ in 0..self.num_bits() {
            elem.push(alg.bool_add_variable());
        }
        elem.into_boxed_slice()
    }
}

pub trait Finite: Domain {}
