/*
* Copyright (C) 2019-2020, Miklos Maroti
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

use super::tensor::TensorAlg;

pub trait RelationAlg {
    /// The type representing the relations.
    type Elem: Clone;

    /// Returns the arity of the relation.
    fn arity(elem: &Self::Elem) -> usize;

    /// Returns the diagonal binary relation.
    fn diagonal(self: &mut Self) -> Self::Elem;

    /// Intersection of a pair of relations of the same arity.
    fn intersection(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Complement (negation) of the given relation.
    fn complement(self: &mut Self, elem: &Self::Elem) -> Self::Elem;

    /// Union of a pair of relations of the same arity.
    fn union(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let elem1 = self.complement(elem1);
        let elem2 = self.complement(elem2);
        let elem3 = self.intersection(&elem1, &elem2);
        self.complement(&elem3)
    }
}

pub struct Relations<ALG>
where
    ALG: TensorAlg,
{
    alg: ALG,
    size: usize,
}

impl<ALG> Relations<ALG>
where
    ALG: TensorAlg,
{
    pub fn is_member(self: &Self, elem: &ALG::Elem) -> bool {
        ALG::shape(elem).is_rectangular(self.size)
    }
}

impl<ALG> RelationAlg for Relations<ALG>
where
    ALG: TensorAlg,
{
    type Elem = ALG::Elem;

    fn arity(elem: &Self::Elem) -> usize {
        ALG::shape(elem).len()
    }

    fn diagonal(self: &mut Self) -> Self::Elem {
        self.alg.diagonal(self.size)
    }

    fn intersection(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(self.is_member(elem1) && ALG::shape(elem1) == ALG::shape(elem2));
        self.alg.tensor_and(elem1, elem2)
    }

    fn complement(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        assert!(self.is_member(elem));
        self.alg.tensor_not(elem)
    }

    fn union(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(self.is_member(elem1) && ALG::shape(elem1) == ALG::shape(elem2));
        self.alg.tensor_or(elem1, elem2)
    }
}
