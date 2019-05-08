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
/// produces some number (usually one) tensors as output.
pub trait MonoCalculation {
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

pub enum MonoPrimitive {
    Scalar(bool),
    Diagonal(usize),
    Polymer(Shape, Vec<usize>, Shape),
    TensorNot(Shape),
    TensorOr(Shape),
    TensorAnd(Shape),
}

impl MonoCalculation for MonoPrimitive {
    fn input_shapes(self: &Self) -> Vec<Shape> {
        match self {
            MonoPrimitive::Scalar(_) => vec![],
            MonoPrimitive::Diagonal(dim) => vec![],
            MonoPrimitive::Polymer(shape, _, _) => vec![shape.clone()],
            MonoPrimitive::TensorNot(shape) => vec![shape.clone()],
            MonoPrimitive::TensorOr(shape) => vec![shape.clone(), shape.clone()],
            MonoPrimitive::TensorAnd(shape) => vec![shape.clone(), shape.clone()],
        }
    }

    fn calculate<A>(self: &Self, alg: &mut A, input: &[A::Tensor]) -> Vec<A::Tensor>
    where
        A: TensorAlg,
    {
        match self {
            MonoPrimitive::Scalar(elem) => vec![alg.scalar(*elem)],
            MonoPrimitive::Diagonal(dim) => vec![alg.diagonal(*dim)],
            _ => unimplemented!(),
        }
    }
}
