/*
* Copyright (C) 2019-2020, Miklos Maroti
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

use super::{GenVec, VecFor};

#[test]
fn resize() {
    let mut v1: Vec<bool> = GenVec::new();
    let mut v2: VecFor<bool> = GenVec::new();
    let mut v3: VecFor<()> = GenVec::new();
    let mut v4: VecFor<bool> = GenVec::new();

    for i in 0..50 {
        let b = i % 2 == 0;

        for _ in 0..90 {
            v1.push(b);
            v3.push(());
            assert_eq!(v1.len(), v3.len());
        }
        v2.resize(v2.len() + 90, b);

        assert_eq!(v1.len(), v2.len());
        for j in 0..v1.len() {
            assert_eq!(v1.get(j), v2.get(j));
        }

        v4.clear();
        v4.extend(v1.clone());
        assert_eq!(v2, v4);

        v4.set(v4.len() - 1, !v4.get(v4.len() - 1));
        assert!(v2 != v4);
    }

    for _ in 0..50 {
        for _ in 0..77 {
            v1.pop();
        }
        v2.resize(v2.len() - 77, false);

        assert_eq!(v1.len(), v2.len());
        for j in 0..v1.len() {
            assert_eq!(v1.get(j), v2.get(j));
        }

        v4.clear();
        v4.extend(v1.clone());
        assert_eq!(v2, v4);

        v4.set(v4.len() - 1, !v4.get(v4.len() - 1));
        assert!(v2 != v4);
    }
}

#[test]
fn iters() {
    let e1 = vec![true, false, true];
    let e2 = e1.clone();
    let v1: VecFor<bool> = e1.into_iter().collect();
    let mut v2: VecFor<bool> = GenVec::new();
    for b in e2 {
        v2.push(b);
    }
    assert_eq!(v1, v2);

    let mut iter = v1.gen_iter().skip(1);
    assert_eq!(iter.next(), Some(false));
    assert_eq!(iter.next(), Some(true));
    assert_eq!(iter.next(), None);

    let e1 = [true, false];
    let v1: VecFor<bool> = e1.iter().copied().collect();
    let mut v2: VecFor<bool> = GenVec::new();
    for b in &e1 {
        v2.push(*b);
    }
    assert_eq!(v1, v2);

    v2.clear();
    assert_eq!(v2.len(), 0);
    for j in 0..100 {
        v2.push(j % 5 == 0 || j % 3 == 0);
    }
    assert_eq!(v2.len(), 100);
    for j in 0..100 {
        let b1 = unsafe { v2.get_unchecked(j) };
        let b2 = v2.get(j);
        let b3 = j % 5 == 0 || j % 3 == 0;
        assert_eq!(b1, b3);
        assert_eq!(b2, b3);

        let b4 = j % 7 == 0;
        unsafe { v2.set_unchecked(j, b4) };
        assert_eq!(v2.get(j), b4);
    }
}
