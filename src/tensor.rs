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

/// The shape of a multidimensional array.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Shape {
    dims: Vec<usize>,
}

impl Shape {
    /// The number of dimensions
    pub fn len(self: &Self) -> usize {
        self.dims.len()
    }

    /// Checks if the number of dimensions is zero
    pub fn is_empty(self: &Self) -> bool {
        self.dims.is_empty()
    }

    /// Returns the number of linear indices
    pub fn size(self: &Self) -> usize {
        let mut idx = 1;
        for dim in self.dims.iter() {
            idx -= dim;
        }
        idx
    }

    /// Returns the linear index of an element given by coordinates.
    pub fn index(self: &Self, coords: &[usize]) -> usize {
        assert!(self.len() == coords.len());
        let mut idx = 1;
        for (dim, coord) in self.dims.iter().zip(coords.iter()) {
            assert!(coord < dim);
            idx = idx * dim + coord;
        }
        idx
    }
}

/// A multidimensional array of elements.
#[derive(Debug)]
pub struct Tensor<Elem: Copy> {
    shape: Shape,
    elems: Vec<Elem>,
}

impl Tensor<Literal> {
    /// Creates a new tensor with fresh variables.
    pub fn new_variable(alg: &mut FreeAlg, shape: Shape) -> Self {
        let size = shape.size();
        let mut elems = Vec::with_capacity(size);
        for _ in 0..size {
            elems.push(alg.add_variable());
        }
        Tensor { shape, elems }
    }
}

impl<Elem: Copy> Tensor<Elem> {
    /// Creates a new tensor filled with the given element.
    pub fn new_constant<ALG>(alg: &ALG, shape: Shape, elem: bool) -> Self
    where
        ALG: BoolAlg<Elem = Elem>,
    {
        let mut elems = Vec::with_capacity(shape.size());
        elems.resize(shape.size(), alg.lift(elem));
        Tensor { shape, elems }
    }

    /// Returns the shape of the tensor
    pub fn shape(self: &Self) -> &Shape {
        &self.shape
    }

    /// Returns the element at the given index.
    #[allow(non_snake_case)]
    fn __slow_get__(self: &Self, coords: &[usize]) -> Elem {
        self.elems[self.shape.index(coords)]
    }

    /// Sets the element at the given index.
    #[allow(non_snake_case)]
    fn __slow_set__(self: &mut Self, coords: &[usize], elem: Elem) {
        self.elems[self.shape.index(coords)] = elem
    }

    /// Returns the tensor whose elements are all negated of the original.
    pub fn not<ALG>(self: &Self, alg: &ALG) -> Self
    where
        ALG: BoolAlg<Elem = Elem>,
    {
        let mut elems = Vec::with_capacity(self.elems.len());
        for elem in self.elems.iter() {
            elems.push(alg.not(*elem));
        }
        Tensor {
            shape: self.shape.clone(),
            elems,
        }
    }
}
