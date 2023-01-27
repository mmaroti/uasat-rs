/*
* Copyright (C) 2022, Miklos Maroti
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
    BooleanAlgebra, BooleanSolver, Countable, Domain, GenElem, GenVec, Power, SmallSet, VecFor,
    BOOLEAN,
};

#[derive(Debug, Clone)]
pub struct Relation<DOM, ELEM>
where
    DOM: Countable,
    ELEM: GenElem,
{
    domain: DOM,
    arity: usize,
    relation: VecFor<ELEM>,
}

impl<DOM, ELEM> Relation<DOM, ELEM>
where
    DOM: Countable,
    ELEM: GenElem,
{
    /// Creates a new relation with unspecified truth values.
    pub fn new_variable<ALG>(domain: DOM, arity: usize, alg: &mut ALG) -> Self
    where
        ALG: BooleanSolver<Elem = ELEM>,
    {
        let relations = Power::new(BOOLEAN, Power::new(domain.clone(), SmallSet::new(arity)));
        let relation = relations.add_variable(alg);
        Self {
            domain,
            arity,
            relation,
        }
    }

    pub fn is_reflexive<ALG>(&self, alg: &mut ALG) -> ELEM
    where
        ALG: BooleanAlgebra<Elem = ELEM>,
    {
        assert!(self.arity == 2);
        let size = self.domain.size();
        let mut pos = 0;
        let mut result = alg.bool_lift(true);
        for _ in 0..size {
            result = alg.bool_and(result, self.relation.get(pos));
            pos += size + 1;
        }
        result
    }
}

impl<DOM> Relation<DOM, bool>
where
    DOM: Countable,
{
    pub fn diagonal(domain: DOM) -> Self {
        let size = domain.size();
        let square = size * size;
        let mut relation: VecFor<bool> = GenVec::with_capacity(square);
        for i in 0..size {
            for j in 0..size {
                relation.push(i == j);
            }
        }
        Self {
            domain,
            arity: 2,
            relation,
        }
    }
}
