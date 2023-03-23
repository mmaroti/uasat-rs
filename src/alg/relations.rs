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
    BitSlice, BitVec, Boolean, BooleanLattice, BooleanLogic, BoundedOrder, Countable, Domain,
    MeetSemilattice, Power, RankedDomain, Slice, SmallSet, Vector,
};

impl<DOM> Power<Boolean, Power<DOM, SmallSet>>
where
    DOM: Countable,
{
    /// Returns the relation that is true if and only if all arguments are
    /// the same. This method panics if the arity is zero.
    pub fn get_diagonal(&self) -> BitVec {
        assert!(self.arity() >= 1);

        let num_bits = self.num_bits();
        let size = self.exponent().size();

        let stride = if size >= 2 {
            (num_bits - 1) / (size - 1)
        } else {
            1
        };

        let mut result: BitVec = Vector::with_values(num_bits, false);

        let mut index = 0;
        for _ in 0..size {
            result.set(index, true);
            index += stride;
        }
        debug_assert_eq!(index - stride, num_bits - 1);

        result
    }

    /// Checks if the given element is the diagonal relation.
    pub fn is_diagonal<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let diag = self.lift(logic, self.get_diagonal().slice());
        self.equals(logic, elem, diag.slice())
    }

    /// Checks if the given binary relation is reflexive.
    pub fn is_reflexive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(self.arity(), 2);
        let dom1 = self.other(1);
        let elem = self.polymer(elem, 1, &[0, 0]);
        dom1.is_top(logic, elem.slice())
    }

    /// Returns a unary relation containing the given element only. This
    /// method panics if the arity of this ranked domain is not one.
    pub fn get_singleton(&self, elem: BitSlice<'_>) -> BitVec {
        assert!(self.arity() == 1);

        let num_bits = self.num_bits();
        let mut result: BitVec = Vector::with_values(num_bits, false);

        let index = self.exponent().base().index(elem);
        result.set(index, true);

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

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using logical and.
    pub fn fold_all<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let dom = self.other(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_all(part.copy_iter()));
        }
        result
    }

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using logical or.
    pub fn fold_any<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let dom = self.other(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_any(part.copy_iter()));
        }
        result
    }

    /// Returns true if the given binary relation is symmetric.
    pub fn is_symmetric<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let conv = self.polymer(elem, 2, &[1, 0]);
        let elem = self.implies(logic, elem, conv.slice());
        self.is_top(logic, elem.slice())
    }

    /// Returns the composition of the given binary relations.
    pub fn compose<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(self.arity(), 2);
        let dom3 = self.other(3);
        let elem0: LOGIC::Vector = self.polymer(elem0, 3, &[1, 0]);
        let elem1: LOGIC::Vector = self.polymer(elem1, 3, &[0, 2]);
        let elem2 = dom3.meet(logic, elem0.slice(), elem1.slice());
        dom3.fold_any(logic, elem2.slice())
    }

    /// Returns true if the given binary relation is transitive.
    pub fn is_transitive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let comp = self.compose(logic, elem, elem);
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
}
