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

#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;

mod lexer;

#[cfg(feature = "console_error_panic_hook")]
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn uasat_init() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn test(input: String) -> String {
    let lexer = lexer::Lexer::new(input.as_str());
    let mut output = String::new();
    for token in lexer {
        output.push_str(format!("{}\n", token).as_str());
    }
    output
}
