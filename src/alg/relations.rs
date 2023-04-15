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
    BitSlice, Boolean, BooleanLattice, BooleanLogic, Countable, Domain, Functions, PartIter, Power,
    PowerDomain, Slice, SmallSet, Vector,
};

/// A domain of relations, which are functions to the BOOLEAN domain.
pub trait Relations: Functions<Base = Boolean> + BooleanLattice
where
    Self::Exp: PowerDomain,
    <Self::Exp as PowerDomain>::Base: Countable,
{
    /// Returns the relation that is true if and only if all arguments are
    /// the same. This method panics if the arity is zero.
    fn get_diagonal<LOGIC>(&self, logic: &LOGIC) -> LOGIC::Vector
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
    fn is_diagonal<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let diag = self.get_diagonal(logic);
        self.equals(logic, elem, diag.slice())
    }

    /// Returns a unary relation containing only the given tuple. This
    /// method panics if the number of elements in the tuple does not
    /// match the arity of the domain.
    fn get_singleton<LOGIC>(&self, logic: &LOGIC, elem: &[BitSlice<'_>]) -> LOGIC::Vector
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
    fn is_singleton<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
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
        PartIter::new(elem, self.base().num_bits() * self.domain().size())
    }

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using the logical and operation.
    fn fold_all<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let dom = self.change_arity(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_all(part.copy_iter()));
        }
        result
    }

    /// Returns a new relation of arity one less where the first coordinate is
    /// removed and folded using the logical or operation.
    fn fold_any<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let dom = self.change_arity(self.arity() - 1);
        let mut result: LOGIC::Vector = Vector::with_capacity(dom.num_bits());
        for part in self.fold_iter(elem) {
            result.push(logic.bool_fold_any(part.copy_iter()));
        }
        result
    }
}

impl<DOM> Relations for Power<Boolean, Power<DOM, SmallSet>> where DOM: Countable {}

/// A domain of binary relations.
pub trait BinaryRelations: Relations
where
    Self::Exp: PowerDomain,
    <Self::Exp as PowerDomain>::Base: Countable,
{
    /// Checks if the given binary relation is reflexive.
    fn is_reflexive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(self.arity(), 2);
        let elem = self.identify(elem);
        let dom1 = self.change_arity(1);
        dom1.is_top(logic, elem.slice())
    }

    /// Returns true if the given binary relation is symmetric.
    fn is_symmetric<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let conv = self.polymer(elem, 2, &[1, 0]);
        let elem = self.implies(logic, elem, conv.slice());
        self.is_top(logic, elem.slice())
    }

    /// Checks if the given binary relation is antisymmetric.
    fn is_antisymmetric<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let conv = self.polymer(elem, 2, &[1, 0]);
        let elem = self.meet(logic, elem, conv.slice());
        let diag = self.get_diagonal(logic);
        self.is_edge(logic, elem.slice(), diag.slice())
    }

    /// Returns the composition of the given binary relations.
    fn compose<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(self.arity(), 2);
        let dom3 = self.change_arity(3);
        let elem0: LOGIC::Vector = self.polymer(elem0, 3, &[1, 0]);
        let elem1: LOGIC::Vector = self.polymer(elem1, 3, &[0, 2]);
        let elem2 = dom3.meet(logic, elem0.slice(), elem1.slice());
        dom3.fold_any(logic, elem2.slice())
    }

    /// Returns true if the given binary relation is transitive.
    fn is_transitive<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let comp = self.compose(logic, elem, elem);
        let elem = self.implies(logic, comp.slice(), elem);
        self.is_top(logic, elem.slice())
    }

    /// Returns true if the given binary relation is an equivalence relation.
    fn is_equivalence<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
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
    fn is_partial_order<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let test0 = self.is_reflexive(logic, elem);
        let test1 = self.is_antisymmetric(logic, elem);
        let test2 = self.is_transitive(logic, elem);
        let test3 = logic.bool_and(test0, test1);
        logic.bool_and(test2, test3)
    }
}

impl<DOM> BinaryRelations for Power<Boolean, Power<DOM, SmallSet>> where DOM: Countable {}
