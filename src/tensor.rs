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
use std::ops::Index;

/// The shape of a multidimensional array.
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
pub struct Tensor<Elem: Copy> {
    shape: Shape,
    elems: Vec<Elem>,
}

impl<Elem: Copy> Tensor<Elem> {
    /// Creates a tensor filled with constant value
    pub fn new(shape: Shape, elem: Elem) -> Self {
        let size = shape.size();
        let mut elems = Vec::with_capacity(size);
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
        self.elems[self.shape.index(coords)]
    }

    /// Sets the element at the given index.
    #[allow(non_snake_case)]
    pub fn __slow_set__(self: &mut Self, coords: &[usize], elem: Elem) {
        self.elems[self.shape.index(coords)] = elem
    }

    /// Creates a new tensor of the given shape from the given old tensor with
    /// permuted, identified or new dummy coordinates. The mapping is a vector
    /// of length of the original tensor shape with entries identifying the
    /// matching coordinates in the new tensor.
    pub fn polymer(self: &Self, shape: Shape, mapping: &[usize]) -> Self {
        assert!(mapping.len() == self.shape.len());

        let size = shape.size();
        let mut elems = Vec::with_capacity(size);

        let mut iter = StrideIter::new(&shape);
        let strides = self.shape.strides();
        for (idx, val) in mapping.iter().enumerate() {
            assert!(self.shape[idx] == shape[*val]);
            iter.add_stride(*val, strides[idx]);
        }

        for index in iter {
            elems.push(self.elems[index]);
        }
        assert!(elems.len() == size);

        Tensor { shape, elems }
    }
}

/// A tensor algebra for tensors.
pub trait TensorAlg {
    /// The type representing the tensor.
    type Tensor;

    /// Creates a new tensor filled with the given element.
    fn constant(self: &mut Self, shape: Shape, elem: bool) -> Self::Tensor;

    /// Returns a diagonal tensor with true elements on the diagonal and false
    /// everywhere else. The shape must have at least two dimensions and the
    /// first two dimensions must be equal.
    fn diagonal(self: &mut Self, shape: Shape) -> Self::Tensor;

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
    fn not(self: &mut Self, tensor: &Self::Tensor) -> Self::Tensor;

    /// Returns a new tensor whose elements are conjunctions of the original
    /// elements.
    fn and(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor;
}

/// The tensor algebra used for checking the shapes of calculations.
pub struct ShapeCheck();

impl TensorAlg for ShapeCheck {
    type Tensor = Shape;

    fn constant(self: &mut Self, shape: Shape, _elem: bool) -> Self::Tensor {
        shape
    }

    fn diagonal(self: &mut Self, shape: Shape) -> Self::Tensor {
        assert!(shape.len() >= 2 && shape[0] == shape[1]);
        shape
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

    fn not(self: &mut Self, tensor: &Self::Tensor) -> Self::Tensor {
        tensor.clone()
    }

    fn and(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        assert!(tensor1 == tensor2);
        tensor1.clone()
    }
}

impl<ALG: BoolAlg> TensorAlg for ALG {
    type Tensor = Tensor<ALG::Elem>;

    fn constant(self: &mut Self, shape: Shape, elem: bool) -> Self::Tensor {
        let size = shape.size();
        let mut elems = Vec::with_capacity(size);
        elems.resize(size, self.lift(elem));
        Tensor { shape, elems }
    }

    fn diagonal(self: &mut Self, shape: Shape) -> Self::Tensor {
        assert!(shape.len() >= 2 && shape[0] == shape[1]);

        let dim = shape[0];
        let mut tensor = self.constant(Shape::new(&[dim, dim]), false);

        let unit = self.unit();
        for idx in 0..dim {
            tensor.elems[idx * (dim + 1)] = unit;
        }

        if shape.len() == 2 {
            tensor
        } else {
            self.polymer(&tensor, shape, &[0, 1])
        }
    }

    fn polymer(
        self: &mut Self,
        tensor: &Self::Tensor,
        shape: Shape,
        mapping: &[usize],
    ) -> Self::Tensor {
        tensor.polymer(shape, mapping)
    }

    fn not(self: &mut Self, tensor: &Self::Tensor) -> Self::Tensor {
        let shape = tensor.shape.clone();
        let mut elems = Vec::with_capacity(tensor.elems.len());
        for elem in tensor.elems.iter() {
            elems.push(BoolAlg::not(self, *elem));
        }

        Tensor { shape, elems }
    }

    fn and(self: &mut Self, tensor1: &Self::Tensor, tensor2: &Self::Tensor) -> Self::Tensor {
        let shape = tensor1.shape.clone();
        let mut elems = Vec::with_capacity(tensor1.elems.len());
        for (elem1, elem2) in tensor1.elems.iter().zip(tensor2.elems.iter()) {
            elems.push(BoolAlg::and(self, *elem1, *elem2));
        }

        Tensor { shape, elems }
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
    type Tensor = (Tensor<Literal>, usize);

    fn variable(self: &mut Self, shape: Shape) -> Self::Tensor {
        let size = shape.size();
        let mut elems = Vec::with_capacity(size);
        for _ in 0..size {
            elems.push(self.add_variable());
        }
        let marker = self as *const Self as usize;
        (Tensor { shape, elems }, marker)
    }
}

#[cfg(test)]
mod tests {
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
}
