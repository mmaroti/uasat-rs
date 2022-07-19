/*
* Copyright (C) 2021, Miklos Maroti
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

//! Module for the core components that seems to have stabilized.

// mod bitvec;

mod genvec;
pub(crate) use genvec::GenericVec;
pub use genvec::{GenericElem, GenericVector};

mod solver;
pub use solver::{create_solver, Literal, SatInterface};

mod tensor;
pub use tensor::{Shape, Tensor, TensorAlgebra, TensorSolver};

mod boolean;
pub use boolean::{BooleanAlgebra, BooleanSolver, Bools, Solver};

mod progress;
pub use progress::{add_progress, del_progress, set_progress};
