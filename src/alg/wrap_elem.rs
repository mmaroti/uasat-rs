/*
* Copyright (C) 2022, Miklos Maroti
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

use super::{
    BipartiteGraph, BitSlice, BitVec, Boolean, BooleanLogic, Countable, Domain, Power, Product2,
    Slice,
};

#[derive(Debug, Clone)]

pub struct WrapElem<DOM>
where
    DOM: Domain,
{
    domain: DOM,
    _elem: BitVec,
}

impl<DOM> WrapElem<DOM>
where
    DOM: Domain,
{
    /// Creates a new domain that wraps the given element.
    pub fn new(domain: DOM, elem: BitSlice<'_>) -> Self {
        Self {
            domain,
            _elem: elem.copy_iter().collect(),
        }
    }
}

impl<DOM0, DOM1> BipartiteGraph<DOM0, DOM1> for WrapElem<Power<Boolean, Product2<DOM0, DOM1>>>
where
    DOM0: Countable,
    DOM1: Countable,
{
    fn domain(&self) -> &DOM0 {
        self.domain.exponent().dom0()
    }

    fn codomain(&self) -> &DOM1 {
        self.domain.exponent().dom1()
    }

    fn is_edge<LOGIC>(
        &self,
        _logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        _elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        // TODO: implement this
        elem0.get(0)
    }
}
