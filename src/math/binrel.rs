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

#![allow(dead_code)]

use crate::tensor::{Shape, Tensor, TensorAlg};

/// Creates a tensor of shape `[size, size]` representing the
/// binary less than or equal relation of the crown.
pub fn crown_poset(size: usize) -> Tensor<bool> {
    assert!(size >= 4 && size % 2 == 0);
    Tensor::create(Shape::new(vec![size, size]), |i| {
        if i[0] % 2 == 1 {
            i[0] == i[1]
        } else if i[0] == 0 {
            i[1] <= 1 || i[1] == size - 1
        } else {
            i[1] >= i[0] - 1 && i[1] <= i[0] + 1
        }
    })
}

/// Creates the diagonal relation of shape `[size, size]`.
pub fn diagonal(size: usize) -> Tensor<bool> {
    Tensor::create(Shape::new(vec![size, size]), |i| i[0] == i[1])
}

/// Creates the less than relation of shape `[size, size]`.
pub fn less_than(size: usize) -> Tensor<bool> {
    Tensor::create(Shape::new(vec![size, size]), |i| i[0] < i[1])
}

/// Returns an almost empty binary relation except for the edge from
/// `pos[0]` to `pos[1]`.
pub fn singleton(size: usize, pos: [usize; 2]) -> Tensor<bool> {
    Tensor::create(Shape::new(vec![size, size]), |i| {
        i[0] == pos[0] && i[1] == pos[1]
    })
}

/// Creates a partial map from the source to the target universe.
pub fn partial_map(map: &[isize], target: usize) -> Tensor<bool> {
    let shape = Shape::new(vec![map.len(), target]);
    Tensor::create(shape, |i| map[i[0]] == i[1] as isize)
}

/// Returns the list of edges of the binary relation.
pub fn edges(rel: &Tensor<bool>) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 0..rel.shape()[0] {
        for j in 0..rel.shape()[1] {
            if rel.very_slow_get(&[i, j]) {
                edges.push((i, j));
            }
        }
    }
    edges
}

pub trait BinaryRel: TensorAlg {
    /// Checks if the given tensor of shape `[a, b]` is a mapping from an
    /// a-element set to a b-element set, and returns the result in a tensor
    /// of shape `[]`.
    fn is_function(&mut self, fun: Self::Elem) -> Self::Elem {
        let fun = self.transpose(fun);
        let fun = self.tensor_one(fun);
        self.tensor_all(fun)
    }

    /// Checks if the given tensor of shape `[a, b]` is a surjective
    /// mapping from an a-element set to a b-element set, and returns
    /// the result in a tensor of shape `[]`.
    fn is_surjective(&mut self, fun: Self::Elem) -> Self::Elem {
        let fun = self.tensor_any(fun);
        self.tensor_all(fun)
    }

    /// Checks if the binary relation of shape `[a, a]` is reflexive
    /// and returns the result in a tensor of shape `[]`.
    fn is_reflexive(&mut self, rel: Self::Elem) -> Self::Elem {
        let (n, shape) = self.shape(&rel).split1();
        assert_eq!(n, shape[0]);
        let mapping: Vec<usize> = std::iter::once(0).chain(0..shape.len()).collect();
        let rel = self.tensor_polymer(rel, shape, &mapping);
        self.tensor_all(rel)
    }

    /// Composes a binary relation of shape `[a, b]` with another one with
    /// shape `[b, c]` and returns their composition as a relation of shape
    /// `[a, c]`.
    fn compose(&mut self, rel0: Self::Elem, rel1: Self::Elem) -> Self::Elem {
        let (a, b0, rest0) = self.shape(&rel0).split2();
        let (b1, c, rest1) = self.shape(&rel1).split2();
        assert_eq!(b0, b1);
        assert_eq!(rest0, rest1);
        let shape = rest0.join(&[b0, a, c]);
        let rel0 = self.tensor_polymer(rel0, shape.clone(), &[1, 0]);
        let rel1 = self.tensor_polymer(rel1, shape, &[0, 2]);
        let rel2 = self.tensor_and(rel0, rel1);
        self.tensor_any(rel2)
    }

    /// Checks if the first tensor of shape `[a, b]` is a subset of another one
    /// of the same shape, and returns the result as a tensor of shape `[]`.
    fn is_subset_of(&mut self, rel0: Self::Elem, rel1: Self::Elem) -> Self::Elem {
        let rel2 = self.tensor_imp(rel0, rel1);
        let rel2 = self.tensor_all(rel2);
        self.tensor_all(rel2)
    }

    /// Checks if the first tensor of shape `[a, b]` is a proper subset of
    /// the other one of the same shape, and returns the result as a tensor
    /// of shape `[]`.
    fn is_proper_subset_of(&mut self, rel0: Self::Elem, rel1: Self::Elem) -> Self::Elem {
        let tmp1 = self.is_subset_of(rel0.clone(), rel1.clone());
        let tmp2 = self.is_not_equal_to(rel0, rel1);
        self.tensor_and(tmp1, tmp2)
    }

    /// Checks if the binary relation of shape `[a, a]` is transitive
    /// and returns the result in a tensor of shape `[]`.
    fn is_transitive(&mut self, rel: Self::Elem) -> Self::Elem {
        let tmp = self.compose(rel.clone(), rel.clone());
        self.is_subset_of(tmp, rel)
    }

    /// Returns the transpose of the binary relation of shape `[a, b]`
    /// as a tensor of shape `[b, a]`.
    fn transpose(&mut self, rel: Self::Elem) -> Self::Elem {
        let (a, b, shape) = self.shape(&rel).split2();
        let shape = shape.join(&[b, a]);
        self.tensor_polymer(rel, shape, &[1, 0])
    }

    /// Checks if the binary relation of shape `[a, a]` is symmetric
    /// and returns the result in a tensor of shape `[]`.
    fn is_symmetric(&mut self, rel: Self::Elem) -> Self::Elem {
        let tmp = self.transpose(rel.clone());
        let tmp = self.tensor_imp(tmp, rel);
        let tmp = self.tensor_all(tmp);
        self.tensor_all(tmp)
    }

    /// Checks if the binary relation of shape `[a, a]` is anti-symmetric
    /// and returns the result in a tensor of shape `[]`.
    fn is_antisymmetric(&mut self, rel: Self::Elem) -> Self::Elem {
        let size = self.shape(&rel)[0];
        let tmp = diagonal(size);
        let tmp = self.tensor_lift(tmp);
        let rel = self.tensor_not(rel);
        let tmp = self.tensor_or(tmp, rel.clone());
        let rel = self.transpose(rel);
        let tmp = self.tensor_or(tmp, rel);
        let tmp = self.tensor_all(tmp);
        self.tensor_all(tmp)
    }

    /// Checks if the binary relation of shape `[a, a]` is a partial
    /// order and returns the result as a tensor of shape `[]`.
    fn is_partial_order(&mut self, rel: Self::Elem) -> Self::Elem {
        let tmp1 = self.is_reflexive(rel.clone());
        let tmp2 = self.is_transitive(rel.clone());
        let tmp3 = self.is_antisymmetric(rel);
        let tmp4 = self.tensor_and(tmp1, tmp2);
        self.tensor_and(tmp3, tmp4)
    }

    /// Checks if the binary relation of shape `[a, a]` is an equivalence
    /// relation and returns the result as a tensor of shape `[]`.
    fn is_equivalence(&mut self, rel: Self::Elem) -> Self::Elem {
        let tmp1 = self.is_reflexive(rel.clone());
        let tmp2 = self.is_transitive(rel.clone());
        let tmp3 = self.is_symmetric(rel);
        let tmp4 = self.tensor_and(tmp1, tmp2);
        self.tensor_and(tmp3, tmp4)
    }

    /// Removes reflexive and transitive edges. Takes a binary relation of
    /// shape `[a,a]` and returns another of the same shape.
    fn covers(&mut self, rel: Self::Elem) -> Self::Elem {
        let size = self.shape(&rel)[0];
        let tmp = diagonal(size);
        let tmp = self.tensor_lift(tmp);
        let tmp = self.tensor_not(tmp);
        let rel = self.tensor_and(rel, tmp);
        let tmp = self.compose(rel.clone(), rel.clone());
        let tmp = self.tensor_not(tmp);
        self.tensor_and(rel, tmp)
    }

    /// Takes a function of shape `[a, b]`, and a pair of binary relations of
    /// shapes `[a, a]` and `[b, b]` and checks if the function is a compatible
    /// map from the first relation to the second. The result is of shape `[]`.
    fn is_compatible(&mut self, fun: Self::Elem, rel0: Self::Elem, rel1: Self::Elem) -> Self::Elem {
        let tmp = self.compose(rel0, fun.clone());
        let fun = self.transpose(fun);
        let tmp = self.compose(fun, tmp);
        let tmp = self.tensor_imp(tmp, rel1);
        let tmp = self.tensor_all(tmp);
        self.tensor_all(tmp)
    }

    /// Takes two binary relations of shape `[a, b]` and checks if they
    /// are equal, and the result is returned as a tensor of shape `[]`.
    fn is_equal_to(&mut self, rel0: Self::Elem, rel1: Self::Elem) -> Self::Elem {
        let tmp = self.tensor_equ(rel0, rel1);
        let tmp = self.tensor_all(tmp);
        self.tensor_all(tmp)
    }

    /// Takes two binary relations of shape `[a, b]` and checks if they
    /// are different, and the result is returned as a tensor of shape `[]`.
    fn is_not_equal_to(&mut self, rel0: Self::Elem, rel1: Self::Elem) -> Self::Elem {
        let tmp = self.is_equal_to(rel0, rel1);
        self.tensor_not(tmp)
    }
}

impl<ALG> BinaryRel for ALG where ALG: TensorAlg {}
