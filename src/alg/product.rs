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
    BitSlice, BooleanLattice, BooleanLogic, BoundedOrder, Indexable, DirectedGraph, Domain,
    Lattice, MeetSemilattice, Monoid, PartialOrder, Semigroup, Slice, Vector,
};

/// The product of two domains.
#[derive(Debug, Clone, PartialEq)]
pub struct Product2<DOM0, DOM1> {
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

    pub fn dom0(&self) -> &DOM0 {
        &self.dom0
    }

    pub fn dom1(&self) -> &DOM1 {
        &self.dom1
    }

    /// Returns the first part of an element.
    pub fn part0<'a, ELEM>(&self, elem: ELEM) -> ELEM
    where
        ELEM: Slice<'a>,
    {
        debug_assert_eq!(elem.len(), self.num_bits());
        elem.head(self.dom0().num_bits())
    }

    /// Returns the second part of an element.
    pub fn part1<'a, ELEM>(&self, elem: ELEM) -> ELEM
    where
        ELEM: Slice<'a>,
    {
        let result = elem.tail(self.dom0().num_bits());
        debug_assert_eq!(result.len(), self.dom1().num_bits());
        result
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

    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let valid0 = self.dom0.contains(logic, elem.head(bits0));
        let valid1 = self.dom1.contains(logic, elem.tail(bits0));
        logic.bool_and(valid0, valid1)
    }

    fn equals<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
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
}

impl<DOM0, DOM1> Indexable for Product2<DOM0, DOM1>
where
    DOM0: Indexable,
    DOM1: Indexable,
{
    fn size(&self) -> usize {
        self.dom0.size() * self.dom1.size()
    }

    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let size0 = self.dom0.size();
        let mut result: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        result.extend(self.dom0.get_elem(logic, index % size0));
        result.extend(self.dom1.get_elem(logic, index / size0));
        debug_assert!(result.len() == self.num_bits());
        result
    }

    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        debug_assert!(elem.len() == self.num_bits());
        let bits0 = self.dom0.num_bits();
        let part0 = self.dom0.get_index(elem.head(bits0));

        let size0 = self.dom0.size();
        part0 + size0 * self.dom1.get_index(elem.tail(bits0))
    }

    fn onehot<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let part0 = self.dom0.onehot(logic, elem.head(bits0));
        let part1 = self.dom1.onehot(logic, elem.tail(bits0));

        let mut result: LOGIC::Vector = Vector::with_capacity(self.size());
        for v1 in part1.copy_iter() {
            for v0 in part0.copy_iter() {
                result.push(logic.bool_and(v0, v1));
            }
        }

        debug_assert_eq!(result.len(), self.size());
        result
    }
}

impl<DOM0, DOM1> DirectedGraph for Product2<DOM0, DOM1>
where
    DOM0: DirectedGraph,
    DOM1: DirectedGraph,
{
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
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

impl<DOM0, DOM1> PartialOrder for Product2<DOM0, DOM1>
where
    DOM0: PartialOrder,
    DOM1: PartialOrder,
{
}

impl<DOM0, DOM1> BoundedOrder for Product2<DOM0, DOM1>
where
    DOM0: BoundedOrder,
    DOM1: BoundedOrder,
{
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.get_top(logic));
        elem.append(&mut self.dom1.get_top(logic));
        elem
    }

    fn is_top<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let test0 = self.dom0.is_top(logic, elem.head(bits0));
        let test1 = self.dom1.is_top(logic, elem.tail(bits0));
        logic.bool_and(test0, test1)
    }

    fn get_bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.get_bottom(logic));
        elem.append(&mut self.dom1.get_bottom(logic));
        elem
    }

    fn is_bottom<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let test0 = self.dom0.is_bottom(logic, elem.head(bits0));
        let test1 = self.dom1.is_bottom(logic, elem.tail(bits0));
        logic.bool_and(test0, test1)
    }
}

impl<DOM0, DOM1> MeetSemilattice for Product2<DOM0, DOM1>
where
    DOM0: MeetSemilattice,
    DOM1: MeetSemilattice,
{
    fn meet<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
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
    fn join<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
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
    fn complement<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let mut result: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        result.extend(self.dom0.complement(logic, elem.head(bits0)));
        result
    }
}

impl<DOM0, DOM1> Semigroup for Product2<DOM0, DOM1>
where
    DOM0: Semigroup,
    DOM1: Semigroup,
{
    fn product<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.extend(
            self.dom0
                .product(logic, elem0.head(bits0), elem1.head(bits0)),
        );
        elem.extend(
            self.dom1
                .product(logic, elem0.tail(bits0), elem1.tail(bits0)),
        );
        elem
    }
}

impl<DOM0, DOM1> Monoid for Product2<DOM0, DOM1>
where
    DOM0: Monoid,
    DOM1: Monoid,
{
    fn get_identity<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let mut elem: LOGIC::Vector = Vector::with_capacity(self.num_bits());
        elem.append(&mut self.dom0.get_identity(logic));
        elem.append(&mut self.dom1.get_identity(logic));
        elem
    }

    fn is_identity<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let bits0 = self.dom0.num_bits();
        let test0 = self.dom0.is_identity(logic, elem.head(bits0));
        let test1 = self.dom1.is_identity(logic, elem.tail(bits0));
        logic.bool_and(test0, test1)
    }
}
