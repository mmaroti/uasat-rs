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

use super::tensor::{Checker, Shape, TensorAlg};

/// A monomorphic calculation that takes a number of tensors and
/// produces some number of (usually one) tensors as output.
pub trait MonoCalc {
    /// Returns the required shape of the inputs.
    fn input_shapes(self: &Self) -> Vec<Shape>;

    /// Returns the input arity of the calculation.
    fn input_arity(self: &Self) -> usize {
        self.input_shapes().len()
    }

    /// Returns the produced shape of the outputs.
    fn output_shapes(self: &Self) -> Vec<Shape> {
        self.calculate(&mut Checker(), &self.input_shapes())
    }

    /// Returns the output arity of the calculation.
    fn output_arity(self: &Self) -> usize {
        self.output_shapes().len()
    }

    /// Evaluates the calculation within the given tensor algebra.
    fn calculate<A>(self: &Self, alg: &mut A, input: &[A::Tensor]) -> Vec<A::Tensor>
    where
        A: TensorAlg;
}

pub enum MonoPrim {
    Scalar(bool),
    Diagonal(usize),
    Polymer(Shape, Shape, Vec<usize>),
    TensorNot(Shape),
    TensorOr(Shape),
    TensorAdd(Shape),
    TensorAnd(Shape),
    TensorEqu(Shape),
    TensorLeq(Shape),
}

impl MonoCalc for MonoPrim {
    fn input_shapes(self: &Self) -> Vec<Shape> {
        match self {
            MonoPrim::Scalar(_) => vec![],
            MonoPrim::Diagonal(_) => vec![],
            MonoPrim::Polymer(shape, _, _) => vec![shape.clone()],
            MonoPrim::TensorNot(shape) => vec![shape.clone()],
            MonoPrim::TensorOr(shape) => vec![shape.clone(), shape.clone()],
            MonoPrim::TensorAnd(shape) => vec![shape.clone(), shape.clone()],
            MonoPrim::TensorAdd(shape) => vec![shape.clone(), shape.clone()],
            MonoPrim::TensorEqu(shape) => vec![shape.clone(), shape.clone()],
            MonoPrim::TensorLeq(shape) => vec![shape.clone(), shape.clone()],
        }
    }

    fn calculate<A>(self: &Self, alg: &mut A, input: &[A::Tensor]) -> Vec<A::Tensor>
    where
        A: TensorAlg,
    {
        debug_assert_eq!(self.input_arity(), input.len());
        match self {
            MonoPrim::Scalar(elem) => vec![alg.scalar(*elem)],
            MonoPrim::Diagonal(dim) => vec![alg.diagonal(*dim)],
            MonoPrim::Polymer(_, shape, map) => vec![alg.polymer(&input[0], shape.clone(), map)],
            MonoPrim::TensorNot(_) => vec![alg.tensor_not(&input[0])],
            MonoPrim::TensorOr(_) => vec![alg.tensor_or(&input[0], &input[1])],
            MonoPrim::TensorAnd(_) => vec![alg.tensor_and(&input[0], &input[1])],
            MonoPrim::TensorAdd(_) => vec![alg.tensor_add(&input[0], &input[1])],
            MonoPrim::TensorEqu(_) => vec![alg.tensor_equ(&input[0], &input[1])],
            MonoPrim::TensorLeq(_) => vec![alg.tensor_leq(&input[0], &input[1])],
        }
    }
}
