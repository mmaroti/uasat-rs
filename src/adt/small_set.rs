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

use super::{BooleanAlgebra, Domain, GenSlice, SliceFor};

/// A small set encoded as a one-hot vector of booleans.
#[derive(Clone)]
pub struct SmallSet {
    size: usize,
}

impl SmallSet {
    /// Creates a new small set of the given size.
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl Domain for SmallSet {
    fn num_bits(&self) -> usize {
        self.size
    }

    fn contains<ALG>(&self, alg: &mut ALG, elem: SliceFor<'_, ALG::Elem>) -> ALG::Elem
    where
        ALG: BooleanAlgebra,
    {
        alg.bool_fold_one(elem.copy_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{BooleanSolver, Solver};

    #[test]
    fn size() {
        let domain = SmallSet::new(5);

        let mut solver = Solver::new("");
        let elem = domain.add_variable(&mut solver);
        let test = domain.contains(&mut solver, &elem);
        solver.bool_add_clause(&[test]);

        let num = solver.bool_find_num_models_method1(elem.iter().cloned());
        assert_eq!(num, 5);
    }
}
