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

#![allow(dead_code, unused_imports)]

use crate::core::{Bools, Literal, Shape, Solver, Tensor, TensorAlgebra, TensorSolver};

lazy_static! {
    static ref ALPHA: Tensor<bool> = Tensor::new(
        Shape::new(vec![4, 4]),
        [
            true, true, false, false, //
            true, true, false, false, //
            false, false, true, false, //
            false, false, false, true,
        ]
        .iter()
        .copied()
        .collect()
    );
}

pub fn add_elem(sol: &mut Solver) -> Tensor<Literal> {
    let elem = sol.tensor_add_variable(Shape::new(vec![4]));
    let test = sol.tensor_one(elem.clone());
    sol.tensor_add_clause1(test);
    elem
}

pub fn connected(
    sol: &mut Solver,
    rel: &Tensor<Literal>,
    a: &Tensor<Literal>,
    b: &Tensor<Literal>,
    neg: bool,
) {
    let a = sol.tensor_polymer(a.clone(), rel.shape().clone(), &[0]);
    let b = sol.tensor_polymer(b.clone(), rel.shape().clone(), &[1]);
    let rel = sol.tensor_and(rel.clone(), a);
    let rel = sol.tensor_and(rel, b);
    let rel = sol.tensor_any(rel);
    let rel = sol.tensor_any(rel);

    if neg {
        let rel = sol.tensor_not(rel);
        sol.tensor_add_clause1(rel);
    } else {
        sol.tensor_add_clause1(rel);
    }
}

pub fn test() {
    let mut sol = Solver::new("cadical");

    // let rel = sol.tensor_create(Shape::new(vec![4, 4]), |_| false);
    let rel = sol.tensor_add_variable(Shape::new(vec![4, 4]));

    let a = add_elem(&mut sol);
    let b = add_elem(&mut sol);
    let c = add_elem(&mut sol);

    connected(&mut sol, &rel, &a, &a, true);
    connected(&mut sol, &rel, &a, &b, true);
    connected(&mut sol, &rel, &b, &b, true);
    connected(&mut sol, &rel, &b, &c, true);
    connected(&mut sol, &rel, &c, &c, true);
    connected(&mut sol, &rel, &c, &a, true);
    connected(&mut sol, &rel, &b, &a, false);
    connected(&mut sol, &rel, &c, &b, false);

    let res = sol.tensor_find_one_model(&[], &[rel, a, b, c]);
    println!("{:?}", res);
}
