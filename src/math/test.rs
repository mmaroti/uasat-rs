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

use crate::math::binrel;
use crate::math::BinaryRel;
use crate::tensor::{Boolean, Shape, Solver, Tensor, TensorAlg, TensorSat};

fn single(size: usize, i0: usize, i1: usize) -> Tensor<bool> {
    let shape = Shape::new(vec![size, size]);
    Tensor::create(shape, |i| i[0] == i0 && i[1] == i1)
}

fn mapping(map: &[isize], target: usize) -> Tensor<bool> {
    let shape = Shape::new(vec![map.len(), target]);
    Tensor::create(shape, |i| map[i[0]] == i[1] as isize)
}

pub fn test1() {
    let mut alg = Solver::new("cadical");

    let fun = alg.tensor_add_variable(Shape::new(vec![20, 20]));
    let tmp = alg.is_function(fun.clone());
    alg.tensor_add_clause(&[tmp]);

    let tmp = alg.is_surjective(fun.clone());
    alg.tensor_add_clause(&[tmp]);

    let rel = binrel::crown_poset(20);
    let rel = alg.tensor_lift(rel);
    let tmp = alg.is_compatible(fun.clone(), rel.clone(), rel.clone());
    alg.tensor_add_clause(&[tmp]);

    while let Some(sol) = alg.tensor_find_one_model1(fun.clone()) {
        println!("{:?}", sol);
        let tmp = alg.tensor_lift(sol);
        let tmp = alg.is_equal_to(tmp, fun.clone());
        let tmp = alg.tensor_not(tmp);
        alg.tensor_add_clause(&[tmp]);
    }

    // let num = alg.tensor_find_num_models(&[fun]);
    // println!("{}", num);
}
