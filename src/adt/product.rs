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

use super::{BooleanAlgebra, Countable, Domain, GenSlice, GenVec, SliceFor, VecFor};

/// The product of two domains.
#[derive(Debug, Clone)]
pub struct Product2<DOM0, DOM1>
where
    DOM0: Domain,
    DOM1: Domain,
{
    dom0: DOM0,
    dom1: DOM1,
}

impl<DOM0, DOM1> Product2<DOM0, DOM1>
where
    DOM0: Domain,
    DOM1: Domain,
{
    /// Creates the product of two domains.
    pub fn new(dom0: DOM0, dom1: DOM1) -> Self {
        Self { dom0, dom1 }
    }

    /// Returns the first domain of this product.
    pub fn dom0(&self) -> &DOM0 {
        &self.dom0
    }

    /// Returns the second domain of this product.
    pub fn dom1(&self) -> &DOM1 {
        &self.dom1
    }
}

impl<DOM0, DOM1> Domain for Product2<DOM0, DOM1>
where
    DOM0: Domain,
    DOM1: Domain,
{
    fn num_bits(&self) -> usize {
        self.dom0.num_bits() + self.dom1.num_bits()
    }

    fn contains<ALG>(&self, alg: &mut ALG, elem: SliceFor<'_, ALG::Elem>) -> ALG::Elem
    where
        ALG: BooleanAlgebra,
    {
        let bits0 = self.dom0.num_bits();
        let valid0 = self.dom0.contains(alg, elem.slice(0, bits0));
        let valid1 = self.dom1.contains(alg, elem.slice(bits0, elem.len()));
        alg.bool_and(valid0, valid1)
    }

    fn display_elem<'a>(
        &self,
        f: &mut std::fmt::Formatter<'a>,
        elem: SliceFor<'_, bool>,
    ) -> std::fmt::Result {
        let bits0 = self.dom0.num_bits();
        write!(f, "(")?;
        self.dom0.display_elem(f, elem.slice(0, bits0))?;
        write!(f, ",")?;
        self.dom1.display_elem(f, elem.slice(bits0, elem.len()))?;
        write!(f, ")")
    }
}

impl<DOM0, DOM1> Countable for Product2<DOM0, DOM1>
where
    DOM0: Countable,
    DOM1: Countable,
{
    fn size(&self) -> usize {
        self.dom0.size() * self.dom1.size()
    }

    fn elem(&self, index: usize) -> VecFor<bool> {
        let size0 = self.dom0.size();
        let mut result: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        result.extend(self.dom0.elem(index % size0));
        result.extend(self.dom1.elem(index / size0));
        debug_assert!(result.len() == self.num_bits());
        result
    }

    fn index(&self, elem: SliceFor<'_, bool>) -> usize {
        debug_assert!(elem.len() == self.num_bits());
        let bits0 = self.dom0.num_bits();
        let part0 = self.dom0.index(elem.slice(0, bits0));

        let size0 = self.dom0.size();
        part0 + size0 * self.dom1.index(elem.slice(bits0, elem.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::super::SmallSet;
    use super::*;
    use crate::core::{BooleanSolver, Bools, Solver};

    #[test]
    fn size() {
        let domain = Product2::new(SmallSet::new(5), SmallSet::new(3));

        let mut solver = Solver::new("");
        let elem = domain.add_variable(&mut solver);
        let test = domain.contains(&mut solver, &elem);
        solver.bool_add_clause(&[test]);

        let num = solver.bool_find_num_models_method1(elem.iter().copied());
        assert_eq!(num, 15);
    }

    #[test]
    fn index() {
        let mut alg = Bools();
        let domain = Product2::new(SmallSet::new(5), SmallSet::new(3));
        assert!(domain.size() == 15);

        for idx in 0..domain.size() {
            let elem = domain.elem(idx);
            assert!(domain.contains(&mut alg, elem.slice()));
            assert!(domain.index(elem.slice()) == idx);
        }
    }
}
