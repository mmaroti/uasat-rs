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
    BitSlice, BitVec, BooleanLattice, BooleanLogic, BoundedOrder, Countable, DirectedGraph, Domain,
    Base, Lattice, Logic, MeetSemilattice, PartialOrder, Slice, Vector,
};

/// The product of two domains.
#[derive(Debug, Clone, PartialEq)]
pub struct Product2<DOM0, DOM1> {
    dom0: DOM0,
    dom1: DOM1,
}

impl<DOM0, DOM1> Product2<DOM0, DOM1>
where
    DOM0: Domain<Logic>,
    DOM1: Domain<Logic>,
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
    pub fn part0<'a, ELEM>(&self, elem: ELEM) -> ELEM
    where
        ELEM: Slice<'a>,
    {
        debug_assert!(elem.len() == self.num_bits());
        elem.head(self.dom0.num_bits())
    }

    /// Returns the second part of an element.
    pub fn part1<'a, ELEM>(&self, elem: ELEM) -> ELEM
    where
        ELEM: Slice<'a>,
    {
        debug_assert!(elem.len() == self.num_bits());
        elem.tail(self.dom0.num_bits())
    }
}

impl<DOM0, DOM1> Base for Product2<DOM0, DOM1>
where
    DOM0: Base,
    DOM1: Base,
{
    fn num_bits(&self) -> usize {
        self.dom0.num_bits() + self.dom1.num_bits()
    }

    fn display_elem(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        elem: BitSlice<'_>,
    ) -> std::fmt::Result {
        let bits0 = self.dom0.num_bits();
        write!(f, "(")?;
        self.dom0.display_elem(f, elem.head(bits0))?;
        write!(f, ",")?;
        self.dom1.display_elem(f, elem.tail(bits0))?;
        write!(f, ")")
    }
}

impl<DOM0, DOM1, LOGIC> Domain<LOGIC> for Product2<DOM0, DOM1>
where
    DOM0: Domain<LOGIC>,
    DOM1: Domain<LOGIC>,
    LOGIC: BooleanLogic,
{
    fn contains(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        let bits0 = self.dom0.num_bits();
        let valid0 = self.dom0.contains(logic, elem.head(bits0));
        let valid1 = self.dom1.contains(logic, elem.tail(bits0));
        logic.bool_and(valid0, valid1)
    }

    fn equals(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem {
        let bits0 = self.dom0.num_bits();
        let test0 = self
            .dom0
            .equals(logic, elem0.head(bits0), elem1.head(bits0));
        let test1 = self
            .dom1
            .equals(logic, elem0.tail(bits0), elem1.tail(bits0));
        logic.bool_and(test0, test1)
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
        let mut result: BitVec = Vector::with_capacity(self.num_bits());
        result.extend(self.dom0.elem(index % size0));
        result.extend(self.dom1.elem(index / size0));
        debug_assert!(result.len() == self.num_bits());
        result
    }

    fn index(&self, elem: BitSlice<'_>) -> usize {
        debug_assert!(elem.len() == self.num_bits());
        let bits0 = self.dom0.num_bits();
        let part0 = self.dom0.index(elem.head(bits0));

        let size0 = self.dom0.size();
        part0 + size0 * self.dom1.index(elem.tail(bits0))
    }
}

impl<DOM0, DOM1, LOGIC> DirectedGraph<LOGIC> for Product2<DOM0, DOM1>
where
    DOM0: DirectedGraph<LOGIC>,
    DOM1: DirectedGraph<LOGIC>,
    LOGIC: BooleanLogic,
{
    fn is_edge(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem {
        let bits0 = self.dom0.num_bits();
        let test0 = self
            .dom0
            .is_edge(logic, elem0.head(bits0), elem1.head(bits0));
        let test1 = self
            .dom1
            .is_edge(logic, elem0.tail(bits0), elem1.tail(bits0));
        logic.bool_and(test0, test1)
    }
}

impl<DOM0, DOM1, LOGIC: BooleanLogic> PartialOrder<LOGIC> for Product2<DOM0, DOM1>
where
    DOM0: PartialOrder<LOGIC>,
    DOM1: PartialOrder<LOGIC>,
    LOGIC: BooleanLogic,
{
}

impl<DOM0, DOM1, LOGIC> BoundedOrder<LOGIC> for Product2<DOM0, DOM1>
where
    DOM0: BoundedOrder<LOGIC>,
    DOM1: BoundedOrder<LOGIC>,
    LOGIC: BooleanLogic,
{
    fn top(&self, logic: &LOGIC) -> LOGIC::Vector {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.top(logic));
        elem.append(&mut self.dom1.top(logic));
        elem
    }

    fn is_top(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        let bits0 = self.dom0.num_bits();
        let test0 = self.dom0.is_top(logic, elem.head(bits0));
        let test1 = self.dom1.is_top(logic, elem.tail(bits0));
        logic.bool_and(test0, test1)
    }

    fn bottom(&self, logic: &LOGIC) -> LOGIC::Vector {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.bottom(logic));
        elem.append(&mut self.dom1.bottom(logic));
        elem
    }

    fn is_bottom(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem {
        let bits0 = self.dom0.num_bits();
        let test0 = self.dom0.is_bottom(logic, elem.head(bits0));
        let test1 = self.dom1.is_bottom(logic, elem.tail(bits0));
        logic.bool_and(test0, test1)
    }
}

impl<DOM0, DOM1, LOGIC> MeetSemilattice<LOGIC> for Product2<DOM0, DOM1>
where
    DOM0: MeetSemilattice<LOGIC>,
    DOM1: MeetSemilattice<LOGIC>,
    LOGIC: BooleanLogic,
{
    fn meet(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector {
        let bits0 = self.dom0.num_bits();
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.extend(self.dom0.meet(logic, elem0.head(bits0), elem1.head(bits0)));
        elem.extend(self.dom1.meet(logic, elem0.tail(bits0), elem1.tail(bits0)));
        elem
    }
}

impl<DOM0, DOM1, LOGIC> Lattice<LOGIC> for Product2<DOM0, DOM1>
where
    DOM0: Lattice<LOGIC>,
    DOM1: Lattice<LOGIC>,
    LOGIC: BooleanLogic,
{
    fn join(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector {
        let bits0 = self.dom0.num_bits();
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.extend(self.dom0.join(logic, elem0.head(bits0), elem1.head(bits0)));
        elem.extend(self.dom1.join(logic, elem0.tail(bits0), elem1.tail(bits0)));
        elem
    }
}

impl<DOM0, DOM1, LOGIC> BooleanLattice<LOGIC> for Product2<DOM0, DOM1>
where
    DOM0: BooleanLattice<LOGIC>,
    DOM1: BooleanLattice<LOGIC>,
    LOGIC: BooleanLogic,
{
    fn complement(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector {
        let bits0 = self.dom0.num_bits();
        let mut result: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        result.extend(self.dom0.complement(logic, elem.head(bits0)));
        result
    }
}
