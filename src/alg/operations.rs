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
    BooleanLogic, Countable, Domain, Functions, Power, RankedDomain, Slice, SmallSet, Vector,
};

pub trait Operations<LOGIC>: Functions<LOGIC>
where
    LOGIC: BooleanLogic,
{
    /// Returns the graph of this operation, which is a relation
    /// of arity one larger than this operation.
    fn graph(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector;
}

impl<DOM, LOGIC> Operations<LOGIC> for Power<DOM, Power<DOM, SmallSet>>
where
    DOM: Countable,
    LOGIC: BooleanLogic,
{
    fn graph(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(self.base(), self.exponent().base());
        let domain = self.base();

        let size = domain.size();
        let mut power = size;
        for _ in 0..self.arity() {
            power *= size;
        }

        let mut result: LOGIC::Vector = Vector::with_capacity(power);
        for part in self.part_iter(elem) {
            for index in 0..size {
                let value = domain.lift(logic, domain.elem(index).slice());
                result.push(domain.equals(logic, part, value.slice()));
            }
        }

        debug_assert_eq!(result.len(), power);
        result
    }
}

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
