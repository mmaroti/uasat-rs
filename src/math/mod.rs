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

//! Module for doing research experimentation using the uasat crate.

mod binrel;
mod blocker;
mod extremeconn;
mod obstruction;
mod test;
mod validate;
mod taylor;

pub use binrel::BinaryRel;
pub use blocker::test as blocker_test;
pub use extremeconn::test as extremeconn_test;
pub use obstruction::test as obstruction_test;
pub use validate::validate;
pub use taylor::main as taylor_main;