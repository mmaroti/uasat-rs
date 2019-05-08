/*
* Copyright (C) 2019, Miklos Maroti
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

//! Basic multidimensional array type and operations over boolean algebras.

use super::boolalg::{BoolAlg, FreeAlg, Literal};
use super::genvec::{GenElem, GenVec};
use std::ops::Index;

/// The shape of a tensor.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Shape {
    dims: Vec<usize>,
}

impl Shape {
    /// Creates a new shape object.
    pub fn new(dims: &[usize]) -> Self {
        Shape {
            dims: Vec::from(dims),
        }
    }

    /// The number of dimensions.
    pub fn len(self: &Self) -> usize {
        self.dims.len()
    }

    /// Checks if the number of dimensions is zero.
    pub fn is_empty(self: &Self) -> bool {
        self.dims.is_empty()
    }

    /// Returns the number of elements this shape represents.
    pub fn size(self: &Self) -> usize {
        let mut size = 1;
        for dim in self.dims.iter() {
            size *= *dim;
        }
        size
    }

    /// Returns the linear index of an element given by coordinates.
    fn index(self: &Self, coords: &[usize]) -> usize {
        assert!(self.len() == coords.len());
        let mut index = 0;
        let mut size = 1;
        for (coord, dim) in coords.iter().zip(self.dims.iter()) {
            assert!(coord < dim);
            index += *coord * size;
            size *= *dim;
        }
        index
    }

    /// Returns the vector of strides for linear indexing
    fn strides(self: &Self) -> Vec<usize> {
        let mut strides = Vec::with_capacity(self.dims.len());
        let mut size = 1;
        for dim in self.dims.iter() {
            strides.push(size);
            size *= *dim;
        }
        strides
    }
}

impl Index<usize> for Shape {
    type Output = usize;

    fn index(self: &Self, idx: usize) -> &Self::Output {
        &self.dims[idx]
    }
}

/// Iterator for implementing the polymer operation
struct StrideIter {
    entries: Vec<(usize, usize, usize)>,
    index: usize,
    done: bool,
}

impl StrideIter {
    fn new(shape: &Shape) -> Self {
        let mut entries = Vec::with_capacity(shape.dims.len());
        let mut done = false;
        for dim in shape.dims.iter() {
            entries.push((0, *dim, 0));
            done |= *dim == 0;
        }
        StrideIter {
            entries,
            index: 0,
            done,
        }
    }

    fn add_stride(self: &mut Self, idx: usize, stride: usize) {
        self.entries[idx].2 += stride;
    }
}

impl Iterator for StrideIter {
    type Item = usize;

    fn next(self: &mut Self) -> Option<usize> {
        if !self.done {
            let index = self.index;
            for entry in self.entries.iter_mut() {
                self.index += entry.2;
                entry.0 += 1;
                if entry.0 >= entry.1 {
                    self.index -= entry.0 * entry.2;
                    entry.0 = 0;
                } else {
                    return Some(index);
                }
            }
            self.done = true;
            return Some(index);
        }
        None
    }
}

/// A multidimensional array of elements.
#[derive(Debug)]
pub struct Tensor<Elem: GenElem> {
    shape: Shape,
    elems: Elem::Vector,
}

impl<Elem: GenElem> Tensor<Elem> {
    /// Creates a tensor filled with constant value
    pub fn new(shape: Shape, elem: Elem) -> Self {
        let size = shape.size();
        let mut elems: Elem::Vector = GenVec::with_capacity(size);
        elems.resize(size, elem);
        Tensor { shape, elems }
    }

    /// Returns the shape of the tensor
    pub fn shape(self: &Self) -> &Shape {
        &self.shape
    }

    /// Returns the element at the given index.
    #[allow(non_snake_case)]
    pub fn __slow_get__(self: &Self, coords: &[usize]) -> Elem {
        self.elems.get(self.shape.index(coords))
    }

    /// Sets the element at the given index.
    #[allow(non_snake_case)]
    pub fn __slow_set__(self: &mut Self, coords: &[usize], elem: Elem) {
        self.elems.set(self.shape.index(coords), elem);
    }

    /// Creates a new tensor of the given shape from the given old tensor with
    /// permuted, identified or new dummy coordinates. The mapping is a vector
    /// of length of the original tensor shape with entries identifying the
    /// matching coordinates in the new tensor.
    pub fn polymer(self: &Self, shape: Shape, mapping: &[usize]) -> Self {
        assert!(mapping.len() == self.shape.len());

        let mut iter = StrideIter::new(&shape);
        let strides = self.shape.strides();
        for (idx, val) in mapping.iter().enumerate() {
            assert!(self.shape[idx] == shape[*val]);
            iter.add_stride(*val, strides[idx]);
        }

        let size = shape.size();
        let mut elems: Elem::Vector = GenVec::with_capacity(size);
        for index in iter {
            elems.push(self.elems.get(index));
        }
        assert!(elems.len() == size);

        Tensor { shape, elems }
    }
}

/// A tensor algebra for tensors.
pub trait TensorAlg {
    /// The type representing the tensor.
    type Tensor;

    /// Creates a new scalar tensor for the given element.
    fn scalar(self: &mut Self, elem: bool) -> Self::Tensor;

    /// Returns a diagonal tensor of rank two with true elements on the
    /// diagonal and false everywhere else.
    fn diagonal(self: &mut Self, dim: usize) -> Self::Tensor;

    /// Creates a new tensor of the given shape from the given old tensor with
    /// permuted, identified or new dummy coordinates. The mapping is a vector
    /// of length of the old tensor shape with entries identifying the
    /// coordinate in the new tensor.
    fn polymer(
        self: &mut Self,
        tensor: &Self::Tensor,
        shape: Shape,
        mapping: &[usize],
    ) -> Self::Tensor;

    /// Returns a new tensor whose elements are all negated of the original.
    fn tensor_not(self: &mut Self, tensor: &Self::Tensor) -> Self::Tensor;

    /// Returns a new tensor whose elements are disjunctions of the original
    /// elements.
    fn tensor_or(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor;

    /// Returns a new tensor whose elements are the boolean additions of the
    /// original elements.
    fn tensor_add(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor;

    /// Returns a new tensor whose elements are the conjunctions of the
    /// original elements.
    fn tensor_and(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor;

    /// Returns a new tensor whose elements are the logical equivalence of the
    /// original elements.
    fn tensor_equ(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor;

    /// Returns a new tensor whose elements are the logical implication of the
    /// original elements.
    fn tensor_leq(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor;
}

/// The tensor algebra used for checking the shapes of calculations.
pub struct Checker();

fn checker_binop(tensor1: &Shape, tensor2: &Shape) -> Shape {
    assert!(tensor1 == tensor2);
    tensor1.clone()
}

impl TensorAlg for Checker {
    type Tensor = Shape;

    fn scalar(self: &mut Self, _elem: bool) -> Self::Tensor {
        Shape::new(&[])
    }

    fn diagonal(self: &mut Self, dim: usize) -> Self::Tensor {
        Shape::new(&[dim, dim])
    }

    fn polymer(
        self: &mut Self,
        tensor: &Self::Tensor,
        shape: Shape,
        mapping: &[usize],
    ) -> Self::Tensor {
        assert!(mapping.len() == tensor.len());
        for (idx, val) in mapping.iter().enumerate() {
            assert!(tensor[idx] == shape[*val]);
        }
        shape
    }

    fn tensor_not(self: &mut Self, tensor: &Self::Tensor) -> Self::Tensor {
        tensor.clone()
    }

    fn tensor_or(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        checker_binop(tensor1, tensor2)
    }

    fn tensor_add(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        checker_binop(tensor1, tensor2)
    }

    fn tensor_and(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        checker_binop(tensor1, tensor2)
    }

    fn tensor_equ(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        checker_binop(tensor1, tensor2)
    }

    fn tensor_leq(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        checker_binop(tensor1, tensor2)
    }
}

fn boolalg_binop<A, F>(
    alg: &mut A,
    tensor1: &Tensor<A::Elem>,
    tensor2: &Tensor<A::Elem>,
    mut op: F,
) -> Tensor<A::Elem>
where
    A: BoolAlg,
    A::Elem: GenElem,
    F: FnMut(&mut A, A::Elem, A::Elem) -> A::Elem,
{
    assert!(tensor1.shape() == tensor2.shape());
    let shape = tensor1.shape.clone();
    let elems = GenVec::from_fn(tensor1.elems.len(), |i| {
        op(alg, tensor1.elems.get(i), tensor2.elems.get(i))
    });
    Tensor { shape, elems }
}

impl<A> TensorAlg for A
where
    A: BoolAlg,
    A::Elem: GenElem,
{
    type Tensor = Tensor<A::Elem>;

    fn scalar(self: &mut Self, elem: bool) -> Self::Tensor {
        Tensor::new(Shape::new(&[]), self.bool_lift(elem))
    }

    fn diagonal(self: &mut Self, dim: usize) -> Self::Tensor {
        let mut tensor = Tensor::new(Shape::new(&[dim, dim]), self.bool_zero());

        let unit = self.bool_unit();
        for idx in 0..dim {
            tensor.elems.set(idx * (dim + 1), unit);
        }

        tensor
    }

    fn polymer(
        self: &mut Self,
        tensor: &Self::Tensor,
        shape: Shape,
        mapping: &[usize],
    ) -> Self::Tensor {
        tensor.polymer(shape, mapping)
    }

    fn tensor_not(self: &mut Self, tensor: &Self::Tensor) -> Self::Tensor {
        let shape = tensor.shape.clone();
        let elems = GenVec::from_fn(tensor.elems.len(), |i| self.bool_not(tensor.elems.get(i)));
        Tensor { shape, elems }
    }

    fn tensor_or(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        boolalg_binop(self, tensor1, tensor2, BoolAlg::bool_or)
    }

    fn tensor_add(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        boolalg_binop(self, tensor1, tensor2, BoolAlg::bool_add)
    }

    fn tensor_and(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        boolalg_binop(self, tensor1, tensor2, BoolAlg::bool_and)
    }

    fn tensor_equ(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        boolalg_binop(self, tensor1, tensor2, BoolAlg::bool_equ)
    }

    fn tensor_leq(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        boolalg_binop(self, tensor1, tensor2, BoolAlg::bool_leq)
    }
}

/// The trait for solving tensor algebra problems.
pub trait SolverAlg {
    /// The type representing the tensor.
    type Tensor;

    /// Creates a new tensor with fresh variables.
    fn variable(self: &mut Self, shape: Shape) -> Self::Tensor;
}

impl SolverAlg for FreeAlg {
    type Tensor = Tensor<Literal>;

    fn variable(self: &mut Self, shape: Shape) -> Self::Tensor {
        let size = shape.size();
        let mut elems = Vec::with_capacity(size);
        for _ in 0..size {
            elems.push(self.add_variable());
        }
        Tensor { shape, elems }
    }
}

impl GenElem for Literal {
    type Vector = Vec<Self>;
}

#[cfg(test)]
mod tests {
    use super::super::boolalg::Boolean;
    use super::*;

    #[test]
    fn test_polymer() {
        let mut tensor: Tensor<usize> = Tensor::new(Shape::new(&[2, 3]), 0);
        for i in 0..2 {
            for j in 0..3 {
                tensor.__slow_set__(&[i, j], i + 10 * j);
            }
        }
        let tensor = tensor.polymer(Shape::new(&[3, 4, 2]), &[2, 0]);
        assert_eq!(*tensor.shape(), Shape::new(&[3, 4, 2]));
        for i in 0..2 {
            for j in 0..3 {
                for k in 0..4 {
                    assert_eq!(tensor.__slow_get__(&[j, k, i]), i + 10 * j);
                }
            }
        }
    }

    #[test]
    fn test_bool_tensor() {
        let mut alg = Boolean();
        let mut t1: Tensor<bool> = Tensor::new(Shape::new(&[2, 3]), false);
        t1.__slow_set__(&[0, 0], true);
        t1.__slow_set__(&[1, 1], true);
        t1.__slow_set__(&[1, 2], true);

        let t2 = alg.tensor_not(&t1);
        assert_eq!(t2.__slow_get__(&[0, 0]), false);
        assert_eq!(t2.__slow_get__(&[0, 1]), true);

        t1.__slow_set__(&[0, 1], true);
        let t3 = alg.tensor_and(&t1, &t2);
        assert_eq!(t3.__slow_get__(&[0, 0]), false);
        assert_eq!(t3.__slow_get__(&[0, 1]), true);
        assert_eq!(t3.__slow_get__(&[0, 2]), false);
        assert_eq!(t3.__slow_get__(&[1, 0]), false);
        assert_eq!(t3.__slow_get__(&[1, 1]), false);
        assert_eq!(t3.__slow_get__(&[1, 2]), false);
    }
}
