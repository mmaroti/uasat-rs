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

pub const OPERATORS: &'static str = "()[],=";

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Identifier,
    Operator,
    Integer,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    data: &'a str,
    kind: Kind,
    line: u32,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        return fmt::Debug::fmt(self, f);
    }
}

pub struct Lexer<'a> {
    data: &'a str,
    iter: str::CharIndices<'a>,
    offset: usize,
    next: Option<char>,
    line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut iter = data.char_indices();
        let (offset, next) = match iter.next() {
            Some((o, c)) => (o, Some(c)),
            None => (0, None),
        };
        return Lexer {
            data: data,
            iter: iter,
            offset: offset,
            next: next,
            line: 1,
        };
    }

    #[inline]
    fn read_char(self: &mut Self) {
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

    fn get_range(self: &mut Self, kind: Kind, pred: impl Fn(char) -> bool) -> Token<'a> {
        let o = self.offset;
        while let Some(c) = self.next {
            if pred(c) {
                self.read_char();
            } else {
                break;
            }
        }
        let d = unsafe { self.data.get_unchecked(o..self.offset) };
        return Token {
            data: d,
            kind: kind,
            line: self.line,
        };
    }

    fn get_single(self: &mut Self, kind: Kind) -> Token<'a> {
        let o = self.offset;
        self.read_char();
        let d = unsafe { self.data.get_unchecked(o..self.offset) };
        return Token {
            data: d,
            kind: kind,
            line: self.line,
        };
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(self: &mut Self) -> Option<Self::Item> {
        while let Some(c) = self.next {
            if c.is_alphabetic() {
                let p = |c: char| c.is_alphanumeric() || c == '_';
                return Some(self.get_range(Kind::Identifier, p));
            } else if c.is_digit(10) {
                let p = |c: char| c.is_digit(10);
                return Some(self.get_range(Kind::Integer, p));
            } else if OPERATORS.contains(c) {
                return Some(self.get_single(Kind::Operator));
            } else if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                }
                self.read_char();
            } else {
                return Some(self.get_single(Kind::Unknown));
            }
        }
        return None;
    }
}

impl<'a> iter::FusedIterator for Lexer<'a> {}
