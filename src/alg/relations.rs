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
    Domain, Lattice, MeetSemilattice, PartIter, PartialOrder, Power, Slice, SmallSet, Vector,
};

/// A domain containing relations of a fixed arity.
#[derive(Debug, Clone, PartialEq)]
pub struct Relations<DOM>(Power<Boolean, Power<DOM, SmallSet>>)
where
    DOM: Countable;

/// A domain of relations, which are functions to the BOOLEAN domain.

impl<DOM> Relations<DOM>
where
    DOM: Countable,
{
    /// Creates a new function domain from the given domain to
    /// the target codomain.
    pub fn new(dom: DOM, arity: usize) -> Self {
        Relations(Power::new(Boolean(), Power::new(dom, SmallSet::new(arity))))
    }

    /// Returns the arity (rank) of all relations in the domain.
    pub fn arity(&self) -> usize {
        self.0.exponent().exponent().size()
    }

    /// Returns the domain of the relations.
    pub fn domain(&self) -> &DOM {
        self.0.exponent().base()
    }

    /// Returns another domain of relations with same domain but with the
    /// new given arity.
    pub fn change_arity(&self, arity: usize) -> Self {
        Relations::new(self.domain().clone(), arity)
    }

    /// Creates a new relation of the given arity from an old relation with
    /// permuted, identified and/or new dummy coordinates. The mapping is a
    /// vector of length of the arity of the original function with entries
    /// identifying the matching coordinates in the new function.
    pub fn polymer<'a, SLICE>(&self, elem: SLICE, arity: usize, mapping: &[usize]) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(mapping.len(), self.arity());

        let mut strides: Vec<(usize, usize, usize)> = vec![(0, 0, 0); arity];
        let size = self.domain().size();
        let mut power: usize = 1;
        for &i in mapping {
            assert!(i < arity);
            strides[i].0 += power;
            power *= size;
        }

        power = 1;
        for s in strides.iter_mut() {
            s.2 = size * s.0;
            power *= size;
        }

        let mut result: SLICE::Vector = Vector::with_capacity(power);
        let mut index = 0;
        'outer: loop {
            result.push(elem.get(index));

            for stride in strides.iter_mut() {
                index += stride.0;
                stride.1 += 1;
                if stride.1 >= size {
                    stride.1 = 0;
                    index -= stride.2;
                } else {
                    continue 'outer;
                }
            }

            break;
        }

        debug_assert_eq!(result.len(), power);
        result
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

impl<DOM> Domain for Relations<DOM>
where
    DOM: Countable,
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

impl<DOM> Countable for Relations<DOM>
where
    DOM: Countable,
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
    fn get_index(&self, elem: crate::genvec::BitSlice<'_>) -> usize {
        self.0.get_index(elem)
    }

    #[inline]
    fn onehot<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.onehot(logic, elem)
    }
}

impl<DOM> DirectedGraph for Relations<DOM>
where
    DOM: Countable,
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

impl<DOM> PartialOrder for Relations<DOM> where DOM: Countable {}

impl<DOM> BoundedOrder for Relations<DOM>
where
    DOM: Countable,
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

impl<DOM> MeetSemilattice for Relations<DOM>
where
    DOM: Countable,
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

impl<DOM> Lattice for Relations<DOM>
where
    DOM: Countable,
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

impl<DOM> BooleanLattice for Relations<DOM>
where
    DOM: Countable,
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
