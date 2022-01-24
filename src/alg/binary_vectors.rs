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

use super::{
    AdditiveGroup, BooleanAlgebra, BoundedPartialOrder, DirectedGraph, Domain, FreeBooleanAlg,
    Lattice, Monoid, PartialOrder, Ring, Semigroup, UnitaryRing,
};

/// A finite power of a boolean algebra. The elements are represented as vectors of boolean
/// values backed by the underlying logic. All operations are acting coordinate-wise, the
/// order is the natural order on the power.
#[derive(Debug)]
pub struct BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    length: usize,
    logic: &'a L,
}

impl<'a, L> BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    pub fn new(length: usize, logic: &'a L) -> Self {
        Self { length, logic }
    }
}

impl<'a> BinaryVectors<'a, FreeBooleanAlg> {
    /// Returns a new variable to the solver.
    pub fn add_variable(&self) -> Vec<<FreeBooleanAlg as Domain>::Elem> {
        (0..self.length)
            .map(|_| self.logic.add_generator())
            .collect()
    }
}

impl<'a, L> Domain for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    type Elem = Vec<L::Elem>;

    type Logic = L;

    fn logic(&self) -> &Self::Logic {
        self.logic
    }

    fn contains(&self, elem: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        if elem.len() == self.length {
            self.logic.top()
        } else {
            self.logic.bot()
        }
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        assert_eq!(elem0.len(), elem1.len());
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.equ(a, b))
            .fold(self.logic.top(), |a, b| self.logic.meet(&a, &b))
    }
}

impl<'a, L> DirectedGraph for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    fn edge(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Logic as Domain>::Elem {
        assert_eq!(elem0.len(), elem1.len());
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.imp(a, b))
            .fold(self.logic.top(), |a, b| self.logic.meet(&a, &b))
    }
}

impl<'a, L> PartialOrder for BinaryVectors<'a, L> where L: BooleanAlgebra {}

impl<'a, L> BoundedPartialOrder for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    fn top(&self) -> Self::Elem {
        let elem = self.logic.top();
        vec![elem; self.length]
    }

    fn bot(&self) -> Self::Elem {
        let elem = self.logic.bot();
        vec![elem; self.length]
    }
}

impl<'a, L> Lattice for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    fn meet(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        assert_eq!(elem0.len(), elem1.len());
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.meet(a, b))
            .collect()
    }

    fn join(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        assert_eq!(elem0.len(), elem1.len());
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.join(a, b))
            .collect()
    }
}

impl<'a, L> BooleanAlgebra for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    fn not(&self, elem: &Self::Elem) -> Self::Elem {
        assert_eq!(elem.len(), self.length);
        elem.iter().map(|a| self.logic.not(a)).collect()
    }

    fn xor(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        assert_eq!(elem0.len(), elem1.len());
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.xor(a, b))
            .collect()
    }

    fn imp(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        assert_eq!(elem0.len(), elem1.len());
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.imp(a, b))
            .collect()
    }

    fn equ(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        assert_eq!(elem0.len(), elem1.len());
        elem0
            .iter()
            .zip(elem1.iter())
            .map(|(a, b)| self.logic.equ(a, b))
            .collect()
    }
}

impl<'a, L> AdditiveGroup for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    fn zero(&self) -> Self::Elem {
        self.bot()
    }

    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        elem.clone()
    }

    fn add(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.xor(elem0, elem1)
    }
}

impl<'a, L> Semigroup for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    fn mul(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        self.meet(elem0, elem1)
    }
}

impl<'a, L> Monoid for BinaryVectors<'a, L>
where
    L: BooleanAlgebra,
{
    fn unit(&self) -> Self::Elem {
        self.top()
    }
}

impl<'a, L> Ring for BinaryVectors<'a, L> where L: BooleanAlgebra {}

impl<'a, L> UnitaryRing for BinaryVectors<'a, L> where L: BooleanAlgebra {}
