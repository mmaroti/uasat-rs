/*
* Copyright (C) 2019-2020, Miklos Maroti
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

pub mod boolean;
pub mod genvec;
pub mod solver;
pub mod tensor;

pub mod math;
pub mod old;

#[cfg(feature = "console_error_panic_hook")]
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn uasat_init() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

pub fn main() {
    math::binrel::test();
}
