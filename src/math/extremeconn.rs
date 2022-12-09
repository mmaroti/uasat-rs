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

use crate::core::{
    BooleanAlgebra, BooleanSolver, Bools, Literal, Shape, Solver, Tensor, TensorAlgebra,
    TensorSolver,
};

pub fn test() {
    let mut solver = Solver::new("minisat");

    let target_size = 4;

    // {0, 1}, {2}, {3}
    let target_alpha: Tensor<bool> = Tensor::new(
        Shape::new(vec![4, 4]),
        [
            true, true, false, false, //
            true, true, false, false, //
            false, false, true, false, //
            false, false, false, true,
        ]
        .iter()
        .copied()
        .collect(),
    );
    let target_alpha = solver.tensor_lift(target_alpha);

    // {0, 2}, {1}, {3}
    let target_beta: Tensor<bool> = Tensor::new(
        Shape::new(vec![4, 4]),
        [
            true, false, true, false, //
            false, true, false, true, //
            true, false, true, false, //
            false, true, false, true,
        ]
        .iter()
        .copied()
        .collect(),
    );
    let target_beta = solver.tensor_lift(target_beta);

    // {0, 1}, {2, 3}
    let target_gamma: Tensor<bool> = Tensor::new(
        Shape::new(vec![4, 4]),
        [
            true, true, false, false, //
            true, true, false, false, //
            false, false, true, true, //
            false, false, true, true,
        ]
        .iter()
        .copied()
        .collect(),
    );
    let target_gamma = solver.tensor_lift(target_gamma);

    let node_size = 2;
    let node_vars = solver.tensor_add_variable(Shape::new(vec![target_size, 3, node_size]));
    let node_fun = solver.tensor_one(node_vars.clone());
    solver.tensor_add_clause(&[node_fun]);

    let extr_size = 3;
    let extr_vars = solver.tensor_add_variable(Shape::new(vec![target_size, 6, extr_size]));
    let extr_fun = solver.tensor_one(extr_vars.clone());
    solver.tensor_add_clause(&[extr_fun]);

    let edge_size = extr_size + 2 * node_size;
    let edge_vars = Tensor::create(Shape::new(vec![target_size, 6, edge_size]), |xs| {
        let i = xs[0];
        let j = xs[1];
        let k = xs[2];
        if k < extr_size {
            extr_vars.very_slow_get(&[i, j, k])
        } else if k < extr_size + node_size {
            node_vars.very_slow_get(&[i, j / 2, k - extr_size])
        } else {
            node_vars.very_slow_get(&[i, ((j + 1) % 6) / 2, k - extr_size - node_size])
        }
    });

    let cross1 = edge_vars.polymer(
        Shape::new(vec![target_size, target_size, 6, edge_size, edge_size]),
        &[0, 2, 3],
    );
    let cross2 = edge_vars.polymer(
        Shape::new(vec![target_size, target_size, 6, edge_size, edge_size]),
        &[1, 2, 4],
    );
    let cross = solver.tensor_and(cross1, cross2);

    let source_alpha = solver.tensor_and(
        cross.clone(),
        target_alpha.polymer(
            Shape::new(vec![target_size, target_size, 6, edge_size, edge_size]),
            &[0, 1],
        ),
    );
    let source_alpha = solver.tensor_any(source_alpha);
    let source_alpha = solver.tensor_any(source_alpha);
    let source_alpha = solver.tensor_all(source_alpha);

    let source_beta = solver.tensor_and(
        cross.clone(),
        target_beta.polymer(
            Shape::new(vec![target_size, target_size, 6, edge_size, edge_size]),
            &[0, 1],
        ),
    );
    let source_beta = solver.tensor_any(source_beta);
    let source_beta = solver.tensor_any(source_beta);
    let source_beta = solver.tensor_all(source_beta);

    let source_gamma = solver.tensor_and(
        cross,
        target_gamma.polymer(
            Shape::new(vec![target_size, target_size, 6, edge_size, edge_size]),
            &[0, 1],
        ),
    );
    let source_gamma = solver.tensor_any(source_gamma);
    let source_gamma = solver.tensor_any(source_gamma);
    let source_gamma = solver.tensor_all(source_gamma);

    let mut edge_relation =
        solver.tensor_create(Shape::new(vec![target_size; edge_size]), |_| true);
    for i in 0..(edge_size - 1) {
        for j in (i + 1)..edge_size {
            let r = source_alpha.very_slow_get(&[i, j]);
            let r = Tensor::create(Shape::new(vec![target_size, target_size]), |_| r);
            let r = solver.tensor_imp(r, target_alpha.clone());
            let r = solver.tensor_polymer(r, Shape::new(vec![target_size; edge_size]), &[i, j]);
            edge_relation = solver.tensor_and(edge_relation, r);

            let r = source_beta.very_slow_get(&[i, j]);
            let r = Tensor::create(Shape::new(vec![target_size, target_size]), |_| r);
            let r = solver.tensor_imp(r, target_beta.clone());
            let r = solver.tensor_polymer(r, Shape::new(vec![target_size; edge_size]), &[i, j]);
            edge_relation = solver.tensor_and(edge_relation, r);

            let r = source_gamma.very_slow_get(&[i, j]);
            let r = Tensor::create(Shape::new(vec![target_size, target_size]), |_| r);
            let r = solver.tensor_imp(r, target_gamma.clone());
            let r = solver.tensor_polymer(r, Shape::new(vec![target_size; edge_size]), &[i, j]);
            edge_relation = solver.tensor_and(edge_relation, r);
        }
    }

    for _ in 0..extr_size {
        edge_relation = solver.tensor_any(edge_relation);
    }

    let mut map0 = Vec::new();
    map0.extend(node_size..2 * node_size);
    map0.extend(0..node_size);

    let edge_double = solver.tensor_and(
        edge_relation.clone(),
        edge_relation.polymer(Shape::new(vec![target_size; node_size * 2]), &map0),
    );

    let mut map1 = Vec::new();
    map1.extend(node_size..2 * node_size);
    map1.extend(0..node_size);

    let mut map2 = Vec::new();
    map2.extend(0..node_size);
    map2.extend(2 * node_size..3 * node_size);

    let mut edge_trans = solver.tensor_and(
        edge_double.polymer(Shape::new(vec![target_size; node_size * 3]), &map1),
        edge_double.polymer(Shape::new(vec![target_size; node_size * 3]), &map2),
    );

    for _ in 0..node_size {
        edge_trans = solver.tensor_any(edge_trans);
    }

    let mut test = edge_trans;
    for j in 0..(2 * node_size) {
        let v = Tensor::create(Shape::new(vec![target_size]), |xs| {
            if j < node_size {
                node_vars.very_slow_get(&[xs[0], 1, j])
            } else {
                node_vars.very_slow_get(&[xs[0], 0, j - node_size])
            }
        });
        let v = v.polymer(test.shape().clone(), &[0]);
        test = solver.tensor_and(test, v);
        test = solver.tensor_any(test);
    }

    let test = solver.tensor_not(test);
    solver.tensor_add_clause(&[test]);

    let result = solver.tensor_find_one_model(
        &[],
        &[
            edge_vars,
            source_alpha,
            source_beta,
            source_gamma,
            edge_relation,
        ],
    );
    println!("{:?}", result);
}
