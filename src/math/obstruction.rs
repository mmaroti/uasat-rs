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

use super::binrel::BinaryRel;
use crate::core::{
    BooleanLogic, BooleanSolver, Logic, Literal, Shape, Solver, Tensor, TensorAlgebra,
    TensorSolver,
};

struct Obstruction {
    solver: Solver,
    source: Tensor<Literal>,
    target: Tensor<Literal>,
    maps: Vec<Tensor<Literal>>,
}

impl Obstruction {
    pub fn new(solver: &str, source_size: usize, target_size: usize) -> Self {
        let mut solver = Solver::new(solver);
        let source = solver.tensor_add_variable(Shape::new(vec![source_size, source_size]));
        let target = solver.tensor_add_variable(Shape::new(vec![target_size, target_size]));
        Self {
            solver,
            source,
            target,
            maps: Vec::default(),
        }
    }

    pub fn source_size(&self) -> usize {
        self.source.shape()[0]
    }

    pub fn target_size(&self) -> usize {
        self.target.shape()[0]
    }

    pub fn set_source_edge(&mut self, elem1: usize, elem2: usize, value: bool) {
        let lit = self.source.very_slow_get(&[elem1, elem2]);
        let lit = self.solver.bool_xor(lit, self.solver.bool_lift(value));
        self.solver.bool_add_clause(&[lit]);
    }

    pub fn set_source_graph(&mut self, graph: Tensor<bool>) {
        let graph = self.solver.tensor_lift(graph);
        let graph = self.solver.tensor_xor(self.source.clone(), graph);
        self.solver.tensor_add_clause1(graph);
    }

    pub fn set_target_edge(&mut self, elem1: usize, elem2: usize, value: bool) {
        let lit = self.target.very_slow_get(&[elem1, elem2]);
        let lit = self.solver.bool_xor(lit, self.solver.bool_lift(value));
        self.solver.bool_add_clause(&[lit]);
    }

    pub fn set_target_graph(&mut self, graph: Tensor<bool>) {
        let graph = self.solver.tensor_lift(graph);
        let graph = self.solver.tensor_xor(self.target.clone(), graph);
        self.solver.tensor_add_clause1(graph);
    }

    pub fn add_map(&mut self) -> usize {
        let idx = self.maps.len();
        let map = self
            .solver
            .tensor_add_variable(Shape::new(vec![self.source_size(), self.target_size()]));
        self.maps.push(map);
        idx
    }
}

pub fn test() {
    let mut boolean = Logic();

    let mut obst = Obstruction::new("", 2, 6);

    let target = boolean.create_from_edges(
        6,
        6,
        &[
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (1, 1),
            (1, 3),
            (1, 4),
            (1, 5),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (3, 3),
            (3, 5),
            (4, 4),
            (4, 5),
            (5, 5),
        ],
    );
    println!(
        "partial order: {}",
        boolean.is_partial_order(target.clone()).scalar()
    );

    obst.set_target_graph(target);
}
