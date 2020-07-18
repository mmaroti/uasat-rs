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

pub struct Signature {
    op_arities: Vec<usize>,
}

impl Signature {
    pub fn op_add(&mut self, arity: usize) -> usize {
        let op = self.op_arities.len();
        self.op_arities.push(arity);
        op
    }

    pub fn op_count(&self) -> usize {
        self.op_arities.len()
    }

    pub fn op_arity(&self, op: usize) -> usize {
        self.op_arities[op]
    }
}

/// The common operations of all algebras.
pub trait Algebra {
    /// The element type of this algebra
    type Elem;

    /// Returns the signature of this algebra.
    fn signature(&self) -> &Signature;

    /// Applies the given operation to the list of arguments.
    fn operation(&mut self, op: usize, args: &[Self::Elem]) -> Self::Elem;
}
