/*
* Copyright (C) 2022, Miklos Maroti
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

//! Module for working with abstract data types.

#[allow(unused_imports)]
use super::core::{BooleanLogic, BooleanSolver, Logic, Solver};
use super::genvec::{BitSlice, BitVec, Slice, Vector};

mod binary_relations;
pub use binary_relations::*;

mod boolean;
pub use boolean::*;

mod fixed_set;
pub use fixed_set::*;

mod operations;
pub use operations::*;

mod permutations;
pub use permutations::*;

mod power;
pub use power::*;

mod product;
pub use product::*;

mod relations;
pub use relations::*;

mod small_set;
pub use small_set::*;

mod traits;
pub use traits::*;

mod unary_operations;
pub use unary_operations::*;

mod wrap_elem;
pub use wrap_elem::*;

mod preservation;
pub use preservation::*;

#[cfg(test)]
mod validate;

pub fn test() {
    let logic = Logic();
    let dom = Permutations::new(SmallSet::new(3));
    for i in 0..dom.size() {
        let elem = dom.get_elem(&logic, i);
        println!("{}\t{}", i, dom.format(elem.slice()));
        let j = dom.get_index(elem.slice());
        println!("{}", j);
    }
}
