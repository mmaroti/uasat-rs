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

use super::{Countable, Domain, Power, PowerDomain, Slice, SmallSet, Vector};

/// A domain of function from a fixed domain and codomain.
pub trait Functions: PowerDomain
where
    Self::Exp: PowerDomain,
    <Self::Exp as PowerDomain>::Base: Countable,
{
    /// Returns the arity (rank) of all functions in the domain.
    fn arity(&self) -> usize {
        self.exponent().exponent().size()
    }

    /// Returns the domain of the functions.
    fn domain(&self) -> &<Self::Exp as PowerDomain>::Base {
        self.exponent().base()
    }

    /// Returns the domain of functions with the given arity.
    fn change_arity(&self, arity: usize) -> Self;

    /// Creates a new function of the given arity from an old function with
    /// permuted, identified and/or new dummy coordinates. The mapping is a
    /// vector of length of the arity of the original element with entries
    /// identifying the matching coordinates in the new function.
    fn polymer<'a, SLICE>(&self, elem: SLICE, arity: usize, mapping: &[usize]) -> SLICE::Vector
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
    fn identify<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        assert!(self.arity() >= 1);
        self.polymer(elem, 1, &vec![0; self.arity()])
    }

    /// Reverses the set of coordinates of the given function.
    fn converse<'a, SLICE>(&self, elem: SLICE) -> SLICE::Vector
    where
        SLICE: Slice<'a>,
    {
        let map: Vec<usize> = (0..self.arity()).rev().collect();
        self.polymer(elem, map.len(), &map)
    }
}

impl<DOM0, DOM1> Functions for Power<DOM0, Power<DOM1, SmallSet>>
where
    DOM0: Domain,
    DOM1: Countable,
{
    fn change_arity(&self, arity: usize) -> Self {
        Power::new(
            self.base().clone(),
            Power::new(self.domain().clone(), SmallSet::new(arity)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BitVec, Domain, Functions, Logic, Vector};
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

        let elem4 = <_ as Functions>::polymer(&op1, elem2.slice(), 2, &[1, 0]);
        assert_eq!(elem3, elem4);

        let elem5 = <_ as Functions>::polymer(&op1, elem2.slice(), 1, &[0, 0]);
        assert_eq!(elem1, elem5);
    }
}
