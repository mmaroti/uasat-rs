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

use super::core::{BooleanAlgebra, BooleanSolver, Solver};
use super::genvec::{GenSlice, GenVec, SliceFor, VecFor};

mod boolean;
pub use boolean::*;

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

pub fn test() {
    let alg = Product2::new(
        Power::new(BOOLEAN, Power::new(SmallSet::new(4), TWO)),
        SmallSet::new(7),
    );
    let elem = alg.find_element().unwrap();
    println!("{}", alg.format(elem.slice()));
}
