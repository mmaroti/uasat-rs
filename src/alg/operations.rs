/*
* Copyright (C) 2023, Miklos Maroti
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
    Boolean, BooleanLogic, BoundedOrder, Countable, Domain, Functions, Power, PowerDomain, Slice,
    SmallSet, Vector,
};

pub trait Operations: Functions
where
    Self::Exp: PowerDomain<Base = Self::Base>,
    Self::Base: Countable,
{
    /// Returns the graph of the given operation, which is a relation
    /// of arity one larger than this operation.
    fn graph<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(self.base(), self.domain());
        let domain = self.base();

        let size = domain.size();
        let mut power = size;
        for _ in 0..self.arity() {
            power *= size;
        }

        let mut result: LOGIC::Vector = Vector::with_capacity(power);
        for part in self.part_iter(elem) {
            for index in 0..size {
                let value = domain.get_elem(logic, index);
                result.push(domain.equals(logic, part, value.slice()));
            }
        }

        debug_assert_eq!(result.len(), power);
        result
    }

    /// Returns a unary relation containing the range of the given operation.
    fn range<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(self.base(), self.domain());

        let mut result: LOGIC::Vector = Vector::with_values(self.base().size(), logic.bool_zero());

        for part in self.part_iter(elem) {
            let part = self.base().onehot(logic, part);
            assert_eq!(part.len(), result.len());
            for (idx, val) in part.copy_iter().enumerate() {
                result.set(idx, logic.bool_or(result.get(idx), val));
            }
        }

        result
    }

    /// Returns true if the given element is a surjective operation.
    fn is_surjective<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let range = self.range(logic, elem);
        let dom = Power::new(Boolean(), self.domain().clone());
        dom.is_top(logic, range.slice())
    }
}

impl<DOM> Operations for Power<DOM, Power<DOM, SmallSet>> where DOM: Countable {}

pub trait UnaryOperations: Operations
where
    Self::Exp: PowerDomain<Base = Self::Base>,
    Self::Base: Countable,
{
    /// Returns true if the given element is a permutation (surjective).
    fn is_permutation<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(self.arity(), 1);
        self.is_surjective(logic, elem)
    }
}

impl<DOM> UnaryOperations for Power<DOM, Power<DOM, SmallSet>> where DOM: Countable {}

#[cfg(test)]
mod tests {
    use super::super::{BitVec, Domain, Logic, Solver, Vector, BOOLEAN};
    use super::*;

    #[test]
    fn graph() {
        let dom = SmallSet::new(3);
        let ops = Power::new(dom.clone(), Power::new(dom.clone(), SmallSet::new(1)));
        let rel = Power::new(BOOLEAN, Power::new(dom.clone(), SmallSet::new(2)));

        let mut logic = Logic();
        let elem1: BitVec = vec![true, false, false, false, false, true, false, true, false]
            .into_iter()
            .collect();
        assert!(ops.contains(&mut logic, elem1.slice()));

        let graph1 = ops.graph(&mut logic, elem1.slice());
        assert!(rel.contains(&mut logic, graph1.slice()));
        assert_eq!(elem1, graph1);

        let mut solver = Solver::new("");
        let elem2 = ops.add_variable(&mut solver);
        let graph2 = ops.graph(&mut solver, elem2.slice());
        assert_eq!(elem2, graph2);

        let dom = BOOLEAN;
        let ops = Power::new(dom.clone(), Power::new(dom.clone(), SmallSet::new(1)));

        let elem3 = ops.add_variable(&mut solver);
        let graph3 = ops.graph(&mut solver, elem3.slice());
        assert_eq!(elem3.len(), 2);
        assert_eq!(graph3.len(), 4);
        assert_eq!(graph3.get(0), solver.bool_not(elem3.get(0)));
        assert_eq!(graph3.get(1), elem3.get(0));
        assert_eq!(graph3.get(2), solver.bool_not(elem3.get(1)));
        assert_eq!(graph3.get(3), elem3.get(1));
    }
}
