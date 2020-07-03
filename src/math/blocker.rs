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

use crate::math::BinaryRel;
use crate::tensor::{Shape, Solver, Tensor, TensorAlg, TensorSat, BOOLEAN};

pub struct Blocker {
    solver: String,
    partial_map: Tensor<bool>,
    target_graph: Tensor<bool>,
}

impl Blocker {
    pub fn find_extension(&self, source_graph: Tensor<bool>) -> Option<Tensor<bool>> {
        let mut alg = Solver::new(&self.solver);

        let partial_map = alg.tensor_lift(self.partial_map.clone());
        let source_graph = alg.tensor_lift(source_graph.clone());
        let target_graph = alg.tensor_lift(self.target_graph.clone());
        let map = alg.tensor_add_variable(partial_map.shape().clone());

        let tmp = alg.is_function(map.clone());
        alg.tensor_add_clause1(tmp);

        let tmp = alg.is_subset_of(partial_map, map.clone());
        alg.tensor_add_clause1(tmp);

        let tmp = alg.is_compatible(map.clone(), source_graph, target_graph);
        alg.tensor_add_clause1(tmp);

        alg.tensor_find_one_model1(map)
    }

    pub fn find_source_graph(&self) -> Option<Tensor<bool>> {
        let mut alg = Solver::new(&self.solver);

        let target_graph2 = alg.tensor_lift(self.target_graph.clone());

        let size = self.partial_map.shape()[0];
        let shape = Shape::new(vec![size, size]);
        let source_graph = alg.tensor_add_variable(shape);

        loop {
            let result = alg.tensor_find_one_model1(source_graph.clone());
            if result.is_none() {
                return None;
            }

            let result = result.unwrap();
            let extension = self.find_extension(result.clone());
            if extension.is_none() {
                return Some(result);
            }

            println!("excluding {:?}", extension.as_ref());
            let extension = alg.tensor_lift(extension.unwrap());
            let tmp = alg.is_compatible(extension, source_graph.clone(), target_graph2.clone());
            let tmp = alg.tensor_not(tmp);
            alg.tensor_add_clause1(tmp);
        }
    }
}
pub fn create_partial_map(map: &[isize], target: usize) -> Tensor<bool> {
    let shape = Shape::new(vec![map.len(), target]);
    Tensor::create(shape, |i| map[i[0]] == i[1] as isize)
}

pub fn test() {
    let target_graph = BOOLEAN.crown_poset(4);
    let partial_map = create_partial_map(&[0, 2], target_graph.shape()[0]);

    let blocker = Blocker {
        solver: "cadical".into(),
        partial_map,
        target_graph,
    };

    let source_graph = blocker.find_source_graph();
    println!("source: {:?}", &source_graph);
    println!("target: {:?}", &blocker.target_graph);
}
