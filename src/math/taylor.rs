/*
* Copyright (C) 2020, Miklos Maroti
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

#![allow(dead_code)]

use crate::alg::{
    BinaryRelations, BoundedOrder, Domain, MeetSemilattice, Monoid, Semigroup, SmallSet,
};
use crate::core::Logic;
use crate::genvec::{BitSlice, BitVec, Vector};

#[derive(Debug)]
pub struct BinaryRelClone {
    domain: BinaryRelations<SmallSet>,
    elems: Vec<BitVec>,
    closed: usize,
}

impl BinaryRelClone {
    pub fn new(size: usize) -> Self {
        let logic = Logic();
        let domain = BinaryRelations::new(SmallSet::new(size));
        let elems: Vec<BitVec> = vec![domain.get_top(&logic), domain.get_identity(&logic)];
        let closed = 2;
        Self {
            domain,
            elems,
            closed,
        }
    }

    pub fn contains(&self, elem: BitSlice) -> bool {
        assert_eq!(elem.len(), self.domain.num_bits());
        let mut logic = Logic();
        assert!(self.domain.contains(&mut logic, elem));

        for other in self.elems.iter() {
            if self.domain.equals(&mut logic, elem, other.slice()) {
                return true;
            }
        }
        false
    }

    pub fn add_from_str(&mut self, elem: &str) {
        let elem: BitVec = elem.chars().map(|c| c != '0').collect();
        self.add(elem);
    }

    pub fn add(&mut self, elem: BitVec) {
        if !self.contains(elem.slice()) {
            println!("{}", self.domain.format(elem.slice()));
            self.elems.push(elem);
        }
    }

    pub fn print(&self) {
        for (idx, elem) in self.elems.iter().enumerate() {
            println!("{}: {}", idx, self.domain.format(elem.slice()));
        }
    }

    pub fn close(&mut self) {
        let mut logic = Logic();
        while self.closed < self.elems.len() {
            let elem0 = self.elems[self.closed].clone();

            let elem1 = self.domain.converse(elem0.slice());
            self.add(elem1);

            for idx in 0..self.closed {
                let elem1 = &self.elems[idx];

                let elem2 = self.domain.meet(&mut logic, elem0.slice(), elem1.slice());

                let elem3 = self
                    .domain
                    .product(&mut logic, elem0.slice(), elem1.slice());

                let elem4 = self
                    .domain
                    .product(&mut logic, elem1.slice(), elem0.slice());

                self.add(elem2);
                self.add(elem3);
                self.add(elem4);
            }
            self.closed += 1;
        }
    }
}

pub fn main() {
    let mut clone = BinaryRelClone::new(4);
    clone.add_from_str("1100011000111001");
    clone.add_from_str("1000000000000000");
    clone.add_from_str("0000010000000000");
    clone.add_from_str("0000000000100000");
    clone.add_from_str("0000000000000001");
    clone.close();
    clone.print();
}
