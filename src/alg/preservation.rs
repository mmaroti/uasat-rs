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
pub struct PreservationRelation<DOM> {
    oper: Operations<DOM>,
    rels: Relations<DOM>,
}

impl<DOM> PreservationRelation<DOM>
where
    DOM: Indexable,
{
    /// Creates a new preservation relation over the given domain.
    pub fn new(domain: DOM, oper_arity: usize, rels_arity: usize) -> Self {
        Self {
            oper: Operations::new(domain.clone(), oper_arity),
            rels: Relations::new(domain, rels_arity),
        }
    }

    /// Returns the underlying domain.
    pub fn domain(&self) -> &DOM {
        &self.rels.domain()
    }

    /// Returns the arity of the operations.
    pub fn oper_arity(&self) -> usize {
        self.oper.arity()
    }

    /// Returns the arity of the relations.
    pub fn rels_arity(&self) -> usize {
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
        assert_eq!(operation.len(), self.oper.num_bits());
        assert_eq!(self.oper.arity(), relations.len());
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
        let oper_arity = self.oper.arity();
        let rels_arity = self.rels.arity();

        // convert the operation to a relation
        let operation = self.oper.as_relation(logic, operation);
        let oper = self.rels.change_arity(oper_arity + 1);
        debug_assert_eq!(operation.len(), oper.num_bits());

        let arity = rels_arity * oper_arity + rels_arity;
        let dom = self.rels.change_arity(arity);
        let mut result = dom.get_top(logic);

        // apply the relations
        let mut mapping = vec![0; rels_arity];
        for j in 0..oper_arity {
            for i in 0..rels_arity {
                mapping[i] = rels_arity * j + i;
            }
            let rel = self.rels.polymer(relations[j], arity, &mapping);
            result = dom.meet(logic, result.slice(), rel.slice());
        }

        // apply the operations
        let mut mapping = vec![0; oper_arity + 1];
        for i in 0..rels_arity {
            mapping[0] = rels_arity * oper_arity + i;
            for j in 0..oper_arity {
                mapping[1 + j] = rels_arity * j + i;
            }
            let op = oper.polymer(operation.slice(), arity, &mapping);
            result = dom.meet(logic, result.slice(), op.slice());
        }

        let result = dom.fold_any(logic, result.slice(), rels_arity * oper_arity);
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
        debug_assert_eq!(operation.len(), self.oper.num_bits());
        debug_assert_eq!(relation.len(), self.rels.num_bits());

        let relations = vec![relation; self.oper.arity()];
        let result = self.evaluate(logic, operation, &relations);
        self.rels.is_edge(logic, result.slice(), relation)
    }
}

impl<DOM> BipartiteGraph for PreservationRelation<DOM>
where
    DOM: Indexable,
{
    type Domain0 = Operations<DOM>;
    type Domain1 = Relations<DOM>;

    fn dom0(&self) -> &Self::Domain0 {
        &self.oper
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
