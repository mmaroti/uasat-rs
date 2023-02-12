/*
* Copyright (C) 2022-2023, Miklos Maroti
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
    BooleanLogic, BoundedOrder, Countable, Domain, GenSlice, GenVec, MeetSemilattice, PartialOrder,
    SliceFor, VecFor,
};

/// The product of two domains.
#[derive(Debug, Clone)]
pub struct Product2<DOM0, DOM1>
where
    DOM0: Domain,
    DOM1: Domain,
{
    dom0: DOM0,
    dom1: DOM1,
}

impl<DOM0, DOM1> Product2<DOM0, DOM1>
where
    DOM0: Domain,
    DOM1: Domain,
{
    /// Creates the product of two domains.
    pub fn new(dom0: DOM0, dom1: DOM1) -> Self {
        Self { dom0, dom1 }
    }

    /// Returns the first domain of this product.
    pub fn dom0(&self) -> &DOM0 {
        &self.dom0
    }

    /// Returns the second domain of this product.
    pub fn dom1(&self) -> &DOM1 {
        &self.dom1
    }

    /// Returns the first part of an element.
    pub fn part0<SLICE, ELEM>(&self, elem: SLICE) -> SLICE
    where
        SLICE: GenSlice<ELEM>,
        ELEM: Copy,
    {
        debug_assert!(elem.len() == self.num_bits());
        elem.head(self.dom0.num_bits())
    }

    /// Returns the second part of an element.
    pub fn part1<SLICE, ELEM>(&self, elem: SLICE) -> SLICE
    where
        SLICE: GenSlice<ELEM>,
        ELEM: Copy,
    {
        debug_assert!(elem.len() == self.num_bits());
        elem.tail(self.dom0.num_bits())
    }
}

impl<DOM0, DOM1> Domain for Product2<DOM0, DOM1>
where
    DOM0: Domain,
    DOM1: Domain,
{
    fn num_bits(&self) -> usize {
        self.dom0.num_bits() + self.dom1.num_bits()
    }

    fn contains<ALG>(&self, alg: &mut ALG, elem: SliceFor<'_, ALG::Elem>) -> ALG::Elem
    where
        ALG: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let valid0 = self.dom0.contains(alg, elem.head(bits0));
        let valid1 = self.dom1.contains(alg, elem.tail(bits0));
        alg.bool_and(valid0, valid1)
    }

    fn display_elem<'a>(
        &self,
        f: &mut std::fmt::Formatter<'a>,
        elem: SliceFor<'_, bool>,
    ) -> std::fmt::Result {
        let bits0 = self.dom0.num_bits();
        write!(f, "(")?;
        self.dom0.display_elem(f, elem.head(bits0))?;
        write!(f, ",")?;
        self.dom1.display_elem(f, elem.tail(bits0))?;
        write!(f, ")")
    }
}

impl<DOM0, DOM1> Countable for Product2<DOM0, DOM1>
where
    DOM0: Countable,
    DOM1: Countable,
{
    fn size(&self) -> usize {
        self.dom0.size() * self.dom1.size()
    }

    fn elem(&self, index: usize) -> VecFor<bool> {
        let size0 = self.dom0.size();
        let mut result: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        result.extend(self.dom0.elem(index % size0));
        result.extend(self.dom1.elem(index / size0));
        debug_assert!(result.len() == self.num_bits());
        result
    }

    fn index(&self, elem: SliceFor<'_, bool>) -> usize {
        debug_assert!(elem.len() == self.num_bits());
        let bits0 = self.dom0.num_bits();
        let part0 = self.dom0.index(elem.head(bits0));

        let size0 = self.dom0.size();
        part0 + size0 * self.dom1.index(elem.tail(bits0))
    }
}

impl<DOM0, DOM1> PartialOrder for Product2<DOM0, DOM1>
where
    DOM0: PartialOrder,
    DOM1: PartialOrder,
{
    fn leq<ALG>(
        &self,
        alg: &mut ALG,
        elem0: SliceFor<'_, ALG::Elem>,
        elem1: SliceFor<'_, ALG::Elem>,
    ) -> ALG::Elem
    where
        ALG: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let test0 = self.dom0.leq(alg, elem0.head(bits0), elem1.head(bits0));
        let test1 = self.dom1.leq(alg, elem0.tail(bits0), elem1.tail(bits0));
        alg.bool_and(test0, test1)
    }
}

impl<DOM0, DOM1> BoundedOrder for Product2<DOM0, DOM1>
where
    DOM0: BoundedOrder,
    DOM1: BoundedOrder,
{
    fn top(&self) -> VecFor<bool> {
        let mut elem: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.top());
        elem.append(&mut self.dom1.top());
        elem
    }

    fn bottom(&self) -> VecFor<bool> {
        let mut elem: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.bottom());
        elem.append(&mut self.dom1.bottom());
        elem
    }
}

impl<DOM0, DOM1> MeetSemilattice for Product2<DOM0, DOM1>
where
    DOM0: MeetSemilattice,
    DOM1: MeetSemilattice,
{
    fn meet<ALG>(
        &self,
        alg: &mut ALG,
        elem0: SliceFor<'_, ALG::Elem>,
        elem1: SliceFor<'_, ALG::Elem>,
    ) -> VecFor<ALG::Elem>
    where
        ALG: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let mut elem: VecFor<ALG::Elem> = GenVec::with_capacity(self.num_bits());
        elem.extend(self.dom0.meet(alg, elem0.head(bits0), elem1.head(bits0)));
        elem.extend(self.dom0.meet(alg, elem0.tail(bits0), elem1.tail(bits0)));
        elem
    }
}
