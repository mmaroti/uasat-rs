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
    BipartiteGraph, BitVec, Boolean, BooleanLogic, Countable, Domain, DomainPair, Power, Product2,
    Vector,
};

#[derive(Debug, Clone)]

pub struct WrapElem<DOM>
where
    DOM: Domain,
{
    domain: DOM,
    elem: BitVec,
}

impl<DOM> WrapElem<DOM>
where
    DOM: Domain,
{
    /// Creates a new domain that wraps the given element.
    pub fn new(domain: DOM, elem: BitVec) -> Self {
        assert_eq!(elem.len(), domain.num_bits());
        Self { domain, elem }
    }
}

impl<DOM0, DOM1> DomainPair<DOM0, DOM1> for WrapElem<Power<Boolean, Product2<DOM0, DOM1>>>
where
    DOM0: Countable,
    DOM1: Countable,
{
    fn domain(&self) -> &DOM0 {
        self.domain.exponent().dom0()
    }

    fn codomain(&self) -> &DOM1 {
        self.domain.exponent().dom1()
    }
}

impl<DOM0, DOM1> BipartiteGraph<DOM0, DOM1> for WrapElem<Power<Boolean, Product2<DOM0, DOM1>>>
where
    DOM0: Countable,
    DOM1: Countable,
{
    fn is_edge<LOGIC>(
        &self,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'_>,
        elem1: LOGIC::Slice<'_>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        let elem0 = self.domain().onehot(logic, elem0);
        let elem1 = self.domain().onehot(logic, elem1);
        debug_assert_eq!(elem0.len(), self.domain().size());
        debug_assert_eq!(elem1.len(), self.codomain().size());

        let mut iter = self.elem.copy_iter();
        let mut result = logic.bool_zero();
        for e1 in elem1.copy_iter() {
            for e0 in elem0.copy_iter() {
                let val = iter.next().unwrap();
                if !val {
                    continue;
                }
                let val = logic.bool_and(e0, e1);
                result = logic.bool_or(result, val);
            }
        }
        debug_assert!(iter.next().is_none());

        result
    }
}
