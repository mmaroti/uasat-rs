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

pub const OPERATORS: &'static str = "()[]{}<>=,.?!:;*/+-@%&'\"`_#|~";

#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnexpectedChar(char),
    IntegerTooLarge,
}

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    Operator(char),
    Integer(u32),
    Error(Error),
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub line: u32,
    pub column: u32,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        return fmt::Debug::fmt(self, f);
    }
}

impl fmt::Display for Pos {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "line {} column {}", self.line, self.column);
    }
}

pub struct Lexer<'a> {
    iter: str::CharIndices<'a>,
    pos: Pos,
    offset: usize,
    next: Option<char>,
    data: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut iter = data.char_indices();
        let (offset, next) = match iter.next() {
            Some((o, c)) => (o, Some(c)),
            None => (0, None),
        };
        return Lexer {
            iter: iter,
            pos: Pos { line: 1, column: 1 },
            offset: offset,
            next: next,
            data: data,
        };
    }

    fn read_char(self: &mut Self) {
        if self.next.unwrap_or_default() == '\n' {
            self.pos.line += 1;
            self.pos.column = 1;
        } else {
            self.pos.column += 1;
        }

        match self.iter.next() {
            Some((p, c)) => {
                self.offset = p;
                self.next = Some(c);
            }
            None => {
                self.offset = self.data.len();
                self.next = None;
            }
        };
    }

    fn get_error(self: &mut Self, error: Error) -> Token<'a> {
        self.next = None;
        return Token::Error(error);
    }

    fn add_digit(n: u32, d: u32) -> Option<u32> {
        return match n.checked_mul(10) {
            Some(n2) => match n2.checked_add(d) {
                Some(n3) => Some(n3),
                None => None,
            },
            None => None,
        };
    }

    fn get_integer(self: &mut Self) -> Token<'a> {
        let mut n: u32 = 0;
        while let Some(c) = self.next {
            match c.to_digit(10) {
                Some(d) => match Lexer::add_digit(n, d) {
                    Some(n2) => n = n2,
                    None => return self.get_error(Error::IntegerTooLarge),
                },
                None => break,
            }
            self.read_char();
        }
        return Token::Integer(n);
    }

    fn get_identifier(self: &mut Self) -> Token<'a> {
        let o = self.offset;
        while let Some(c) = self.next {
            if c.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }
        let s = unsafe { self.data.get_unchecked(o..self.offset) };
        return Token::Identifier(s);
    }

    fn get_operator(self: &mut Self) -> Token<'a> {
        let c = self.next.unwrap();
        self.read_char();
        return Token::Operator(c);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token<'a>, Pos);

    fn next(self: &mut Self) -> Option<Self::Item> {
        while let Some(c) = self.next {
            let p = self.pos;
            if c.is_alphabetic() {
                return Some((self.get_identifier(), p));
            } else if c.is_digit(10) {
                return Some((self.get_integer(), p));
            } else if OPERATORS.contains(c) {
                return Some((self.get_operator(), p));
            } else if c.is_whitespace() {
                self.read_char();
            } else {
                self.read_char();
                return Some((self.get_error(Error::UnexpectedChar(c)), p));
            }
        }
        return None;
    }
}

impl<'a> iter::FusedIterator for Lexer<'a> {}
