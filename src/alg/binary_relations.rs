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
    BitSlice, BooleanLattice, BooleanLogic, BoundedOrder, DirectedGraph, Domain, Indexable,
    Lattice, MeetSemilattice, Monoid, PartialOrder, Relations, Semigroup, Slice, Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryRelations<DOM>(Relations<DOM>)
where
    DOM: Indexable;

impl<DOM> BinaryRelations<DOM>
where
    DOM: Indexable,
{
    /// Creates domain of binary relations over the given domain.
    #[inline]
    pub fn new(dom: DOM) -> Self {
        Self(Relations::new(dom, 2))
    }

    /// Returns the underlying domain of this class of relations.
    #[inline]
    pub fn domain(&self) -> &DOM {
        self.0.domain()
    }

    /// Reverses the coordinates of the given binary relation.
    #[inline]
    pub fn converse<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        self.0.polymer(elem, 2, &[1, 0])
    }

    /// Checks if the given relation is reflexive, all constant tuples are members.
    pub fn is_reflexive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_reflexive(logic, elem)
    }

    /// Returns true if the given binary relation is symmetric under the rotation of
    /// coordinates.
    pub fn is_symmetric<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let conv = self.converse(elem);
        let elem = self.implies(logic, elem, conv.slice());
        self.is_top(logic, elem.slice())
    }

    /// Checks if the given binary relation is antisymmetric.
    pub fn is_antisymmetric<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let conv = self.converse(elem);
        let elem = self.meet(logic, elem, conv.slice());
        let diag = self.get_identity(logic);
        self.is_edge(logic, elem.slice(), diag.slice())
    }

    /// Returns true if the given binary relation is transitive.
    pub fn is_transitive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let comp = Semigroup::product(self, logic, elem, elem);
        let elem = self.implies(logic, comp.slice(), elem);
        self.is_top(logic, elem.slice())
    }

    /// Returns true if the given binary relation is an equivalence relation.
    pub fn is_equivalence<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test0 = self.is_reflexive(logic, elem);
        let test1 = self.is_symmetric(logic, elem);
        let test2 = self.is_transitive(logic, elem);
        let test3 = logic.bool_and(test0, test1);
        logic.bool_and(test2, test3)
    }

    /// Returns true if the given binary relation is a partial order relation.
    pub fn is_partial_order<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test0 = self.is_reflexive(logic, elem);
        let test1 = self.is_antisymmetric(logic, elem);
        let test2 = self.is_transitive(logic, elem);
        let test3 = logic.bool_and(test0, test1);
        logic.bool_and(test2, test3)
    }

    /// Returns true if this relation is the graph of an operation, that is the
    /// first coordinate is completely determined by the other coordinates.
    pub fn is_operation<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_operation(logic, elem)
    }

    /// Returns true if this relation is the graph of a partial operation.
    pub fn is_partial_operation<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_partial_operation(logic, elem)
    }

    /// Returns true if this binary relation encodes the graph of a permutation.
    pub fn is_permutation<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test1 = self.is_operation(logic, elem);
        let test2 = self.is_operation(logic, self.converse(elem).slice());
        logic.bool_and(test1, test2)
    }
}

impl<DOM> Domain for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn num_bits(&self) -> usize {
        self.0.num_bits()
    }

    #[inline]
    fn contains<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.contains(logic, elem)
    }

    #[inline]
    fn equals<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.equals(logic, elem0, elem1)
    }
}

impl<DOM> Indexable for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn size(&self) -> usize {
        self.0.size()
    }

    #[inline]
    fn get_elem<LOGIC>(&self, logic: &LOGIC, index: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.get_elem(logic, index)
    }

    #[inline]
    fn get_index(&self, elem: BitSlice<'_>) -> usize {
        self.0.get_index(elem)
    }
}

impl<DOM> DirectedGraph for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_edge(logic, elem0, elem1)
    }
}

impl<DOM> PartialOrder for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn is_less<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_less(logic, elem0, elem1)
    }

    #[inline]
    fn comparable<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.comparable(logic, elem0, elem1)
    }
}

impl<DOM> BoundedOrder for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.get_top(logic)
    }

    #[inline]
    fn is_top<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_top(logic, elem)
    }

    #[inline]
    fn get_bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.get_bottom(logic)
    }

    #[inline]
    fn is_bottom<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_bottom(logic, elem)
    }
}

impl<DOM> MeetSemilattice for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn meet<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.meet(logic, elem0, elem1)
    }
}

impl<DOM> Lattice for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn join<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.join(logic, elem0, elem1)
    }
}

impl<DOM> BooleanLattice for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn complement<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.complement(logic, elem)
    }

    #[inline]
    fn implies<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.implies(logic, elem0, elem1)
    }
}

impl<DOM> Semigroup for BinaryRelations<DOM>
where
    DOM: Indexable,
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
        let elem0: LOGIC::Vector = self.0.polymer(elem0, 3, &[1, 0]);
        let elem1: LOGIC::Vector = self.0.polymer(elem1, 3, &[0, 2]);

        let rels = Relations::new(self.domain().clone(), 3);
        let elem2 = rels.meet(logic, elem0.slice(), elem1.slice());
        rels.fold_any(logic, elem2.slice(), 1)
    }
}

impl<DOM> Monoid for BinaryRelations<DOM>
where
    DOM: Indexable,
{
    #[inline]
    fn get_identity<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.get_diagonal(logic)
    }

    #[inline]
    fn is_identity<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.0.is_diagonal(logic, elem)
    }
}
