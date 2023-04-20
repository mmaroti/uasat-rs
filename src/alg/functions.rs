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

use super::{Countable, Domain, Power, Slice, SmallSet, Vector};

/// A domain containing functions of a fixed arity from a domain to a codomain.
pub type Functions<DOM, COD> = Power<COD, Power<DOM, SmallSet>>;

impl<DOM, COD> Functions<DOM, COD>
where
    DOM: Countable,
    COD: Domain,
{
    /// Creates a new function domain from the given domain to
    /// the target codomain.
    pub fn new_functions(dom: DOM, cod: COD, arity: usize) -> Self {
        Power::new(cod, Power::new(dom, SmallSet::new(arity)))
    }

    /// Returns the arity (rank) of all functions in the domain.
    pub fn arity(&self) -> usize {
        self.exponent().exponent().size()
    }

    /// Returns the domain of the functions.
    pub fn domain(&self) -> &DOM {
        self.exponent().base()
    }

    /// Returns another domain of functions with same domand and codomain
    /// but with the new given arity.
    pub fn change_arity(&self, arity: usize) -> Self {
        Functions::new_functions(self.domain().clone(), self.base().clone(), arity)
    }

    /// Creates a new function of the given arity from an old function with
    /// permuted, identified and/or new dummy coordinates. The mapping is a
    /// vector of length of the arity of the original function with entries
    /// identifying the matching coordinates in the new function.
    pub fn polymer<'a, SLICE>(&self, elem: SLICE, arity: usize, mapping: &[usize]) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(mapping.len(), self.arity());

        let mut strides: Vec<(usize, usize, usize)> = vec![(0, 0, 0); arity];
        let size = self.domain().size();
        let mut power: usize = 1;
        for &i in mapping {
            assert!(i < arity);
            strides[i].0 += power;
            power *= size;
        }

        power = 1;
        for s in strides.iter_mut() {
            s.2 = size * s.0;
            power *= size;
        }

        let mut result: SLICE::Vector = Vector::with_capacity(self.base().num_bits() * power);
        let mut index = 0;
        'outer: loop {
            result.extend(self.part(elem, index).copy_iter());

            for stride in strides.iter_mut() {
                index += stride.0;
                stride.1 += 1;
                if stride.1 >= size {
                    stride.1 = 0;
                    index -= stride.2;
                } else {
                    continue 'outer;
                }
            }

            break;
        }

        debug_assert_eq!(result.len(), self.base().num_bits() * power);
        result
    }

    /// Returns the unary function with all variables identified.
    pub fn identify<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        assert!(self.arity() >= 1);
        self.polymer(elem, 1, &vec![0; self.arity()])
    }

    /// Reverses the set of coordinates of the given function.
    pub fn converse<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        let map: Vec<usize> = (0..self.arity()).rev().collect();
        self.polymer(elem, map.len(), &map)
    }

    /// Rotates the coordinate of the function to the right, such that
    /// f(x,y,z) becomes f(z,x,y).
    pub fn rotate_right<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        assert!(self.arity() >= 1);
        let map: Vec<usize> = if self.arity() <= 1 {
            (0..self.arity()).collect()
        } else {
            (1..self.arity()).chain(std::iter::once(0)).collect()
        };
        self.polymer(elem, map.len(), &map)
    }

    /// Rotates the coordinate of the function to the left, such that
    /// f(x,y,z) becomes f(y,z,x).
    pub fn rotate_left<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        assert!(self.arity() >= 1);
        let map: Vec<usize> = if self.arity() <= 1 {
            (0..self.arity()).collect()
        } else {
            std::iter::once(self.arity() - 1)
                .chain(0..self.arity() - 1)
                .collect()
        };
        self.polymer(elem, map.len(), &map)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BitVec, Domain, Logic, Vector};
    use super::*;

    #[test]
    fn polymer() {
        let dom0 = SmallSet::new(2);
        let dom1 = SmallSet::new(3);
        let op0 = Power::new(dom0.clone(), Power::new(dom1.clone(), SmallSet::new(1)));
        let op1 = Power::new(dom0.clone(), Power::new(dom1.clone(), SmallSet::new(2)));

        assert_eq!(op0.arity(), 1);
        assert_eq!(op1.arity(), 2);

        let mut logic = Logic();

        let elem1: BitVec = vec![false, true, true, false, false, true]
            .into_iter()
            .collect();
        assert!(op0.contains(&mut logic, elem1.slice()));

        let elem2: BitVec = vec![
            false, true, true, false, true, false, false, true, true, false, true, false, false,
            true, false, true, false, true,
        ]
        .into_iter()
        .collect();
        assert!(op1.contains(&mut logic, elem2.slice()));

        let elem3: BitVec = vec![
            false, true, false, true, false, true, true, false, true, false, false, true, true,
            false, true, false, false, true,
        ]
        .into_iter()
        .collect();
        assert!(op1.contains(&mut logic, elem3.slice()));

        let elem4 = op1.polymer(elem2.slice(), 2, &[1, 0]);
        assert_eq!(elem3, elem4);

        let elem5 = op1.polymer(elem2.slice(), 1, &[0, 0]);
        assert_eq!(elem1, elem5);
    }
}
