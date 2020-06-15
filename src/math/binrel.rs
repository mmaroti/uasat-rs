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

use crate::tensor::{Boolean, Shape, Solver, Tensor, TensorAlg, TensorSat};
use std::time::Instant;

trait BinaryRel: TensorAlg {
    /// Creates a tensor of shape `[size, size]` representing the
    /// binary less than or equal relation of the crown.
    fn crown_poset(&self, size: usize) -> Self::Elem {
        assert!(size >= 4 && size % 2 == 0);
        let rel = Tensor::create(Shape::new(vec![size, size]), |i| {
            if i[0] % 2 == 1 {
                i[0] == i[1]
            } else if i[0] == 0 {
                i[1] <= 1 || i[1] == size - 1
            } else {
                i[1] >= i[0] - 1 && i[1] <= i[0] + 1
            }
        });
        self.tensor_lift(rel)
    }

    /// Creates the diagonal relation of shape `[size, size]`.
    fn diagonal(&self, size: usize) -> Self::Elem {
        let rel = Tensor::create(Shape::new(vec![size, size]), |i| i[0] == i[1]);
        self.tensor_lift(rel)
    }

    /// Creates the less than relation of shape `[size, size]`.
    fn lessthan(&self, size: usize) -> Self::Elem {
        let rel = Tensor::create(Shape::new(vec![size, size]), |i| i[0] < i[1]);
        self.tensor_lift(rel)
    }

    /// Checks if the given tensor of shape `[a, b]` is a mapping from a
    /// b-element set to an a-element set, and returns the result in a tensor
    /// of shape `[]`.
    fn is_function<ALG: TensorAlg>(alg: &mut ALG, fun: ALG::Elem) -> ALG::Elem {
        let fun = alg.tensor_one(fun);
        alg.tensor_all(fun)
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
    /// of the same shape, and returns the relation as a tensor of shape `[]`.
    fn is_subsetof(&mut self, rel0: Self::Elem, rel1: Self::Elem) -> Self::Elem {
        let rel2 = self.tensor_imp(rel0, rel1);
        let rel2 = self.tensor_all(rel2);
        self.tensor_all(rel2)
    }

    /// Checks if the binary relation of shape `[a, a]` is transitive
    /// and returns the result in a tensor of shape `[]`.
    fn is_transitive(&mut self, rel: Self::Elem) -> Self::Elem {
        let tmp = self.compose(rel.clone(), rel.clone());
        self.is_subsetof(tmp, rel)
    }

    /// Returns the transpose of the binary relation of shape `[a, b]`
    /// as a tensor of shape `[b, a]`.
    fn transpose(&mut self, rel: Self::Elem) -> Self::Elem {
        let shape = self.shape(&rel).clone();
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
        let tmp = self.diagonal(size);
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
}

impl<ALG> BinaryRel for ALG where ALG: TensorAlg {}

fn check(
    solver: &str,
    desc: &str,
    shape: Shape,
    pred: fn(&mut Solver, elem: <Solver as TensorAlg>::Elem) -> <Solver as TensorAlg>::Elem,
    count: usize,
) {
    let mut sol = Solver::new(solver);
    let elem = sol.tensor_add_variable(shape.clone());
    let cond = pred(&mut sol, elem.clone());
    sol.tensor_add_clause(&[cond]);
    let num = sol.tensor_find_num_models(&[elem]);
    println!("Number of {} of shape {:?} is {}", desc, shape.dims(), num);
    assert_eq!(num, count);
}

/// Validates the solver by calculating some numbers from the
/// Online Encyclopedia of Integer Sequences.
pub fn validate(solver: &str) {
    let start = Instant::now();

    check(
        solver,
        "transitive relations",
        Shape::new(vec![4, 4]),
        <Solver as BinaryRel>::is_transitive,
        3994,
    );

    check(
        solver,
        "equivalence relations",
        Shape::new(vec![8, 8]),
        <Solver as BinaryRel>::is_equivalence,
        4140,
    );

    check(
        solver,
        "partial orders",
        Shape::new(vec![5, 5]),
        <Solver as BinaryRel>::is_partial_order,
        4231,
    );

    check(
        solver,
        "functions",
        Shape::new(vec![6, 5]),
        <Solver as BinaryRel>::is_function,
        7776,
    );

    let duration = Instant::now().duration_since(start).as_secs_f32();
    println!("Solver {} finished in {} seconds\n", solver, duration);
}

pub fn test() {
    let mut alg = Boolean();
    let crown4 = alg.crown_poset(4);
    assert!(alg.is_partial_order(crown4.clone()).scalar());

    validate("cadical");
    validate("batsat");
    validate("minisat");
    validate("varisat");
    validate("cryptominisat");
}
