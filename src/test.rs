/*
* Copyright (C) 2019, Miklos Maroti
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

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

trait VecData {
    type Data: Default;
}

impl VecData for bool {
    type Data = FixedBitSet;
}

impl VecData for u32 {
    type Data = Vec<u32>;
}

struct GenVec<T>(<T as VecData>::Data)
where
    T: VecData;

impl GenVec<bool> {
    pub fn new() -> Self {
        GenVec(FixedBitSet::with_capacity(0))
    }
}

impl GenVec<u32> {
    pub fn new() -> Self {
        GenVec(Vec::new())
    }
}
