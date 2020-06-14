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

use crate::relation::{BinaryRelAlg, Universe};
use crate::tensor::{Boolean, Shape, Solver, Tensor, TensorAlg};

pub fn crown(size: usize) -> Tensor<bool> {
    assert!(size >= 4 && size % 2 == 0);
    Tensor::create(Shape::new(vec![size, size]), |i| {
        if i[0] % 2 == 1 {
            i[0] == i[1]
        } else if i[0] == 0 {
            i[1] <= 1 || i[1] == size - 1
        } else {
            i[1] >= i[0] - 1 && i[1] <= i[0] + 1
        }
    })
}

/// Takes an tensor of shape [n,m,...], returns a tensor of shape [...] and
/// checks if the [n,m] tensor is a mapping from an m-element set to an
/// n-element set.
pub fn is_function<ALG: TensorAlg>(alg: &mut ALG, f: ALG::Elem) -> ALG::Elem {
    assert_eq!(ALG::shape(&f).len(), 2);
    let f = alg.tensor_one(f);
    alg.tensor_all(f)
}

/// Takes a tensor of shape [n,n,...] and returns a tensor of shape [...].
pub fn is_reflexive<ALG: TensorAlg>(alg: &mut ALG, rel: ALG::Elem) -> ALG::Elem {
    let (n, shape) = ALG::shape(&rel).split();
    assert_eq!(n, shape[0]);
    let mapping: Vec<usize> = std::iter::once(0).chain(0..shape.len()).collect();
    let rel = alg.tensor_polymer(rel, shape, &mapping);
    alg.tensor_all(rel)
}

pub fn test() {
    let crown4 = crown(4);
    println!("{:?}", crown4);
    let mut alg = Boolean();
    assert!(is_reflexive(&mut alg, crown4.clone()).scalar());

    let mut univ4 = Universe::new(Boolean(), 4);
    assert!(univ4.is_binary_rel(&crown4));
    let diag = univ4.binrel_diag();
    assert!(univ4.binrel_join(diag, crown4.clone()) == crown4);
    assert!(univ4.binrel_circ(crown4.clone(), crown4.clone()) == crown4);

    let _sat = Solver::new("batsat");
}
