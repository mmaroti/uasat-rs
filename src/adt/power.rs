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

use super::{BooleanAlgebra, Countable, Domain, GenSlice, GenVec, SliceFor, VecFor};

/// The product of a list of domains.
#[derive(Clone)]
pub struct Power<BASE, EXP>
where
    BASE: Domain,
    EXP: Countable,
{
    base: BASE,
    exponent: EXP,
}

impl<BASE, EXP> Power<BASE, EXP>
where
    BASE: Domain,
    EXP: Countable,
{
    /// Creates the product domain from the given list of domains.
    pub fn new(base: BASE, exponent: EXP) -> Self {
        Self { base, exponent }
    }

    /// Returns the base domain of the power domain.
    pub fn base(&self) -> &BASE {
        &self.base
    }

    /// Returns the base domain of the power domain.
    pub fn exponent(&self) -> &EXP {
        &self.exponent
    }
}

impl<PART, EXP> Domain for Power<PART, EXP>
where
    PART: Domain,
    EXP: Countable,
{
    fn num_bits(&self) -> usize {
        self.base.num_bits() * self.exponent.size()
    }

    fn contains<ALG>(&self, alg: &mut ALG, elem: SliceFor<'_, ALG::Elem>) -> ALG::Elem
    where
        ALG: BooleanAlgebra,
    {
        let step = self.base().num_bits();
        let mut valid = alg.bool_lift(true);
        let mut pos = 0;
        while pos < elem.len() {
            let end = pos + step;
            let v = self.base.contains(alg, elem.slice(pos, end));
            valid = alg.bool_and(valid, v);
            pos = end;
        }
        assert!(pos == elem.len());
        valid
    }

    fn display_elem<'a>(
        &self,
        f: &mut std::fmt::Formatter<'a>,
        elem: SliceFor<'_, bool>,
    ) -> std::fmt::Result {
        let step = self.base().num_bits();
        let mut pos = 0;
        write!(f, "[")?;
        while pos < elem.len() {
            if pos > 0 {
                write!(f, ",")?
            }
            let end = pos + step;
            self.base.display_elem(f, elem.slice(pos, end))?;
            pos = end;
        }
        assert!(pos == elem.len());
        write!(f, "]")
    }
}

impl<BASE, EXP> Countable for Power<BASE, EXP>
where
    BASE: Countable,
    EXP: Countable,
{
    fn size(&self) -> usize {
        let mut result = 1;
        let base_size = self.base.size();
        for _ in 0..self.exponent.size() {
            result *= base_size;
        }
        result
    }

    fn elem(&self, index: usize) -> VecFor<bool> {
        let mut index = index;
        let base_size = self.base.size();
        let mut result: VecFor<bool> = GenVec::with_capacity(self.num_bits());
        for _ in 0..self.exponent.size() {
            let other = self.base.elem(index % base_size);
            result.extend(other);
            index /= base_size;
        }
        assert!(index == 0 && result.len() == self.num_bits());
        result
    }

    fn index(&self, elem: SliceFor<'_, bool>) -> usize {
        let mut index = 0;
        let base_size = self.base.size();
        let mut power = 1;

        let step = self.base().num_bits();
        let mut pos = 0;
        while pos < elem.len() {
            let end = pos + step;
            let sub_index = self.base.index(elem.slice(pos, end));
            index += sub_index * power;
            power *= base_size;
            pos = end;
        }
        assert!(pos == elem.len());

        index
    }
}

#[cfg(test)]
mod tests {
    use super::super::SmallSet;
    use super::*;
    use crate::core::{BooleanSolver, Bools, Solver};

    #[test]
    fn size() {
        let domain = Power::new(SmallSet::new(5), SmallSet::new(2));

        let mut solver = Solver::new("");
        let elem = domain.add_variable(&mut solver);
        let test = domain.contains(&mut solver, &elem);
        solver.bool_add_clause(&[test]);

        let num = solver.bool_find_num_models_method1(elem.iter().copied());
        assert_eq!(num, 25);
    }

    #[test]
    fn index() {
        let mut alg = Bools();
        let domain = Power::new(SmallSet::new(5), SmallSet::new(2));
        assert!(domain.size() == 25);

        for idx in 0..domain.size() {
            let elem = domain.elem(idx);
            assert!(domain.contains(&mut alg, elem.slice()));
            assert!(domain.index(elem.slice()) == idx);
        }
    }
}
