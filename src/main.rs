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

//! A SAT based discrete mathematics and universal algebra calculator.

pub mod binary;
pub mod boolean;
pub mod clone;
pub mod genvec;
pub mod lexer;
pub mod parser;
pub mod solver;
pub mod tensor;

#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use solver::*;
#[cfg(feature = "console_error_panic_hook")]
use std::panic;
use std::time::Instant;
use wasm_bindgen::prelude::*;

use boolean::{BoolAlg, BoolSat};
use tensor::{Shape, Solver, TensorAlg, TensorSat};

#[wasm_bindgen(start)]
pub fn uasat_init() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

pub fn test_solver(solver_name: &str, size: usize) -> String {
    let start = Instant::now();

    let mut sol = create_solver(solver_name);
    let mut table: Vec<Literal> = Vec::with_capacity(size * size);

    // create literals
    for _ in 0..(size * size) {
        table.push(sol.add_variable());
    }

    // reflexive
    for i in 0..size {
        sol.add_clause(&[table[i * size + i]]);
    }

    // symmetric
    for i in 0..size {
        for j in 0..size {
            sol.add_clause(&[sol.negate(table[i * size + j]), table[j * size + i]]);
        }
    }

    // transitive
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                sol.add_clause(&[
                    sol.negate(table[i * size + j]),
                    sol.negate(table[j * size + k]),
                    table[i * size + k],
                ]);
            }
        }
    }

    // find all solutions
    let mut count = 0;
    while sol.solve() {
        count += 1;
        let lits: Vec<Literal> = table
            .iter()
            .map(|lit| {
                if sol.get_value(*lit) {
                    sol.negate(*lit)
                } else {
                    *lit
                }
            })
            .collect();
        sol.add_clause(&lits);
    }

    let duration = Instant::now().duration_since(start);
    format!(
        "Test1 {} result {} in {:?}",
        sol.get_name(),
        count,
        duration
    )
}

pub fn test_solver2(solver_name: &str, size: usize) -> String {
    let start = Instant::now();

    let mut sol = Solver::new(solver_name);
    let rel = sol.tensor_add_variable(Shape::new(vec![size, size]));

    let rfl = sol.polymer(&rel, Shape::new(vec![size]), &[0, 0]);
    let rfl = sol.tensor_all(&rfl, 1);
    sol.tensor_add_clause(&[&rfl]);

    let inv = sol.polymer(&rel, Shape::new(vec![size, size]), &[1, 0]);
    //let neg = sol.tensor_not(&rel);
    //sol.add_clause(&[&neg, &inv]);
    let imp = sol.tensor_leq(&rel, &inv);
    let imp = sol.tensor_all(&imp, 2);
    sol.tensor_add_clause(&[&imp]);

    let r01 = sol.polymer(&rel, Shape::new(vec![size, size, size]), &[0, 1]);
    let r01 = sol.tensor_not(&r01);
    let r12 = sol.polymer(&rel, Shape::new(vec![size, size, size]), &[1, 2]);
    let r12 = sol.tensor_not(&r12);
    let r02 = sol.polymer(&rel, Shape::new(vec![size, size, size]), &[0, 2]);
    sol.tensor_add_clause(&[&r01, &r12, &r02]);

    // find all solutions
    let mut count = 0;
    while sol.tensor_find_model() {
        count += 1;
        let rel2 = sol.tensor_get_value(&rel);
        let mut lits = Vec::new();
        lits.resize(size * size, rel.__slow_get__(&[0, 0]));
        for i in 0..size {
            for j in 0..size {
                let lit = rel.__slow_get__(&[i, j]);
                if rel2.__slow_get__(&[i, j]) {
                    lits[i * size + j] = sol.bool_not(lit)
                } else {
                    lits[i * size + j] = lit;
                }
            }
        }
        sol.bool_add_clause(&lits);
    }

    let duration = Instant::now().duration_since(start);
    format!(
        "Test2 {} result {} in {:?}",
        sol.get_name(),
        count,
        duration
    )
}

#[wasm_bindgen]
pub fn test(input: String) -> String {
    let lexer = lexer::Lexer::new(input.as_str());
    let mut output = String::new();
    for token in lexer {
        output.push_str(format!("{}\n", token).as_str());
    }
    #[cfg(feature = "varisat")]
    output.push_str(&test_solver("varisat", 7));
    // output.push_str(&format!("{:?}\n", parser::parse(&input)));
    output
}

fn main() {
    #[cfg(feature = "minisat")]
    println!("{}", test_solver("minisat", 8));
    #[cfg(feature = "minisat")]
    println!("{}", test_solver2("minisat", 8));
    #[cfg(feature = "varisat")]
    println!("{}", test_solver("varisat", 8));
    #[cfg(feature = "varisat")]
    println!("{}", test_solver2("varisat", 8));
    #[cfg(feature = "cryptominisat")]
    println!("{}", test_solver("cryptominisat", 8));
    #[cfg(feature = "cryptominisat")]
    println!("{}", test_solver2("cryptominisat", 8));
}
