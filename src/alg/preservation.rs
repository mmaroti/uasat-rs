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
    BipartiteGraph, BooleanLogic, BoundedOrder, DirectedGraph, Domain, Indexable, MeetSemilattice,
    Operations, Relations, Slice, Vector,
};

/// The preservation relation bipartite graph between the domain
/// of operations with a fixed arity and the domain of relations
/// with a fixed arity over a common domain.
#[derive(Debug, Clone, PartialEq)]
pub struct Preservation<DOM> {
    ops: Operations<DOM>,
    rels: Relations<DOM>,
}

impl<DOM> Preservation<DOM>
where
    DOM: Indexable,
{
    /// Creates a new preservation relation over the given domain.
    pub fn new(domain: DOM, op_arity: usize, rel_arity: usize) -> Self {
        Self {
            ops: Operations::new(domain.clone(), op_arity),
            rels: Relations::new(domain, rel_arity),
        }
    }

    /// Returns the underlying domain.
    pub fn domain(&self) -> &DOM {
        self.rels.domain()
    }

    /// Returns the arity of the operations.
    pub fn op_arity(&self) -> usize {
        self.ops.arity()
    }

    /// Returns the arity of the relations.
    pub fn rel_arity(&self) -> usize {
        self.rels.arity()
    }

    /// Takes an operation and a list of relations and calculates
    /// the relation which contain all the tuples that can be obtained
    /// using the operation applied to tuples in the relations.
    pub fn evaluate<LOGIC>(
        &self,
        logic: &mut LOGIC,
        operation: LOGIC::Slice<'_>,
        relations: &[LOGIC::Slice<'_>],
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(operation.len(), self.ops.num_bits());
        assert_eq!(self.ops.arity(), relations.len());
        for rel in relations {
            assert_eq!(rel.len(), self.rels.num_bits());
        }

        self.evaluate_nm(logic, operation, relations)
    }

    /// This is a general but suboptimal version of the evaluate method.
    fn evaluate_nm<LOGIC>(
        &self,
        logic: &mut LOGIC,
        operation: LOGIC::Slice<'_>,
        relations: &[LOGIC::Slice<'_>],
    ) -> LOGIC::Vector
    where
        LOGIC: BooleanLogic,
    {
        let op_arity = self.ops.arity();
        let rel_arity = self.rels.arity();

        // convert the operation to a relation
        let operation = self.ops.as_relation(logic, operation);
        let oper = self.rels.change_arity(op_arity + 1);
        debug_assert_eq!(operation.len(), oper.num_bits());

        let arity = rel_arity * op_arity + rel_arity;
        let dom = self.rels.change_arity(arity);
        let mut result = dom.get_top(logic);

        // apply the relations
        let mut mapping = vec![0; rel_arity];
        #[allow(clippy::needless_range_loop)]
        for j in 0..op_arity {
            for i in 0..rel_arity {
                mapping[i] = rel_arity * j + i;
            }
            let rel = self.rels.polymer(relations[j], arity, &mapping);
            result = dom.meet(logic, result.slice(), rel.slice());
        }

        // apply the operations
        let mut mapping = vec![0; op_arity + 1];
        for i in 0..rel_arity {
            mapping[0] = rel_arity * op_arity + i;
            for j in 0..op_arity {
                mapping[1 + j] = rel_arity * j + i;
            }
            let op = oper.polymer(operation.slice(), arity, &mapping);
            result = dom.meet(logic, result.slice(), op.slice());
        }

        let result = dom.fold_any(logic, result.slice(), rel_arity * op_arity);
        result
    }

    /// Tests if the given operation preserves the given relation.
    pub fn preserves<LOGIC>(
        &self,
        logic: &mut LOGIC,
        operation: LOGIC::Slice<'_>,
        relation: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        debug_assert_eq!(operation.len(), self.ops.num_bits());
        debug_assert_eq!(relation.len(), self.rels.num_bits());

        let relations = vec![relation; self.ops.arity()];
        let result = self.evaluate(logic, operation, &relations);
        self.rels.is_edge(logic, result.slice(), relation)
    }
}

impl<DOM> BipartiteGraph for Preservation<DOM>
where
    DOM: Indexable,
{
    type Domain0 = Operations<DOM>;
    type Domain1 = Relations<DOM>;

    fn dom0(&self) -> &Self::Domain0 {
        &self.ops
    }

    fn dom1(&self) -> &Self::Domain1 {
        &self.rels
    }

    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        self.preserves(logic, elem0, elem1)
    }
}
