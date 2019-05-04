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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error {
    err: &'static str,
    pos: *const u8,
}

pub trait Parser: Sized + Clone {
    type Output;

    fn parse(self: &Self, text: &mut &str) -> Result<Self::Output, Error>;

    fn map<O, F>(self: Self, fun: F) -> Map<Self, O, F>
    where
        F: Fn(Self::Output) -> Result<O, Error> + Clone,
    {
        Map { par: self, fun }
    }

    fn pair<P>(self: Self, par: P) -> (Self, P)
    where
        P: Parser,
    {
        (self, par)
    }

    fn opt(self: Self) -> Opt<Self> {
        Opt(self)
    }
}

#[derive(Clone, Debug)]
pub struct Tag(&'static str, &'static str);

impl Parser for Tag {
    type Output = &'static str;

    fn parse(self: &Self, text: &mut &str) -> Result<Self::Output, Error> {
        let len = self.0.len();
        if Some(self.0) == text.get(..len) {
            *text = &text[len..];
            Ok(self.0)
        } else {
            Err(Error {
                err: self.1,
                pos: text.as_ptr(),
            })
        }
    }
}

impl<P0: Parser, P1: Parser> Parser for (P0, P1) {
    type Output = (P0::Output, P1::Output);

    fn parse(self: &Self, text: &mut &str) -> Result<Self::Output, Error> {
        let old = *text;
        match self.0.parse(text) {
            Err(err) => {
                debug_assert!(old == *text);
                Err(err)
            }
            Ok(val0) => match self.1.parse(text) {
                Err(err) => {
                    *text = old;
                    Err(err)
                }
                Ok(val1) => Ok((val0, val1)),
            },
        }
    }
}

#[derive(Debug)]
pub struct Map<P, O, F>
where
    P: Parser,
    F: Fn(P::Output) -> Result<O, Error> + Clone,
{
    par: P,
    fun: F,
}

impl<P, O, F> Parser for Map<P, O, F>
where
    P: Parser,
    F: Fn(P::Output) -> Result<O, Error> + Clone,
{
    type Output = O;

    fn parse(self: &Self, text: &mut &str) -> Result<Self::Output, Error> {
        let old = *text;
        match self.par.parse(text) {
            Err(err) => {
                debug_assert!(old == *text);
                Err(err)
            }
            Ok(val) => {
                let fun = &self.fun;
                let val = fun(val);
                if val.is_err() {
                    *text = old;
                }
                val
            }
        }
    }
}

impl<P, O, F> Clone for Map<P, O, F>
where
    P: Parser,
    F: Fn(P::Output) -> Result<O, Error> + Clone,
{
    fn clone(&self) -> Self {
        Map {
            par: self.par.clone(),
            fun: self.fun.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Opt<P>(P)
where
    P: Parser;

impl<P> Parser for Opt<P>
where
    P: Parser,
{
    type Output = Option<P::Output>;

    fn parse(self: &Self, text: &mut &str) -> Result<Self::Output, Error> {
        match self.0.parse(text) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_tag() {
        let text = &mut "212";

        let tag1 = Tag("1", "expected one");
        assert!(tag1.parse(text).is_err());
        assert_eq!(text.len(), 3);

        let tag2 = Tag("2", "expected two");
        assert_eq!(tag2.parse(text), Ok("2"));
        assert_eq!(text.len(), 2);

        assert!((tag1.clone(), tag1.clone()).parse(text).is_err());
        assert_eq!(text.len(), 2);

        assert_eq!((tag1.clone(), tag2.clone()).parse(text), Ok(("1", "2")));
        assert_eq!(text.len(), 0);

        *text = &mut "12";
        let tag3 = tag1.map(|_| Ok(true));
        assert_eq!(tag3.parse(text), Ok(true));
    }
}
