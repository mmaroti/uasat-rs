/*
* Copyright (C) 2022-2024, Miklos Maroti
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

#![allow(unused_imports)]

use super::core::{BooleanLogic, BooleanSolver, Literal, SatInterface, Solver, LOGIC};
use super::genvec::{BitSlice, BitVec, Slice, Vector};

mod solver;
pub use solver::*;

mod traits;
pub use traits::*;

mod boolean;
pub use boolean::*;
