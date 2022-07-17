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

use super::{binrel, BinaryRel};
use crate::core::{
    add_progress, del_progress, set_progress, Bools, Literal, Shape, Solver, Tensor, TensorAlgebra,
    TensorSolver,
};

struct Extension {
    alg: Solver,
    source_graph: Tensor<Literal>,
    extension_map: Tensor<Literal>,
}

impl Extension {
    pub fn new(solver_name: &str, partial_map: &Tensor<bool>, target_graph: &Tensor<bool>) -> Self {
        let mut alg = Solver::new(solver_name);

        let source_size = partial_map.shape()[0];
        let partial_map = alg.tensor_lift(partial_map.clone());
        let source_graph = alg.tensor_add_variable(Shape::new(vec![source_size, source_size]));
        let target_graph = alg.tensor_lift(target_graph.clone());
        let extension_map = alg.tensor_add_variable(partial_map.shape().clone());

        let tmp = alg.is_function(extension_map.clone());
        alg.tensor_add_clause1(tmp);

        let tmp = alg.is_subset_of(partial_map, extension_map.clone());
        alg.tensor_add_clause1(tmp);

        let tmp = alg.is_compatible(extension_map.clone(), source_graph.clone(), target_graph);
        alg.tensor_add_clause1(tmp);

        Extension {
            alg,
            source_graph,
            extension_map,
        }
    }

    pub fn find(&mut self, source_graph: Tensor<bool>) -> Option<Tensor<bool>> {
        let source_graph = self.alg.tensor_lift(source_graph);
        let source_graph = self.alg.tensor_equ(source_graph, self.source_graph.clone());
        let result = self
            .alg
            .tensor_find_one_model(&[source_graph], &[self.extension_map.clone()]);

        result.map(|mut v| {
            assert_eq!(v.len(), 1);
            v.pop().unwrap()
        })
    }
}

pub struct Blocker {
    trace: bool,
    solver_name: String,
    partial_map: Tensor<bool>,
    target_graph: Tensor<bool>,
    extension: Extension,
}

impl Blocker {
    pub fn new(solver_name: &str, partial_map: &[isize], target_graph: Tensor<bool>) -> Self {
        let target_size = target_graph.shape()[0];
        assert_eq!(target_size, target_graph.shape()[1]);

        let boolean = Bools();
        let partial_map = boolean.create_partial_map(partial_map, target_size);
        let extension = Extension::new(solver_name, &partial_map, &target_graph);

        Blocker {
            trace: false,
            solver_name: solver_name.into(),
            partial_map,
            target_graph,
            extension,
        }
    }

    pub fn source_size(&self) -> usize {
        self.partial_map.shape()[0]
    }

    pub fn target_size(&self) -> usize {
        self.partial_map.shape()[1]
    }

    pub fn find_extension1(&self, source_graph: Tensor<bool>) -> Option<Tensor<bool>> {
        let mut alg = Solver::new(&self.solver_name);

        let partial_map = alg.tensor_lift(self.partial_map.clone());
        let source_graph = alg.tensor_lift(source_graph);
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

    pub fn find_extension2(&mut self, source_graph: Tensor<bool>) -> Option<Tensor<bool>> {
        self.extension.find(source_graph)
    }

    pub fn find_source_graph(&mut self) -> Option<Tensor<bool>> {
        let mut alg = Solver::new(&self.solver_name);

        let target_graph2 = alg.tensor_lift(self.target_graph.clone());

        let shape = Shape::new(vec![self.source_size(), self.source_size()]);
        let source_graph = alg.tensor_add_variable(shape);

        for i in 0..self.source_size() {
            for j in 0..self.target_size() {
                if self.partial_map.very_slow_get(&[i, j]) {
                    let mut submap = self.partial_map.clone();
                    submap.very_slow_set(&[i, j], false);
                    let submap = alg.tensor_lift(submap);

                    let map = alg.tensor_add_variable(submap.shape().clone());

                    let tmp = alg.is_function(map.clone());
                    alg.tensor_add_clause1(tmp);

                    let tmp = alg.is_subset_of(submap, map.clone());
                    alg.tensor_add_clause1(tmp);

                    let target_graph = alg.tensor_lift(self.target_graph.clone());
                    let tmp = alg.is_compatible(map.clone(), source_graph.clone(), target_graph);
                    alg.tensor_add_clause1(tmp);
                }
            }
        }

        add_progress("excluded");

        let mut excluded = 0;
        let mut minimal = None;
        loop {
            let result = alg.tensor_find_one_model1(source_graph.clone());
            if result.is_none() {
                break;
            }

            let result = result.unwrap();
            let extension = self.find_extension2(result.clone());
            if extension.is_none() {
                minimal = Some(result.clone());
                let result = alg.tensor_lift(result);
                let tmp = alg.is_proper_subset_of(source_graph.clone(), result);
                alg.tensor_add_clause1(tmp);
                continue;
            }

            if self.trace {
                println!("excluding {:?}", extension.as_ref().unwrap());
            }
            excluded += 1;
            set_progress("excluded", excluded);
            let extension = alg.tensor_lift(extension.unwrap());
            let tmp = alg.is_compatible(extension, source_graph.clone(), target_graph2.clone());
            let tmp = alg.tensor_not(tmp);
            alg.tensor_add_clause1(tmp);
        }

        del_progress("excluded");
        minimal
    }
}

pub fn test() {
    let boolean = Bools();

    let target_graph = boolean.create_crown_poset(6);
    let partial_map = [0, 0, 0, 1, 3, 4, -1, -1, -1, -1, -1];
    let mut blocker = Blocker::new("cadical", &partial_map, target_graph);

    let source_graph = blocker.find_source_graph();
    if source_graph.is_none() {
        println!("source: None");
    } else {
        println!(
            "source: {:?}",
            binrel::edges(source_graph.as_ref().unwrap())
        );
    }
    println!("target: {:?}", binrel::edges(&blocker.target_graph));
}
