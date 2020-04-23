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

use super::tensor;

pub struct Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    alg: ALG,
    size: usize,
}

impl<ALG> Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    pub fn is_relation(self: &Self, elem: &ALG::Elem) -> bool {
        ALG::shape(elem).is_rectangular(self.size)
    }

    pub fn is_binary_rel(self: &Self, elem: &ALG::Elem) -> bool {
        ALG::shape(elem).len() == 2 && self.is_relation(elem)
    }

    fn new_shape(self: &Self, len: usize) -> tensor::Shape {
        let mut shape = Vec::with_capacity(len);
        shape.resize(len, self.size);
        tensor::Shape::new(shape)
    }
}

pub trait BinaryRelAlg {
    type Elem: Clone;

    /// Returns the empty or total relation.
    fn binrel_lift(self: &mut Self, elem: bool) -> Self::Elem;

    /// Returns the empty relation.
    fn binrel_empty(self: &mut Self) -> Self::Elem {
        self.binrel_lift(false)
    }

    /// Returns the total relation.
    fn binrel_total(self: &mut Self) -> Self::Elem {
        self.binrel_lift(true)
    }

    /// Returns the diagonal relation.
    fn binrel_diag(self: &mut Self) -> Self::Elem;

    /// Returns the complement of the given relation.
    fn binrel_comp(self: &mut Self, elem: &Self::Elem) -> Self::Elem;

    /// Intersection of a pair of relations.
    fn binrel_meet(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Union of a pair of relations.
    fn binrel_join(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let elem1 = self.binrel_comp(elem1);
        let elem2 = self.binrel_comp(elem2);
        let elem3 = self.binrel_meet(&elem1, &elem2);
        self.binrel_comp(&elem3)
    }

    /// Returns the inverse of the relation.
    fn binrel_inv(self: &mut Self, elem: &Self::Elem) -> Self::Elem;

    /// Returns the composition of a pair of relations.
    fn binrel_circ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;
}

impl<ALG> BinaryRelAlg for Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    type Elem = ALG::Elem;

    fn binrel_lift(self: &mut Self, elem: bool) -> Self::Elem {
        let elem = self.alg.scalar(elem);
        self.alg.polymer(&elem, self.new_shape(2), &[])
    }

    fn binrel_diag(self: &mut Self) -> Self::Elem {
        self.alg.diagonal(self.size)
    }

    fn binrel_comp(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(elem));
        self.alg.tensor_not(elem)
    }

    fn binrel_meet(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(elem1));
        assert_eq!(ALG::shape(elem1), ALG::shape(elem2));
        self.alg.tensor_and(elem1, elem2)
    }

    fn binrel_join(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(elem1));
        assert_eq!(ALG::shape(elem1), ALG::shape(elem2));
        self.alg.tensor_or(elem1, elem2)
    }

    fn binrel_inv(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(elem));
        self.alg.polymer(&elem, self.new_shape(2), &[1, 0])
    }

    fn binrel_circ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let elem1 = self.alg.polymer(elem1, self.new_shape(3), &[1, 0]);
        let elem2 = self.alg.polymer(elem2, self.new_shape(3), &[0, 2]);
        let elem3 = self.alg.tensor_and(&elem1, &elem2);
        self.alg.tensor_any(&elem3)
    }
}

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

impl<ALG> RelationAlg for Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    type Elem = ALG::Elem;

    fn arity(elem: &Self::Elem) -> usize {
        ALG::shape(elem).len()
    }

    fn diagonal(self: &mut Self) -> Self::Elem {
        self.alg.diagonal(self.size)
    }

    fn intersection(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(self.is_relation(elem1));
        assert_eq!(ALG::shape(elem1), ALG::shape(elem2));
        self.alg.tensor_and(elem1, elem2)
    }

    fn complement(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        assert!(self.is_relation(elem));
        self.alg.tensor_not(elem)
    }

    fn union(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(self.is_relation(elem1));
        assert_eq!(ALG::shape(elem1), ALG::shape(elem2));
        self.alg.tensor_or(elem1, elem2)
    }
}
