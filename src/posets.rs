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

use super::boolalg::Boolean;
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

pub fn binrel_is_subset<A: TensorAlg>(
    alg: &mut A,
    binrel1: &A::Tensor,
    binrel2: &A::Tensor,
) -> A::Tensor {
    let result = alg.tensor_leq(binrel1, binrel2);
    let result = alg.tensor_all(&result);
    alg.tensor_all(&result)
}

pub fn binrel_is_reflexive<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let diagonal = binrel_diagonal_like(alg, binrel);
    binrel_is_subset(alg, &diagonal, binrel)
}

pub fn binrel_inverse<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let shape = binrel.shape();
    assert!(shape.len() >= 2 && shape[0] == shape[1]);

    let map = shape.mapping(&[1, 0], 2);
    alg.polymer(binrel, shape.clone(), &map)
}

pub fn binrel_is_symmetric<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let inverse = binrel_inverse(alg, binrel);
    binrel_is_subset(alg, &inverse, binrel)
}

pub fn binrel_symmetric_edges<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let result = binrel_inverse(alg, binrel);
    alg.tensor_and(&result, binrel)
}

pub fn binrel_is_antisymmetric<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let symmetric = binrel_symmetric_edges(alg, binrel);
    let diagonal = binrel_diagonal_like(alg, binrel);
    binrel_is_subset(alg, &symmetric, &diagonal)
}

pub fn binrel_compose<A: TensorAlg>(
    alg: &mut A,
    binrel1: &A::Tensor,
    binrel2: &A::Tensor,
) -> A::Tensor {
    let shape = binrel1.shape();
    assert!(shape == binrel2.shape());
    assert!(shape.len() >= 2 && shape[0] == shape[1]);

    let shape2 = shape.insert(2, &[shape[0]]);
    let map1 = shape.mapping(&[1, 0], 3);
    let map2 = shape.mapping(&[0, 2], 3);

    let binrel1 = alg.polymer(binrel1, shape2.clone(), &map1);
    let binrel2 = alg.polymer(binrel2, shape2, &map2);
    let result = alg.tensor_and(&binrel1, &binrel2);
    alg.tensor_any(&result)
}

pub fn binrel_is_transitive<A: TensorAlg>(alg: &mut A, binrel: &A::Tensor) -> A::Tensor {
    let composition = binrel_compose(alg, binrel, binrel);
    binrel_is_subset(alg, &composition, binrel)
}

pub fn create_x_obstruction<A: TensorAlg>(alg: &mut A, poset: &A::Tensor) -> A::Tensor {
    let shape = poset.shape();
    assert!(shape.len() >= 2 && shape[0] == shape[1]);

    let shape2 = shape.insert(2, &[shape[0], shape[0], shape[0]]);
    let rel1 = alg.polymer(poset, shape2.clone(), &shape.mapping(&[1, 0], 5));
    let rel2 = alg.polymer(poset, shape2.clone(), &shape.mapping(&[2, 0], 5));
    let rel3 = alg.polymer(poset, shape2.clone(), &shape.mapping(&[0, 3], 5));
    let rel4 = alg.polymer(poset, shape2, &shape.mapping(&[0, 4], 5));

    let result = alg.tensor_and(&rel1, &rel2);
    let result = alg.tensor_and(&result, &rel3);
    let result = alg.tensor_and(&result, &rel4);
    alg.tensor_any(&result)
}

pub fn x_obstruction_tables() -> (Vec<[usize; 2]>, Vec<[usize; 3]>, Vec<[usize; 4]>) {
    let mut alg = Boolean();
    let poset = create_hexagon_poset();
    let relation = create_x_obstruction(&mut alg, &poset);
    let mut table1 = Vec::new();
    let mut table2 = Vec::new();
    let mut table4 = Vec::new();

    for x in 0..6 {
        for y in 0..6 {
            for z in 0..6 {
                for u in 0..6 {
                    if relation.__slow_get__(&[x, y, z, u]) {
                        if table1.iter().all(|&e| e != [x, y]) {
                            table1.push([x, y]);
                        }
                        if table2.iter().all(|&e| e != [x, y, z]) {
                            table2.push([x, y, z]);
                        }
                        table4.push([x, y, z, u]);
                    }
                }
            }
        }
    }
    (table1, table2, table4)
}

pub fn calculate() {
    let (table1, table2, table4) = x_obstruction_tables();
    println!("{} {} {}", table1.len(), table2.len(), table4.len());
}

#[cfg(test)]
mod tests {
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
        let transitive = binrel_is_transitive(&mut alg, &poset);
        assert_eq!(transitive.__slow_get__(&[]), true);
    }
}
