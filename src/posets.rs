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

use super::tensor::{Shape, Tensor, TensorAlg, TensorOps};

pub fn create_hexagon_poset() -> Tensor<bool> {
    let shape = Shape::new(&[6, 6]);
    let mut poset = Tensor::new(shape, false);

    poset.__slow_set__(&[0, 0], true);
    poset.__slow_set__(&[0, 1], true);
    poset.__slow_set__(&[0, 2], true);
    poset.__slow_set__(&[0, 3], true);
    poset.__slow_set__(&[0, 4], true);
    poset.__slow_set__(&[0, 5], true);
    poset.__slow_set__(&[1, 1], true);
    poset.__slow_set__(&[1, 3], true);
    poset.__slow_set__(&[1, 4], true);
    poset.__slow_set__(&[1, 5], true);
    poset.__slow_set__(&[2, 2], true);
    poset.__slow_set__(&[2, 3], true);
    poset.__slow_set__(&[2, 4], true);
    poset.__slow_set__(&[2, 5], true);
    poset.__slow_set__(&[3, 3], true);
    poset.__slow_set__(&[3, 5], true);
    poset.__slow_set__(&[4, 4], true);
    poset.__slow_set__(&[4, 5], true);
    poset.__slow_set__(&[5, 5], true);

    poset
}

pub fn binrel_diagonal_like<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let shape = binrel.shape();
    assert!(shape.len() >= 2 && shape[0] == shape[1]);

    let diagonal = alg.diagonal(shape[0]);
    alg.polymer(&diagonal, shape.clone(), &[0, 1])
}

pub fn binrel_is_reflexive<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let result = binrel_diagonal_like(alg, binrel);
    let result = alg.tensor_leq(&result, binrel);
    let result = alg.tensor_all(&result);
    alg.tensor_all(&result)
}

pub fn binrel_inverse<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let shape = binrel.shape();
    assert!(shape.len() >= 2 && shape[0] == shape[1]);

    let mut map = Vec::with_capacity(shape.len());
    map.push(1);
    map.push(0);
    for i in 2..shape.len() {
        map.push(i)
    }

    alg.polymer(binrel, shape.clone(), &map)
}

pub fn binrel_is_symmetric<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let result = binrel_inverse(alg, binrel);
    let result = alg.tensor_leq(&result, binrel);
    let result = alg.tensor_all(&result);
    alg.tensor_all(&result)
}

pub fn binrel_is_antisymmetric<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let result = binrel_inverse(alg, binrel);
    let result = alg.tensor_and(&result, binrel);
    let diagonal = binrel_diagonal_like(alg, binrel);
    let result = alg.tensor_leq(&result, &diagonal);
    let result = alg.tensor_all(&result);
    alg.tensor_all(&result)
}

#[cfg(test)]
mod tests {
    use super::super::boolalg::Boolean;
    use super::*;

    #[test]
    fn properties() {
        let mut alg = Boolean();
        let poset = create_hexagon_poset();
        let reflexive = binrel_is_reflexive(&mut alg, &poset);
        assert_eq!(reflexive.__slow_get__(&[]), true);
        let symmetric = binrel_is_symmetric(&mut alg, &poset);
        assert_eq!(symmetric.__slow_get__(&[]), false);
        let antisymmetric = binrel_is_antisymmetric(&mut alg, &poset);
        assert_eq!(antisymmetric.__slow_get__(&[]), true);
    }
}
