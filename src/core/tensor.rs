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

//! Basic multidimensional array type and operations over boolean algebras.

use std::ops;

use super::{BooleanAlgebra, BooleanSolver, GenElem, GenVec, VecFor};

/// The shape of a tensor.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Shape {
    dims: Vec<usize>,
}

impl Shape {
    /// Creates a new shape object.
    pub fn new(dims: Vec<usize>) -> Self {
        Shape { dims }
    }

    /// The number of dimensions.
    pub fn len(&self) -> usize {
        self.dims.len()
    }

    /// Checks if the number of dimensions is zero.
    pub fn is_empty(&self) -> bool {
        self.dims.is_empty()
    }

    /// Returns the dimensions.
    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    /// Checks if all dimensions are equal to the given one.
    pub fn is_rectangular(&self, dim: usize) -> bool {
        self.dims.iter().all(|d| *d == dim)
    }

    /// Returns the head and tail of this shape. The shape must have at
    /// least one dimension.
    pub fn split1(&self) -> (usize, Self) {
        assert!(!self.dims.is_empty());
        (self.dims[0], Shape::new(self.dims[1..].to_vec()))
    }

    /// Returns a pair of heads and tail of this shape. The shape must
    /// have at least two dimensions.
    pub fn split2(&self) -> (usize, usize, Self) {
        assert!(!self.dims.len() >= 2);
        (
            self.dims[0],
            self.dims[1],
            Shape::new(self.dims[2..].to_vec()),
        )
    }

    /// Returns a new shape that is the same as this one but a few new
    /// dimension are added to the front.
    pub fn join(&self, prefix: &[usize]) -> Self {
        let mut dims = Vec::with_capacity(self.dims.len() + prefix.len());
        dims.extend(prefix);
        dims.extend(&self.dims);
        Shape { dims }
    }

    /// Returns the number of elements this shape represents.
    pub fn size(&self) -> usize {
        let mut size = 1;
        for dim in self.dims.iter() {
            size *= *dim;
        }
        size
    }

    /// Returns the linear index of an element given by coordinates.
    fn index(&self, coords: &[usize]) -> usize {
        assert_eq!(coords.len(), self.len());
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
    fn strides(&self) -> Vec<usize> {
        let mut size = 1;
        self.dims
            .iter()
            .map(|d| {
                let s = size;
                size *= d;
                s
            })
            .collect()
    }
}

impl ops::Index<usize> for Shape {
    type Output = usize;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.dims[idx]
    }
}

#[doc(hidden)]
/// Iterator for implementing the polymer operation
struct StrideIter {
    entries: Vec<(usize, usize, usize)>,
    index: usize,
    done: bool,
}

impl StrideIter {
    fn new(shape: &Shape) -> Self {
        let mut done = false;
        let entries = shape
            .dims
            .iter()
            .map(|d| {
                done |= *d == 0;
                (0, *d, 0)
            })
            .collect();

        StrideIter {
            entries,
            index: 0,
            done,
        }
    }

    fn add_stride(&mut self, idx: usize, stride: usize) {
        self.entries[idx].2 += stride;
    }
}

impl Iterator for StrideIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
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
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor<ELEM>
where
    ELEM: GenElem,
{
    shape: Shape,
    elems: VecFor<ELEM>,
}

impl<ELEM> Tensor<ELEM>
where
    ELEM: GenElem,
{
    /// Creates a tensor of the given shape and with the given elements.
    pub fn new(shape: Shape, elems: VecFor<ELEM>) -> Self {
        assert_eq!(shape.size(), elems.len());
        Tensor { shape, elems }
    }

    /// Returns the shape of the tensor.
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Creates a new tensor of the given shape where the elements
    /// are calculated by an operation.
    pub fn create<OP>(shape: Shape, mut op: OP) -> Self
    where
        OP: FnMut(&[usize]) -> ELEM,
    {
        let mut coords = vec![0; shape.len()];
        let elems: VecFor<ELEM> = (0..shape.size())
            .map(|_| {
                let e = op(&coords);
                for (a, b) in coords.iter_mut().zip(shape.dims.iter()) {
                    *a += 1;
                    if *a >= *b {
                        *a = 0;
                    } else {
                        break;
                    }
                }
                e
            })
            .collect();

        Tensor::new(shape, elems)
    }

    /// Returns the element at the given index.
    pub fn very_slow_get(&self, coords: &[usize]) -> ELEM {
        self.elems.get(self.shape.index(coords))
    }

    /// Sets the element at the given index.
    pub fn very_slow_set(&mut self, coords: &[usize], elem: ELEM) {
        self.elems.set(self.shape.index(coords), elem);
    }

    /// Returns the scalar value contained within a tensor of shape [].
    pub fn scalar(&self) -> ELEM {
        assert!(self.shape.is_empty());
        assert!(self.elems.len() == 1);
        self.elems.get(0)
    }

    /// Creates a new tensor of the given shape from the given old tensor with
    /// permuted, identified or new dummy coordinates. The mapping is a vector
    /// of length of the original tensor shape with entries identifying the
    /// matching coordinates in the new tensor.
    pub fn polymer(&self, shape: Shape, mapping: &[usize]) -> Self {
        assert_eq!(mapping.len(), self.shape.len());

        let mut iter = StrideIter::new(&shape);
        let strides = self.shape.strides();
        for (idx, val) in mapping.iter().enumerate() {
            assert_eq!(self.shape[idx], shape[*val]);
            iter.add_stride(*val, strides[idx]);
        }

        let elems: VecFor<ELEM> = iter.map(|i| self.elems.get(i)).collect();
        Tensor::new(shape, elems)
    }

    /// Returns a new tensor with the same underling data but with a different
    /// shape. The new shape must have the same size as the original one.
    pub fn reshape(&self, shape: Shape) -> Self {
        Tensor::new(shape, self.elems.clone())
    }
}

/// A tensor algebra for tensors.
pub trait TensorAlgebra {
    /// The type representing the tensor.
    type Elem: Clone;

    /// Returns the shape of the tensor.
    fn shape<'e>(&self, elem: &'e Self::Elem) -> &'e Shape;

    /// Creates a new tensor from the given bool tensor.
    fn tensor_lift(&self, elem: Tensor<bool>) -> Self::Elem;

    /// Creates a new constant tensor of the given shape where
    /// the elements are calculated by an operation.
    fn tensor_create<OP>(&self, shape: Shape, op: OP) -> Self::Elem
    where
        OP: FnMut(&[usize]) -> bool;

    /// Creates a new tensor of the given shape from the given old tensor with
    /// permuted, identified or new dummy coordinates. The mapping is a vector
    /// of length of the old tensor shape with entries identifying the
    /// coordinate in the new tensor.
    fn tensor_polymer(&self, elem: Self::Elem, shape: Shape, mapping: &[usize]) -> Self::Elem;

    /// Returns a new tensor with the same underling data but with a different
    /// shape. The new shape must have the same size as the original one.
    fn tensor_reshape(&self, elem: Self::Elem, shape: Shape) -> Self::Elem;

    /// Returns a new tensor whose elements are all negated of the original.
    fn tensor_not(&mut self, elem: Self::Elem) -> Self::Elem;

    /// Returns a new tensor whose elements are disjunctions of the original
    /// elements.
    fn tensor_or(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns a new tensor whose elements are the conjunctions of the
    /// original elements.
    fn tensor_and(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns a new tensor whose elements are the boolean additions of the
    /// original elements.
    fn tensor_xor(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns a new tensor whose elements are the logical equivalence of the
    /// original elements.
    fn tensor_equ(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns a new tensor whose elements are the logical implication of the
    /// original elements.
    fn tensor_imp(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Returns a new tensor with the first dimension removed where the result
    /// is the conjunction of the elements.
    fn tensor_all(&mut self, elem: Self::Elem) -> Self::Elem;

    /// Returns a new tensor with the first dimension removed where the result
    /// is the disjunction of the elements.
    fn tensor_any(&mut self, elem: Self::Elem) -> Self::Elem;

    /// Returns a new tensor with the first dimension removed where the result
    /// is the binary sum of the elements.
    fn tensor_sum(&mut self, elem: Self::Elem) -> Self::Elem;

    /// Returns a new tensor with the first dimension removed where the result
    /// is the exactly one set predicate.
    fn tensor_one(&mut self, elem: Self::Elem) -> Self::Elem;

    /// Returns a new tensor with the first dimension removed where the result
    /// is the at most one set predicate.
    fn tensor_amo(&mut self, elem: Self::Elem) -> Self::Elem;
}

impl<ALG> TensorAlgebra for ALG
where
    ALG: BooleanAlgebra,
{
    type Elem = Tensor<ALG::Elem>;

    fn shape<'e>(&self, elem: &'e Self::Elem) -> &'e Shape {
        &elem.shape
    }

    fn tensor_lift(&self, elem: Tensor<bool>) -> Self::Elem {
        let elems = elem.elems.gen_iter().map(|b| self.bool_lift(b)).collect();
        Tensor::new(elem.shape, elems)
    }

    fn tensor_create<OP>(&self, shape: Shape, mut op: OP) -> Self::Elem
    where
        OP: FnMut(&[usize]) -> bool,
    {
        Tensor::create(shape, |coords| self.bool_lift(op(coords)))
    }

    fn tensor_polymer(&self, elem: Self::Elem, shape: Shape, mapping: &[usize]) -> Self::Elem {
        elem.polymer(shape, mapping)
    }

    fn tensor_reshape(&self, elem: Self::Elem, shape: Shape) -> Self::Elem {
        elem.reshape(shape)
    }

    fn tensor_not(&mut self, elem: Self::Elem) -> Self::Elem {
        let elems = elem.elems.gen_iter().map(|b| self.bool_not(b)).collect();
        Tensor::new(elem.shape, elems)
    }

    fn tensor_or(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert_eq!(elem1.shape, elem2.shape);
        let elems = elem1
            .elems
            .gen_iter()
            .zip(elem2.elems.gen_iter())
            .map(|(a, b)| self.bool_or(a, b))
            .collect();
        Tensor::new(elem1.shape, elems)
    }

    fn tensor_and(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert_eq!(elem1.shape, elem2.shape);
        let elems = elem1
            .elems
            .gen_iter()
            .zip(elem2.elems.gen_iter())
            .map(|(a, b)| self.bool_and(a, b))
            .collect();
        Tensor::new(elem1.shape, elems)
    }

    fn tensor_xor(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert_eq!(elem1.shape, elem2.shape);
        let elems = elem1
            .elems
            .gen_iter()
            .zip(elem2.elems.gen_iter())
            .map(|(a, b)| self.bool_xor(a, b))
            .collect();
        Tensor::new(elem1.shape, elems)
    }

    fn tensor_equ(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert_eq!(elem1.shape, elem2.shape);
        let elems = elem1
            .elems
            .gen_iter()
            .zip(elem2.elems.gen_iter())
            .map(|(a, b)| self.bool_equ(a, b))
            .collect();
        Tensor::new(elem1.shape, elems)
    }

    fn tensor_imp(&mut self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert_eq!(elem1.shape, elem2.shape);
        let elems = elem1
            .elems
            .gen_iter()
            .zip(elem2.elems.gen_iter())
            .map(|(a, b)| self.bool_imp(a, b))
            .collect();
        Tensor::new(elem1.shape, elems)
    }

    fn tensor_all(&mut self, elem: Self::Elem) -> Self::Elem {
        let (head, shape) = elem.shape.split1();
        let elems = elem
            .elems
            .split(head)
            .iter()
            .map(|v| self.bool_fold_all(v.gen_iter()))
            .collect();
        Tensor::new(shape, elems)
    }

    fn tensor_any(&mut self, elem: Self::Elem) -> Self::Elem {
        let (head, shape) = elem.shape.split1();
        let elems = elem
            .elems
            .split(head)
            .iter()
            .map(|v| self.bool_fold_any(v.gen_iter()))
            .collect();
        Tensor::new(shape, elems)
    }

    fn tensor_sum(&mut self, elem: Self::Elem) -> Self::Elem {
        let (head, shape) = elem.shape.split1();
        let elems = elem
            .elems
            .split(head)
            .iter()
            .map(|v| self.bool_fold_sum(v.gen_iter()))
            .collect();
        Tensor::new(shape, elems)
    }

    fn tensor_one(&mut self, elem: Self::Elem) -> Self::Elem {
        let (head, shape) = elem.shape.split1();
        let elems = elem
            .elems
            .split(head)
            .iter()
            .map(|v| self.bool_fold_one(v.gen_iter()))
            .collect();
        Tensor::new(shape, elems)
    }

    fn tensor_amo(&mut self, elem: Self::Elem) -> Self::Elem {
        let (head, shape) = elem.shape.split1();
        let elems = elem
            .elems
            .split(head)
            .iter()
            .map(|v| self.bool_fold_amo(v.gen_iter()))
            .collect();
        Tensor::new(shape, elems)
    }
}

/// The trait for solving tensor algebra problems.
pub trait TensorSolver: TensorAlgebra {
    /// Creates a new tensor with fresh variables.
    fn tensor_add_variable(&mut self, shape: Shape) -> Self::Elem;

    /// Adds the given (disjunctive) clause to the solver.
    fn tensor_add_clause(&mut self, clause: &[Self::Elem]);

    /// Adds the given 1-element clause to the solver.
    fn tensor_add_clause1(&mut self, elem1: Self::Elem) {
        self.tensor_add_clause(&[elem1]);
    }

    /// Adds the given 2-element clause to the solver.
    fn tensor_add_clause2(&mut self, elem1: Self::Elem, elem2: Self::Elem) {
        self.tensor_add_clause(&[elem1, elem2]);
    }

    /// Runs the solver and returns a model if it exists. The shapes of the
    /// returned tensors match the ones that were passed in.
    fn tensor_find_one_model(
        &mut self,
        assumptions: &[Self::Elem],
        elems: &[Self::Elem],
    ) -> Option<Vec<Tensor<bool>>>;

    /// Runs the solver and returns a model if it exists. The shapes of the
    /// returned tensors match the ones that were passed in.
    fn tensor_find_one_model1(&mut self, elem1: Self::Elem) -> Option<Tensor<bool>> {
        if let Some(mut result) = self.tensor_find_one_model(&[], &[elem1]) {
            assert_eq!(result.len(), 1);
            result.pop()
        } else {
            None
        }
    }

    /// Runs the solver and returns a model if it exists. The shapes of the
    /// returned tensors match the ones that were passed in.
    fn tensor_find_one_model2(
        &mut self,
        elem1: Self::Elem,
        elem2: Self::Elem,
    ) -> Option<(Tensor<bool>, Tensor<bool>)> {
        if let Some(mut result) = self.tensor_find_one_model(&[], &[elem1, elem2]) {
            assert_eq!(result.len(), 2);
            let elem2 = result.pop().unwrap();
            let elem1 = result.pop().unwrap();
            Some((elem1, elem2))
        } else {
            None
        }
    }

    /// Returns the number of models with respect to the given tensors.
    fn tensor_find_num_models(self, elems: &[Self::Elem]) -> usize;
}

impl<ALG> TensorSolver for ALG
where
    ALG: BooleanSolver,
{
    fn tensor_add_variable(&mut self, shape: Shape) -> Self::Elem {
        let elems = (0..shape.size())
            .map(|_| self.bool_add_variable())
            .collect();
        Tensor::new(shape, elems)
    }

    fn tensor_add_clause(&mut self, clause: &[Self::Elem]) {
        if clause.is_empty() {
            self.bool_add_clause(&[]);
            return;
        }

        let shape = clause[0].shape();
        for t in clause.iter().skip(1) {
            assert_eq!(t.shape(), shape);
        }

        if shape.size() == 0 {
            return;
        }

        let mut clause2: Vec<ALG::Elem> = Vec::with_capacity(clause.len());
        for i in 0..shape.size() {
            clause2.clear();
            clause2.extend(clause.iter().map(|t| t.elems.get(i)));
            self.bool_add_clause(&clause2);
        }
    }

    fn tensor_find_one_model(
        &mut self,
        assumptions: &[Self::Elem],
        elems: &[Self::Elem],
    ) -> Option<Vec<Tensor<bool>>> {
        let ass2: Vec<ALG::Elem> = assumptions
            .iter()
            .flat_map(|t| t.elems.gen_iter())
            .collect();
        let literals2 = elems.iter().flat_map(|t| t.elems.gen_iter());
        if let Some(values) = self.bool_find_one_model(&ass2, literals2) {
            let mut result: Vec<Tensor<bool>> = Vec::with_capacity(elems.len());
            let mut pos = 0;
            for t in elems {
                let size = t.shape().size();
                result.push(Tensor::new(
                    t.shape().clone(),
                    values.gen_iter().skip(pos).take(size).collect(),
                ));
                pos += size;
            }
            Some(result)
        } else {
            None
        }
    }

    fn tensor_find_num_models(self, elems: &[Self::Elem]) -> usize {
        let all_elems = elems.iter().flat_map(|t| t.elems.gen_iter());
        self.bool_find_num_models_method1(all_elems)
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::super::Bools;
    use super::*;

    #[test]
    fn polymer() {
        let mut tensor: Tensor<usize> =
            Tensor::new(Shape::new(vec![2, 3]), iter::repeat(0).take(6).collect());
        for i in 0..2 {
            for j in 0..3 {
                tensor.very_slow_set(&[i, j], i + 10 * j);
            }
        }
        let tensor = tensor.polymer(Shape::new(vec![3, 4, 2]), &[2, 0]);
        assert_eq!(tensor.shape, Shape::new(vec![3, 4, 2]));
        for i in 0..2 {
            for j in 0..3 {
                for k in 0..4 {
                    assert_eq!(tensor.very_slow_get(&[j, k, i]), i + 10 * j);
                }
            }
        }
    }

    #[test]
    fn getset() {
        let mut alg = Bools();
        let mut t1: Tensor<bool> = Tensor::new(
            Shape::new(vec![2, 3]),
            iter::repeat(false).take(6).collect(),
        );
        t1.very_slow_set(&[0, 0], true);
        t1.very_slow_set(&[1, 1], true);
        t1.very_slow_set(&[1, 2], true);

        let t2 = alg.tensor_not(t1.clone());
        assert_eq!(t2.very_slow_get(&[0, 0]), false);
        assert_eq!(t2.very_slow_get(&[0, 1]), true);

        t1.very_slow_set(&[0, 1], true);
        let t3 = alg.tensor_and(t1, t2);
        assert_eq!(t3.very_slow_get(&[0, 0]), false);
        assert_eq!(t3.very_slow_get(&[0, 1]), true);
        assert_eq!(t3.very_slow_get(&[0, 2]), false);
        assert_eq!(t3.very_slow_get(&[1, 0]), false);
        assert_eq!(t3.very_slow_get(&[1, 1]), false);
        assert_eq!(t3.very_slow_get(&[1, 2]), false);

        let t4 = Tensor::create(Shape::new(vec![2, 3]), |c| c[0] == 0 && c[1] == 1);
        assert_eq!(t3, t4);
    }

    #[test]
    fn fold() {
        let mut alg = Bools();
        let mut t1: Tensor<bool> = Tensor::new(
            Shape::new(vec![2, 4]),
            iter::repeat(false).take(8).collect(),
        );
        t1.very_slow_set(&[0, 1], true);
        t1.very_slow_set(&[1, 2], true);
        t1.very_slow_set(&[0, 3], true);
        t1.very_slow_set(&[1, 3], true);

        let t2 = alg.tensor_all(t1.clone());
        assert_eq!(t2.shape, Shape::new(vec![4]));
        assert_eq!(t2.very_slow_get(&[0]), false);
        assert_eq!(t2.very_slow_get(&[1]), false);
        assert_eq!(t2.very_slow_get(&[2]), false);
        assert_eq!(t2.very_slow_get(&[3]), true);

        let t3 = t1.reshape(Shape::new(vec![8]));
        let t3 = alg.tensor_all(t3);
        assert_eq!(t3.shape, Shape::new(vec![]));
        assert_eq!(t3.very_slow_get(&[]), false);
    }
}
