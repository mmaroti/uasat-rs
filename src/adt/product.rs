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
    BitVec, BooleanLattice, BooleanLogic, BoundedOrder, Countable, Domain, GenSlice, GenVec,
    Lattice, MeetSemilattice, PartialOrder,
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
    pub fn part0<ELEM>(&self, elem: ELEM) -> ELEM
    where
        ELEM: GenSlice,
    {
        debug_assert!(elem.len() == self.num_bits());
        elem.head(self.dom0.num_bits())
    }

    /// Returns the second part of an element.
    pub fn part1<ELEM>(&self, elem: ELEM) -> ELEM
    where
        ELEM: GenSlice,
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

    fn contains<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let bits0 = self.dom0.num_bits();
        let valid0 = self.dom0.contains(logic, elem.head(bits0));
        let valid1 = self.dom1.contains(logic, elem.tail(bits0));
        logic.bool_and(valid0, valid1)
    }

    fn equals<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let bits0 = self.dom0.num_bits();
        let test0 = self
            .dom0
            .equals(logic, elem0.head(bits0), elem1.head(bits0));
        let test1 = self
            .dom1
            .equals(logic, elem0.tail(bits0), elem1.tail(bits0));
        logic.bool_and(test0, test1)
    }

    fn display_elem<ELEM>(&self, f: &mut std::fmt::Formatter<'_>, elem: ELEM) -> std::fmt::Result
    where
        ELEM: GenSlice<Item = bool>,
    {
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

    fn elem(&self, index: usize) -> BitVec {
        let size0 = self.dom0.size();
        let mut result: BitVec = GenVec::with_capacity(self.num_bits());
        result.extend(self.dom0.elem(index % size0));
        result.extend(self.dom1.elem(index / size0));
        debug_assert!(result.len() == self.num_bits());
        result
    }

    fn index<ELEM>(&self, elem: ELEM) -> usize
    where
        ELEM: GenSlice<Item = bool>,
    {
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
    fn leq<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let bits0 = self.dom0.num_bits();
        let test0 = self.dom0.leq(logic, elem0.head(bits0), elem1.head(bits0));
        let test1 = self.dom1.leq(logic, elem0.tail(bits0), elem1.tail(bits0));
        logic.bool_and(test0, test1)
    }
}

impl<DOM0, DOM1> BoundedOrder for Product2<DOM0, DOM1>
where
    DOM0: BoundedOrder,
    DOM1: BoundedOrder,
{
    fn top(&self) -> BitVec {
        let mut elem: BitVec = GenVec::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.top());
        elem.append(&mut self.dom1.top());
        elem
    }

    fn bottom(&self) -> BitVec {
        let mut elem: BitVec = GenVec::with_capacity(self.num_bits());
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
    fn meet<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let bits0 = self.dom0.num_bits();
        let mut elem: ELEM::Vec = GenVec::with_capacity(self.num_bits());
        elem.extend(self.dom0.meet(logic, elem0.head(bits0), elem1.head(bits0)));
        elem.extend(self.dom1.meet(logic, elem0.tail(bits0), elem1.tail(bits0)));
        elem
    }
}

impl<DOM0, DOM1> Lattice for Product2<DOM0, DOM1>
where
    DOM0: Lattice,
    DOM1: Lattice,
{
    fn join<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem0: ELEM, elem1: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let bits0 = self.dom0.num_bits();
        let mut elem: ELEM::Vec = GenVec::with_capacity(self.num_bits());
        elem.extend(self.dom0.join(logic, elem0.head(bits0), elem1.head(bits0)));
        elem.extend(self.dom1.join(logic, elem0.tail(bits0), elem1.tail(bits0)));
        elem
    }
}

impl<DOM0, DOM1> BooleanLattice for Product2<DOM0, DOM1>
where
    DOM0: BooleanLattice,
    DOM1: BooleanLattice,
{
    fn complement<LOGIC, ELEM>(&self, logic: &mut LOGIC, elem: ELEM) -> ELEM::Vec
    where
        LOGIC: BooleanLogic,
        ELEM: GenSlice<Item = LOGIC::Elem>,
    {
        let bits0 = self.dom0.num_bits();
        let mut result: ELEM::Vec = GenVec::with_capacity(self.num_bits());
        result.extend(self.dom0.complement(logic, elem.head(bits0)));
        result
    }
}
