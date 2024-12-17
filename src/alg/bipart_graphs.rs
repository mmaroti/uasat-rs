/*
* Copyright (C) 2022-2024, Miklos Maroti
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

use super::{Boolean, Indexable, Power, Slice};

#[derive(Debug, Clone, PartialEq)]
pub struct BipartGrpahs<DOM0, DOM1>
where
    DOM0: Indexable,
    DOM1: Indexable,
{
    domain0: DOM0,
    domain1: DOM1,
    power: Power<Boolean>,
}

impl<DOM0, DOM1> BipartGrpahs<DOM0, DOM1>
where
    DOM0: Indexable,
    DOM1: Indexable,
{
    /// Creates a new domain for bipartate graphs between the given domains.
    pub fn new(domain0: DOM0, domain1: DOM1) -> Self {
        let exponent = domain0.size() * domain1.size();

        BipartGrpahs {
            domain0,
            domain1,
            power: Power::new(Boolean(), exponent),
        }
    }

    /// Returns the first domain of the bipartate graph.
    pub fn domain0(&self) -> &DOM0 {
        &self.domain0
    }

    /// Returns the second domain of the bipartate graph.
    pub fn domain1(&self) -> &DOM0 {
        &self.domain0
    }

    /// Creates a new relation of the given arity from an old relation with
    /// permuted, identified and/or new dummy coordinates. The mapping is a
    /// vector of length of the arity of the original relation with entries
    /// identifying the matching coordinates in the new relation.
    pub fn polymer<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        // TODO: implement this
        elem.copy_iter().collect()
    }
}
