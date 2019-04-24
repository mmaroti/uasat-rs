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

use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;
use nom::ErrorKind;

pub enum ParseErr {
    NumberTooLarge = 1,
}

const fn error_code(err: ParseErr) -> ErrorKind {
    ErrorKind::Custom(err as u32)
}

named!(parse_int(&str) -> u32,
    return_error!(
        error_code(ParseErr::NumberTooLarge), 
        map_res!(nom::digit, FromStr::from_str)
    )
);

named!(parse_ints(&str) -> Vec<u32>,
    delimited!(
        tag!("("), 
        separated_list!(tag!(","), parse_int), 
        tag!(")")
    )
);

lazy_static! {
    static ref ERROR_MAP: HashMap<u32, u32> = {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map
    };
}

pub fn handle_error(err: &nom::Err<&str>) {
    match err {
        nom::Err::Incomplete(_) => {
            println!("input is incomplete");
        }
        nom::Err::Error(ctx) | nom::Err::Failure(ctx) => {
            let list = nom::error_to_list(ctx);
            println!("error {:?}", list);
        }
    };
}

pub fn parse(text: &str) -> Result<Vec<u32>, String> {
    match parse_ints(text) {
        Ok((_, result)) => Ok(result),
        Err(err) => {
            println!("{:?}", err);
            handle_error(&err);
            Err(err.description().to_string())
        }
    }
}

pub fn test() {
    println!("{:?}", parse("(12345678111,22)"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_int() {
        assert_eq!(parse_ints("(12,3)"), Ok(("", vec![12, 3])));
    }
}
