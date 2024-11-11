/*
* Copyright (C) 2024, Miklos Maroti
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

use super::{Boolean, BooleanLogic, Indexable, MeetSemilattice, Power, Slice, SmallSet, Vector};

pub struct RelationalClone<DOM> {
    domain: DOM,
}

impl<DOM> RelationalClone<DOM>
where
    DOM: Indexable,
{
    /// Creates a new relational clone over the given domain.
    pub fn new(domain: DOM) -> Self {
        Self { domain }
    }

    /// Returns the underlying domain.
    pub fn domain(&self) -> &DOM {
        &self.domain
    }

    /// Returns the domain of relations of the given arity.
    pub fn relations(&self, arity: usize) -> Power<Boolean, Power<DOM, SmallSet>> {
        Power::new(
            Boolean(),
            Power::new(self.domain.clone(), SmallSet::new(arity)),
        )
    }

    pub fn relation<VECTOR>(&self, arity: usize, elem: VECTOR) -> (usize, VECTOR)
    where
        VECTOR: Vector,
    {
        (arity, elem)
    }

    /// Returns the element part of the given relation.
    pub fn elem<'a, SLICE>(&self, relation: (usize, SLICE)) -> SLICE
    where
        SLICE: Slice<'a>,
    {
        relation.1
    }

    /// Returns the arity of the given relation.
    pub fn arity<'a>(&self, relation: (usize, impl Slice<'a>)) -> usize {
        relation.0
    }

    /// Calculates the meet of a pair of relations of the same arity.
    pub fn meet<LOGIC>(
        &self,
        logic: &mut LOGIC,
        rel0: (usize, LOGIC::Slice<'_>),
        rel1: (usize, LOGIC::Slice<'_>),
    ) -> (usize, LOGIC::Vector)
    where
        LOGIC: BooleanLogic,
    {
        let arity = rel0.0;
        assert_eq!(arity, self.arity(rel1));
        let rels = self.relations(arity);
        let elem = rels.meet(logic, self.elem(rel0), self.elem(rel1));
        (arity, elem)
    }
}
