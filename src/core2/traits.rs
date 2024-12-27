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

use std::fmt::Debug;
use std::iter::FusedIterator;
use std::num::NonZeroI32;

use super::{BitSlice, BitVec, BoolLogic, BooleanLogic, Literal, Slice, Solver};

/// An arbitrary set of elements that can be representable by bit vectors.
pub trait Domain: Debug {
    /// Returns the number of bits used to represent the elements of the
    /// domain.
    fn num_bits(&self) -> usize;

    /// Returns an object for formatting the given element.
    fn format<'a>(&'a self, elem: BitSlice<'a>) -> Format<'a>
    where
        Self: Sized,
    {
        Format::new(self, elem)
    }

    /// Formats the given element using the provided formatter.
    fn display_elem(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        elem: BitSlice<'_>,
    ) -> std::fmt::Result {
        assert!(elem.len() == self.num_bits());
        for v in elem.copy_iter() {
            write!(f, "{}", if v { '1' } else { '0' })?;
        }
        Ok(())
    }
}

/// A helper structure for displaying domain elements.
pub struct Format<'a> {
    base: &'a dyn Domain,
    elem: BitSlice<'a>,
}

impl<'a> Format<'a> {
    /// Returns an object for formatting the given element.
    pub fn new<DOM>(dom: &'a DOM, elem: BitSlice<'a>) -> Self
    where
        DOM: Domain,
    {
        let base = dom as &'a dyn Domain;
        Self { base, elem }
    }
}

impl std::fmt::Display for Format<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.display_elem(f, self.elem)
    }
}

/// A domain where the elements can be counted and indexed.
pub trait Indexable: Domain {
    /// Returns the number of elements of the domain.
    fn size(&self) -> usize;

    /// Returns the given element of the domain.
    fn get_elem(&self, index: usize) -> BitVec;

    /// Returns the index of the given element.
    fn get_index(&self, elem: BitSlice<'_>) -> usize;
}

pub type LitSlice<'a> = &'a [Literal];
pub type LitVec = Vec<Literal>;

pub trait Function: Debug {
    /// Returns the arity of the function
    fn arity(&self) -> usize {
        self.domains().len()
    }

    /// Returns the domain of the function.
    fn domains(&self) -> &[&dyn Domain];

    /// Returns the codomain of the function.
    fn codomain(&self) -> &dyn Domain;

    /// Calculates the complement of the given element.
    fn evaluate1(&self, elems: &[BitSlice<'_>]) -> BitVec;

    /// Calculates the complement of the given element.
    fn evaluate2(&self, logic: &mut Solver, elems: &[LitSlice<'_>]) -> LitVec;
}

fn _test(_hihi: &dyn Function) {}
