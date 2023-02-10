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

use super::{Boolean, BooleanSolver, Countable, Domain, GenVec, Power, SmallSet, VecFor};

pub trait Relations: Domain {
    /// Returns the arity of the relations.
    fn arity(&self) -> usize;

    fn zero<ALG>(&self, alg: &mut ALG) -> VecFor<ALG::Elem>
    where
        ALG: BooleanSolver;
}

impl<DOM> Relations for Power<Boolean, Power<DOM, SmallSet>>
where
    DOM: Countable,
{
    fn arity(&self) -> usize {
        self.exponent().exponent().size()
    }

    fn zero<ALG>(&self, alg: &mut ALG) -> VecFor<ALG::Elem>
    where
        ALG: BooleanSolver,
    {
        let mut vec: VecFor<ALG::Elem> = GenVec::with_capacity(self.num_bits());
        vec.resize(self.num_bits(), alg.bool_lift(false));
        vec
    }
}
