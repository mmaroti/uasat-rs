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
    BitSlice, Boolean, BooleanLogic, BoundedOrder, Indexable, Domain, Monoid, Power, Relations,
    Slice, SmallSet, UnaryOperations, Vector,
};

/// A domain containing operations of a fixed arity.
#[derive(Debug, Clone, PartialEq)]
pub struct Operations<DOM>(Power<DOM, Power<DOM, SmallSet>>)
where
    DOM: Indexable;

impl<DOM> Operations<DOM>
where
    DOM: Indexable,
{
    /// Creates a domain containing operationf of a fixed arity.
    pub fn new(dom: DOM, arity: usize) -> Self {
        Operations(Power::new(
            dom.clone(),
            Power::new(dom, SmallSet::new(arity)),
        ))
    }

    /// Returns the arity (rank) of all operations in the domain.
    pub fn arity(&self) -> usize {
        self.0.exponent().exponent().size()
    }

    /// Returns the domain of the operations.
    pub fn domain(&self) -> &DOM {
        self.0.exponent().base()
    }

    /// Creates a new operation of the given arity from an old operation with
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

        let mut result: SLICE::Vector = Vector::with_capacity(self.domain().num_bits() * power);
        if power == 0 {
            return result;
        }

        let mut index = 0;
        'outer: loop {
            result.extend(self.0.part(elem, index).copy_iter());

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

        debug_assert_eq!(result.len(), self.domain().num_bits() * power);
        result
    }

    /// Returns the graph of the given operation, which is a relation
    /// of arity one larger than this operation.
    pub fn as_relation<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(elem.len(), self.num_bits());
        let domain = self.domain();

        let size = domain.size();
        let mut power = size;
        for _ in 0..self.arity() {
            power *= size;
        }

        let mut result: LOGIC::Vector = Vector::with_capacity(power);
        for part in self.0.part_iter(elem) {
            let mut value = domain.onehot(logic, part);
            result.append(&mut value);
        }

        debug_assert_eq!(result.len(), power);
        result
    }

    /// Returns a unary relation containing the range of the given operation.
    pub fn range<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let graph = self.as_relation(logic, elem);
        let rels = Relations::new(self.domain().clone(), self.arity() + 1);
        rels.project(logic, graph.slice(), &[0])
    }

    /// Returns true if the given element is a surjective operation.
    pub fn is_surjective<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let range = self.range(logic, elem);
        let dom = Power::new(Boolean(), self.domain().clone());
        dom.is_top(logic, range.slice())
    }

    /// Returns the unary identity operation.
    pub fn get_projection<LOGIC>(&self, logic: &mut LOGIC, coord: usize) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert!(coord < self.arity());
        let dom = UnaryOperations::new(self.domain().clone());
        let result = dom.get_identity(logic);
        self.polymer(result.slice(), self.arity(), &[coord])
    }
}

impl<DOM> Domain for Operations<DOM>
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

impl<DOM> Indexable for Operations<DOM>
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

    #[inline]
    fn onehot<LOGIC>(&self, logic: &mut LOGIC, elem: LOGIC::Slice<'_>) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        self.0.onehot(logic, elem)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BitVec, Domain, Logic, SmallSet, Solver, Vector, BOOLEAN};
    use super::*;

    #[test]
    fn graph() {
        let dom = SmallSet::new(3);
        let ops = Operations::new(dom.clone(), 1);
        let rel = Power::new(BOOLEAN, Power::new(dom.clone(), SmallSet::new(2)));

        let mut logic = Logic();
        let elem1: BitVec = vec![true, false, false, false, false, true, false, true, false]
            .into_iter()
            .collect();
        assert!(ops.contains(&mut logic, elem1.slice()));

        let graph1 = ops.as_relation(&mut logic, elem1.slice());
        assert!(rel.contains(&mut logic, graph1.slice()));
        assert_eq!(elem1, graph1);

        let mut solver = Solver::new("");
        let elem2 = ops.add_variable(&mut solver);
        let graph2 = ops.as_relation(&mut solver, elem2.slice());
        assert_eq!(elem2, graph2);

        let dom = BOOLEAN;
        let ops = Operations::new(dom.clone(), 1);

        let elem3 = ops.add_variable(&mut solver);
        let graph3 = ops.as_relation(&mut solver, elem3.slice());
        assert_eq!(elem3.len(), 2);
        assert_eq!(graph3.len(), 4);
        assert_eq!(graph3.get(0), solver.bool_not(elem3.get(0)));
        assert_eq!(graph3.get(1), elem3.get(0));
        assert_eq!(graph3.get(2), solver.bool_not(elem3.get(1)));
        assert_eq!(graph3.get(3), elem3.get(1));
    }
}
