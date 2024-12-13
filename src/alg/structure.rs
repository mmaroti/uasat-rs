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

use super::{BooleanLogic, BooleanSolver, DirectedGraph, Domain, PartialOrder, Solver, Vector};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Symbol {
    name: &'static str,
    arity: usize,
}

impl Symbol {
    pub const fn new(name: &'static str, arity: usize) -> Self {
        Symbol { name, arity }
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn arity(&self) -> usize {
        self.arity
    }
}

pub trait Signature {
    const RELATIONS: &'static [Symbol];

    fn index(symbol: &Symbol) -> usize {
        Self::RELATIONS.iter().position(|s| s == symbol).unwrap()
    }
}

pub struct DirectedGraphSig;

pub const REL: Symbol = Symbol::new("rel", 2);

impl Signature for DirectedGraphSig {
    const RELATIONS: &'static [Symbol] = &[REL];
}

pub trait Structure<SIG>: Domain
where
    SIG: Signature,
{
    /// Returns true if there is an edge in the given relation index for the
    /// given elements.
    fn evaluate<LOGIC>(
        &self,
        relation: &Symbol,
        logic: &mut LOGIC,
        elems: &[LOGIC::Slice<'_>],
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic;

    /// Returns true if this structure is reflexive by constructing a suitable
    /// SAT problem and solving it.
    fn test_reflexivity(&self) -> bool {
        for symbol in SIG::RELATIONS.iter() {
            let mut logic = Solver::new("");
            let elem = self.add_variable(&mut logic);
            let elems = vec![elem.slice(); symbol.arity];
            let test = self.evaluate(symbol, &mut logic, &elems);
            logic.bool_add_clause1(logic.bool_not(test));
            if logic.bool_solvable() {
                return false;
            }
        }
        true
    }
}

impl<DOM> Structure<DirectedGraphSig> for DOM
where
    DOM: DirectedGraph,
{
    fn evaluate<LOGIC>(
        &self,
        relation: &Symbol,
        logic: &mut LOGIC,
        elems: &[LOGIC::Slice<'_>],
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(relation, &REL);
        assert_eq!(elems.len(), 2);
        DirectedGraph::is_edge(self, logic, elems[0], elems[1])
    }
}

pub struct PartialOrderSig;

impl Signature for PartialOrderSig {
    const RELATIONS: &'static [Symbol] = &[REL];
}

impl<DOM> Structure<PartialOrderSig> for DOM
where
    DOM: PartialOrder,
    DOM: Structure<DirectedGraphSig>,
{
    fn evaluate<LOGIC>(
        &self,
        relation: &Symbol,
        logic: &mut LOGIC,
        elems: &[LOGIC::Slice<'_>],
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
    {
        assert_eq!(relation, &REL);
        assert_eq!(elems.len(), 2);
        DirectedGraph::is_edge(self, logic, elems[0], elems[1])
    }
}

impl DirectedGraphSig {
    /// Returns true if the first element is connected to the second
    /// one.
    pub fn is_edge<'a, LOGIC, DOM>(
        dom: &DOM,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'a>,
        elem1: LOGIC::Slice<'a>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        DOM: Structure<Self>,
    {
        dom.evaluate(&REL, logic, &[elem0, elem1])
    }
}

impl PartialOrderSig {
    /// Returns true if the first element is strictly less than the
    /// second one.
    pub fn is_less_than<'a, LOGIC, DOM>(
        dom: &DOM,
        logic: &mut LOGIC,
        elem0: LOGIC::Slice<'a>,
        elem1: LOGIC::Slice<'a>,
    ) -> LOGIC::Elem
    where
        LOGIC: BooleanLogic,
        DOM: Structure<Self> + Structure<DirectedGraphSig>,
    {
        let test0 = DirectedGraphSig::is_edge(dom, logic, elem0, elem1);
        let test1 = DirectedGraphSig::is_edge(dom, logic, elem1, elem0);
        let test1 = logic.bool_not(test1);
        logic.bool_and(test0, test1)
    }
}
