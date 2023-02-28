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

use super::{
    Boolean, BooleanLattice, Countable, Domain, Slice, Vector, Power, SmallSet, BOOLEAN,
};

pub trait Relations: BooleanLattice {
    /// Returns the arity of the relations.
    fn arity(&self) -> usize;

    /// Creates a new element of the given arity from the given old element with
    /// permuted, identified or new dummy coordinates. The mapping is a vector
    /// of length of the original relation with entries identifying the matching
    /// coordinates in the new relation.
    fn polymer<ELEM>(&self, elem: ELEM, arity: usize, mapping: &[usize]) -> <ELEM as Slice>::Vec
    where
        ELEM: Slice;

    /// Returns the diagonal relation of the given relation.
    fn diagonal<ELEM>(&self, elem: ELEM) -> <ELEM as Slice>::Vec
    where
        ELEM: Slice,
    {
        assert!(self.arity() >= 1);
        self.polymer(elem, 1, &vec![0; self.arity()])
    }
}

impl<DOM> Relations for Power<Boolean, Power<DOM, SmallSet>>
where
    DOM: Countable,
{
    fn arity(&self) -> usize {
        self.exponent().exponent().size()
    }

    fn polymer<ELEM>(&self, elem: ELEM, arity: usize, mapping: &[usize]) -> <ELEM as Slice>::Vec
    where
        ELEM: Slice,
    {
        assert_eq!(elem.len(), self.num_bits());
        assert_eq!(mapping.len(), self.arity());

        let mut strides = vec![(0, 0); arity];
        let size = self.exponent().base().size();
        let mut power: usize = 1;
        for &i in mapping {
            assert!(i < arity);
            strides[i].0 += power;
            power *= size;
        }

        let domain = Power::new(
            BOOLEAN,
            Power::new(self.exponent().base().clone(), SmallSet::new(arity)),
        );
        let mut result: <ELEM as Slice>::Vec = Vector::with_capacity(domain.num_bits());

        let mut index = 0;
        'outer: loop {
            result.extend(self.part(elem, index).copy_iter());

            for stride in strides.iter_mut() {
                index += stride.0;
                stride.1 += 1;
                if stride.1 >= size {
                    index -= stride.0 * size;
                    stride.1 = 0;
                } else {
                    continue 'outer;
                }
            }

            break;
        }

        debug_assert_eq!(result.len(), domain.num_bits());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BitVec, Logic};
    use super::*;

    #[test]
    fn polymer() {
        let dom = SmallSet::new(3);
        let rel1 = Power::new(BOOLEAN, Power::new(dom.clone(), SmallSet::new(1)));
        let rel2 = Power::new(BOOLEAN, Power::new(dom.clone(), SmallSet::new(2)));

        assert_eq!(rel1.arity(), 1);
        assert_eq!(rel2.arity(), 2);

        let mut logic = Logic();

        let elem1: BitVec = vec![false, true, false].into_iter().collect();
        assert!(rel1.contains(&mut logic, elem1.slice()));

        let elem2: BitVec = vec![false, true, true, false, true, true, false, false, false]
            .into_iter()
            .collect();
        assert!(rel2.contains(&mut logic, elem2.slice()));

        let elem3: BitVec = vec![false, false, false, true, true, false, true, true, false]
            .into_iter()
            .collect();
        assert!(rel2.contains(&mut logic, elem3.slice()));

        let elem4 = rel2.polymer(elem2.slice(), 2, &[1, 0]);
        assert_eq!(elem3, elem4);

        let elem5 = rel2.polymer(elem2.slice(), 1, &[0, 0]);
        assert_eq!(elem1, elem5);
    }
}
