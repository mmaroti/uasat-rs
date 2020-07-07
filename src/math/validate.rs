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
use crate::tensor::{Shape, Solver, TensorAlg, TensorSat};
use std::time::Instant;

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
pub fn validate_solver(solver: &str) {
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
        Shape::new(vec![5, 6]),
        <Solver as BinaryRel>::is_function,
        7776,
    );

    let duration = Instant::now().duration_since(start).as_secs_f32();
    println!("Solver {} finished in {} seconds\n", solver, duration);
}

pub fn validate() {
    #[cfg(feature = "cadical")]
    validate_solver("cadical");
    #[cfg(feature = "batsat")]
    validate_solver("batsat");
    #[cfg(feature = "minisat")]
    validate_solver("minisat");
    #[cfg(feature = "varisat")]
    validate_solver("varisat");
    #[cfg(feature = "cryptominisat")]
    validate_solver("cryptominisat");
}
