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

use std::{fmt, iter, str};

pub const OPERATORS: &str = "()[],=";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Identifier,
    Operator,
    Integer,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub data: &'a str,
    pub kind: Kind,
    pub line: u32,
    pub column: u32,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Lexer<'a> {
    iter: str::CharIndices<'a>,
    offset: usize,
    next: Option<char>,
    line: u32,
    column: u32,
    data: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut iter = data.char_indices();
        let (offset, next) = match iter.next() {
            Some((o, c)) => (o, Some(c)),
            None => (0, None),
        };
        Lexer {
            data,
            iter,
            offset,
            next,
            line: 1,
            column: 1,
        }
    }

    #[inline]
    fn read_char(&mut self) {
        match self.iter.next() {
            Some((p, c)) => {
                self.offset = p;
                self.next = Some(c);
                self.column += 1;
            }
            None => {
                self.offset = self.data.len();
                self.next = None;
            }
        };
    }

    fn get_range(&mut self, pred: impl Fn(char) -> bool) -> &'a str {
        let offset = self.offset;
        while let Some(c) = self.next {
            if pred(c) {
                self.read_char();
            } else {
                break;
            }
        }
        self.data.get(offset..self.offset).unwrap()
    }

    fn get_single(&mut self) -> &'a str {
        let offset = self.offset;
        self.read_char();
        self.data.get(offset..self.offset).unwrap()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.next {
            if c.is_alphabetic() || c == '_' {
                return Some(Token {
                    kind: Kind::Identifier,
                    line: self.line,
                    column: self.column,
                    data: self.get_range(|c: char| c.is_alphanumeric() || c == '_'),
                });
            } else if c.is_digit(10) {
                return Some(Token {
                    kind: Kind::Integer,
                    line: self.line,
                    column: self.column,
                    data: self.get_range(|c: char| c.is_digit(10)),
                });
            } else if OPERATORS.contains(c) {
                return Some(Token {
                    kind: Kind::Operator,
                    line: self.line,
                    column: self.column,
                    data: self.get_single(),
                });
            } else if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.read_char();
            } else {
                return Some(Token {
                    kind: Kind::Unknown,
                    line: self.line,
                    column: self.column,
                    data: self.get_single(),
                });
            }
        }
        None
    }
}

impl<'a> iter::FusedIterator for Lexer<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let mut lexer = Lexer::new("ab\n ,12");

        assert_eq!(
            lexer.next(),
            Some(Token {
                data: "ab",
                kind: Kind::Identifier,
                line: 1,
                column: 1
            })
        );

        assert_eq!(
            lexer.next(),
            Some(Token {
                data: ",",
                kind: Kind::Operator,
                line: 2,
                column: 2
            })
        );

        assert_eq!(
            lexer.next(),
            Some(Token {
                data: "12",
                kind: Kind::Integer,
                line: 2,
                column: 3
            })
        );

        assert_eq!(lexer.next(), None);
    }
}
