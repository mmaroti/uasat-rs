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

pub mod array;
pub mod boolalg;
mod lexer;

fn main() {
    println!("{}", std::mem::size_of::<lexer::Token>());
    let data = "a\n(1234567890123456789, ab)\nHello, world! Continue";
    // let data = &std::fs::read_to_string("LICENSE").unwrap();

    let lexer = lexer::Lexer::new(data);
    for item in lexer {
        println!("{}", item);
    }
}
