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

pub trait MyVec<ELEM>
where
    ELEM: Copy,
{
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> ELEM;
}

pub struct MyIter<VEC> {
    pos: usize,
    vec: VEC,
}

impl<VEC> Iterator for MyIter<VEC>
where
    VEC: MyVec<u32>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.vec.len() {
            let a = self.vec.get(self.pos);
            self.pos += 1;
            Some(a)
        } else {
            None
        }
    }
}

pub trait GenVec1<ELEM>
where
    ELEM: Copy,
    Self: Default + Clone,
    Self: IntoIterator<Item = ELEM> + std::iter::FromIterator<ELEM>,
{
}

pub trait GenVec2
where
    Self: Default + Clone + IntoIterator,
    Self: std::iter::FromIterator<<Self as IntoIterator>::Item>,
{
    fn get(&self, index: usize) -> <Self as IntoIterator>::Item;
}

pub trait HasElem {
    type Elem;
}

pub trait GenVec3
where
    Self: Default + Clone + HasElem,
    Self: IntoIterator<Item = <Self as HasElem>::Elem>,
    Self: std::iter::FromIterator<<Self as HasElem>::Elem>,
{
    fn get(&self, index: usize) -> <Self as HasElem>::Elem;
}
