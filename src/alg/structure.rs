/*
* Copyright (C) 2024, Miklos Maroti
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

use super::{BooleanLogic, Domain, Indexable, Product2, Relations, Vector};

#[derive(Clone, Debug)]
struct Relation<LOGIC>
where
    LOGIC: BooleanLogic,
{
    name: &'static str,
    arity: usize,
    value: LOGIC::Vector,
}

#[derive(Debug)]
pub struct Structure<LOGIC, DOM>
where
    LOGIC: BooleanLogic,
    DOM: Indexable,
{
    domain: DOM,
    relations: Vec<Relation<LOGIC>>,
}

impl<LOGIC, DOM> Structure<LOGIC, DOM>
where
    LOGIC: BooleanLogic,
    DOM: Indexable,
{
    /// Creates the product domain from the given list of domains.
    pub fn new(domain: DOM) -> Self {
        Self {
            domain,
            relations: Default::default(),
        }
    }

    pub fn domain(&self) -> &DOM {
        &self.domain
    }

    pub fn add(&mut self, name: &'static str, arity: usize, value: LOGIC::Vector) {
        assert!(self.relations.iter().all(|r| r.name != name));
        let dom = Relations::new(self.domain.clone(), arity);
        assert_eq!(dom.num_bits(), value.len());
        self.relations.push(Relation { name, arity, value });
    }

    pub fn get(&self, name: &'static str) -> &LOGIC::Vector {
        &self
            .relations
            .iter()
            .find(|r| r.name == name)
            .unwrap()
            .value
    }

    pub fn product<DOM0, DOM1>(
        str0: Structure<LOGIC, DOM0>,
        str1: Structure<LOGIC, DOM1>,
    ) -> Structure<LOGIC, Product2<DOM0, DOM1>>
    where
        DOM0: Indexable,
        DOM1: Indexable,
    {
        let domain = Product2::new(str0.domain().clone(), str1.domain().clone());

        assert_eq!(str0.relations.len(), str1.relations.len());
        let relations = str0
            .relations
            .iter()
            .zip(str1.relations.iter())
            .map(|(a, b)| {
                let name = a.name;
                assert_eq!(name, b.name);
                let arity = a.arity;
                assert_eq!(arity, b.arity);

                let value = a.value.clone();
                Relation { name, arity, value }
            })
            .collect();

        Structure { domain, relations }
    }
}
