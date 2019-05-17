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

pub mod boolalg;
pub mod genvec;
pub mod lexer;
pub mod parser;
pub mod posets;
pub mod semantics;
pub mod tensor;

#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use boolalg::*;
#[cfg(feature = "console_error_panic_hook")]
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn uasat_init() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

pub fn test_solver(solver_name: &str, size: usize) -> String {
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

    format!("{} result {}\n", sol.get_name(), count)
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
    // #[cfg(feature = "minisat")]
    // println!("MiniSat {}", test_solver("minisat", 9));
    // #[cfg(feature = "varisat")]
    // println!("VariSat {}", test_solver("varisat", 9));
}
