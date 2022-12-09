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

use super::{BooleanAlgebra, Domain};

/// The product of a list of domains.
#[derive(Clone)]
pub struct Product<BASE>
where
    BASE: Domain,
{
    parts: Box<[BASE]>,
}

impl<BASE> Product<BASE>
where
    BASE: Domain,
{
    /// Creates the product domain from the given list of domains.
    pub fn new(parts: Vec<BASE>) -> Self {
        let parts = parts.into_boxed_slice();
        Self { parts }
    }

    /// Returns the list of parts of this product domain.
    pub fn parts(&self) -> &[BASE] {
        &self.parts
    }
}

impl<PART> Domain for Product<PART>
where
    PART: Domain,
{
    fn num_bits(&self) -> usize {
        self.parts.iter().map(|p| p.num_bits()).sum()
    }

    fn contains<ALG>(&self, alg: &mut ALG, elem: &[ALG::Elem]) -> ALG::Elem
    where
        ALG: BooleanAlgebra,
    {
        let mut valid = alg.bool_lift(true);
        let mut pos = 0;
        for p in self.parts.iter() {
            let end = pos + p.num_bits();
            let v = p.contains(alg, &elem[pos..end]);
            valid = alg.bool_and(valid, v);
            pos = end;
        }
        assert!(pos == elem.len());
        valid
    }
}
