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
    BitSlice, Boolean, BooleanLattice, BooleanLogic, BoundedOrder, Countable, DirectedGraph,
    Domain, Functions, Lattice, MeetSemilattice, PartIter, PartialOrder, Slice, Vector,
};

/// A domain containing relations of a fixed arity.
pub type Relations<DOM> = Functions<DOM, Boolean>;

/// A domain of relations, which are functions to the BOOLEAN domain.

impl<DOM> Relations<DOM>
where
    DOM: Countable,
{
    /// Creates a new function domain from the given domain to
    /// the target codomain.
    pub fn new_relations(dom: DOM, arity: usize) -> Self {
        Functions::new(dom, Boolean(), arity)
    }

    /// Returns the relation that is true if and only if all arguments are
    /// the same. This method panics if the arity is zero.
    pub fn get_diagonal<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(self.arity() >= 1);

        let num_bits = self.num_bits();
        let size = self.domain().size();

        let stride = if size >= 2 {
            (num_bits - 1) / (size - 1)
        } else {
            1
        };

        let mut result: LOGIC::Vector = Vector::with_values(num_bits, logic.bool_zero());

        let mut index = 0;
        for _ in 0..size {
            result.set(index, logic.bool_unit());
            index += stride;
        }
        debug_assert_eq!(index - stride, num_bits - 1);

        result
    }

    /// Checks if the given relation is the diagonal relation (only the
    /// elements in the diagonal are set).
    pub fn is_diagonal<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let diag = self.get_diagonal(logic);
        self.equals(logic, elem, diag.slice())
    }

    /// Returns a unary relation containing only the given tuple. This
    /// method panics if the number of elements in the tuple does not
    /// match the arity of the domain.
    pub fn get_singleton<LOGIC>(&self, logic: &LOGIC, elem: &[BitSlice<'_>]) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(self.arity() == elem.len());

        let size = self.domain().size();

        let mut index = 0;
        for value in elem.iter().rev() {
            index *= size;
            index += self.domain().get_index(*value);
        }

        let mut result: LOGIC::Vector = Vector::with_values(self.num_bits(), logic.bool_zero());
        result.set(index, logic.bool_unit());
        result
    }

    /// Checks if the given element is a singleton.
    pub fn is_singleton<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert!(self.arity() == 1);
        logic.bool_fold_one(elem.copy_iter())
    }

    /// Returns the domain size slices of elements.
    fn fold_iter<'a, ELEM>(&self, elem: ELEM) -> PartIter<'a, ELEM>
    where
        ELEM: Slice<'a>,
    {
        assert!(self.arity() >= 1);
        assert_eq!(elem.len(), self.num_bits());
        PartIter::new(elem, self.domain().size())
    }

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using the logical and operation.
    pub fn fold_all<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(self.arity() >= 1);
        let dom = self.change_arity(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_all(part.copy_iter()));
        }
        result
    }

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using the logical or operation.
    pub fn fold_any<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(self.arity() >= 1);
        let dom = self.change_arity(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_any(part.copy_iter()));
        }
        result
    }

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using the operation that is true when exavtly one
    /// of the elements is true.
    pub fn fold_one<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(self.arity() >= 1);
        let dom = self.change_arity(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_one(part.copy_iter()));
        }
        result
    }

    /// Returns the projection of the given relation to the given coordinates.
    /// The set of coordinates mut be distinct. A tuple is in the new
    /// relation there are elements for the missing coordinates such that
    /// the extended tuple is in the old relation.
    pub fn project<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem: LOGIC::Slice<'_>,
        coords: &[usize],
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(coords.len() <= self.arity());
        let start = self.arity() - coords.len();

        let mut pos = start;
        let mut map = vec![self.arity(); self.arity()];
        for &i in coords {
            assert!(map[i] == self.arity());
            map[i] = pos;
            pos += 1;
        }

        pos = 0;
        for m in map.iter_mut() {
            if *m == self.arity() {
                *m = pos;
                pos += 1;
            }
        }
        debug_assert_eq!(pos, start);

        let mut elem = self.polymer(elem, self.arity(), &map);
        for _ in 0..start {
            elem = self.fold_any(logic, elem.slice());
        }

        elem
    }
}

impl<DOM> DirectedGraph for Relations<DOM>
where
    DOM: Countable,
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
        self.as_power().is_edge(logic, elem0, elem1)
    }
}

impl<DOM> PartialOrder for Relations<DOM> where DOM: Countable {}

impl<DOM> BoundedOrder for Relations<DOM>
where
    DOM: Countable,
{
    fn get_top<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.as_power().get_top(logic)
    }

    fn is_top<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.as_power().is_top(logic, elem)
    }

    fn get_bottom<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.as_power().get_bottom(logic)
    }

    fn is_bottom<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.as_power().is_bottom(logic, elem)
    }
}

impl<DOM> MeetSemilattice for Relations<DOM>
where
    DOM: Countable,
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
        self.as_power().meet(logic, elem0, elem1)
    }
}

impl<DOM> Lattice for Relations<DOM>
where
    DOM: Countable,
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
        self.as_power().join(logic, elem0, elem1)
    }
}

impl<DOM> BooleanLattice for Relations<DOM>
where
    DOM: Countable,
{
    fn complement<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.as_power().complement(logic, elem)
    }

    fn implies<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.as_power().implies(logic, elem0, elem1)
    }
}
