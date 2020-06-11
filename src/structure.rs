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

/// Trait for structures implementing logical operations.
pub trait Logic: Eq {
    type Elem: Copy;

    /// Returns the logical negation of the given element.
    fn not(&self, elem: Self::Elem) -> Self::Elem;

    /// Returns the logical conjunction of the given elements.
    fn and(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        self.not(self.or(self.not(elem0), self.not(elem1)))
    }

    /// Returns the logical disjunction of the given elements.
    fn or(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem;

    /// Returns the logical implication of the given elements.
    fn imp(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        self.or(self.not(elem0), elem1)
    }

    /// Returns the logical exclusive or operation of the given elements.
    fn xor(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem;

    /// Returns the logical equivalence of the given elements.
    fn iff(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        self.xor(self.not(elem0), elem1)
    }
}

/// The trivial structure that has `()` as its sole element.
#[derive(PartialEq, Eq)]
pub struct Trivial;

pub const TRIVIAL: Trivial = Trivial;

impl Logic for Trivial {
    type Elem = ();

    #[inline(always)]
    fn not(&self, _elem: Self::Elem) -> Self::Elem {}

    #[inline(always)]
    fn and(&self, _elem1: Self::Elem, _elem2: Self::Elem) -> Self::Elem {}

    #[inline(always)]
    fn or(&self, _elem1: Self::Elem, _elem2: Self::Elem) -> Self::Elem {}

    #[inline(always)]
    fn imp(&self, _elem1: Self::Elem, _elem2: Self::Elem) -> Self::Elem {}

    #[inline(always)]
    fn xor(&self, _elem1: Self::Elem, _elem2: Self::Elem) -> Self::Elem {}

    #[inline(always)]
    fn iff(&self, _elem1: Self::Elem, _elem2: Self::Elem) -> Self::Elem {}
}

/// The two-element structure implementing the standard classical logic.
#[derive(PartialEq, Eq)]
pub struct Boolean;

pub const BOOLEAN: Boolean = Boolean;

impl Logic for Boolean {
    type Elem = bool;

    #[inline(always)]
    fn not(&self, elem: Self::Elem) -> Self::Elem {
        !elem
    }

    #[inline(always)]
    fn and(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        elem0 && elem1
    }

    #[inline(always)]
    fn or(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        elem0 || elem1
    }

    #[inline(always)]
    fn imp(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        elem0 <= elem1
    }

    #[inline(always)]
    fn xor(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        elem0 != elem1
    }

    #[inline(always)]
    fn iff(&self, elem0: Self::Elem, elem1: Self::Elem) -> Self::Elem {
        elem0 == elem1
    }
}

/// Trait for structures that implement equality over a set of elements.
pub trait Universe {
    /// The element type
    type Elem: Clone;

    /// The logic used within this universe.
    type Bool: Logic;

    /// Returns the underlying logic object.
    fn logic(&self) -> &Self::Bool;

    /// Returns the size of the universe if known.
    fn size(&self) -> Option<usize>;

    /// Checks if the given element is a member of this universe.
    fn contains(&self, elem: &Self::Elem) -> <Self::Bool as Logic>::Elem;

    /// Checks if the given elements are equal in this universe.
    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Bool as Logic>::Elem;
}

pub trait PartialOrder: Universe {
    /// Checks if the firt element is less than or equals to the second one.
    fn less_or_equals(&self, elem0: &Self::Elem, elem1: &Self::Elem)
        -> <Self::Bool as Logic>::Elem;

    fn less_than(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        self.logic().and(
            self.less_or_equals(elem0, elem1),
            self.logic().not(self.equals(elem0, elem1)),
        )
    }
}

pub trait Group: Universe {
    /// Returns the unit element of the group.
    fn unit(&self) -> Self::Elem;

    /// Returns the inverse of the given element.
    fn inverse(&self, elem: &Self::Elem) -> Self::Elem;

    /// Returns the product of the given elements.
    fn product(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem;
}

struct ModuloInt {
    pub size: u32,
}

impl Universe for ModuloInt {
    type Elem = u32;

    type Bool = Boolean;

    #[inline(always)]
    fn logic(&self) -> &Self::Bool {
        &BOOLEAN
    }

    fn size(&self) -> Option<usize> {
        Some(self.size as usize)
    }

    fn contains(&self, elem: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        assert!(*elem < self.size);
        *elem < self.size
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        assert!(*elem0 < self.size && *elem1 < self.size);
        *elem0 == *elem1
    }
}

impl PartialOrder for ModuloInt {
    fn less_or_equals(
        &self,
        elem0: &Self::Elem,
        elem1: &Self::Elem,
    ) -> <Self::Bool as Logic>::Elem {
        assert!(*elem0 < self.size && *elem1 < self.size);
        *elem0 <= *elem1
    }

    fn less_than(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        assert!(*elem0 < self.size && *elem1 < self.size);
        *elem0 < *elem1
    }
}

impl Group for ModuloInt {
    fn unit(&self) -> Self::Elem {
        assert!(self.size != 0);
        0
    }

    fn inverse(&self, elem: &Self::Elem) -> Self::Elem {
        assert!(*elem < self.size);
        if *elem == 0 {
            0
        } else {
            self.size - *elem
        }
    }

    fn product(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> Self::Elem {
        assert!(*elem0 < self.size && *elem1 < self.size);
        let elem3 = *elem0 + *elem1;
        if elem3 < self.size {
            elem3
        } else {
            elem3 - self.size
        }
    }
}

struct AntiChain {
    pub size: u32,
}

impl Universe for AntiChain {
    type Elem = u32;

    type Bool = Boolean;

    #[inline(always)]
    fn logic(&self) -> &Self::Bool {
        &BOOLEAN
    }

    fn size(&self) -> Option<usize> {
        Some(self.size as usize)
    }

    fn contains(&self, elem: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        assert!(*elem < self.size);
        *elem < self.size
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        assert!(*elem0 < self.size && *elem1 < self.size);
        *elem0 == *elem1
    }
}

impl PartialOrder for AntiChain {
    fn less_or_equals(
        &self,
        elem0: &Self::Elem,
        elem1: &Self::Elem,
    ) -> <Self::Bool as Logic>::Elem {
        assert!(*elem0 < self.size && *elem1 < self.size);
        *elem0 == *elem1
    }

    fn less_than(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        assert!(*elem0 < self.size && *elem1 < self.size);
        false
    }
}

pub struct Product<A, B>(A, B)
where
    A: Universe,
    B: Universe<Bool = A::Bool>;

impl<A, B> Product<A, B>
where
    A: Universe,
    B: Universe<Bool = A::Bool>,
{
    pub fn new(a: A, b: B) -> Self {
        assert!(std::ptr::eq(a.logic(), b.logic()));
        Product(a, b)
    }
}

impl<A, B> Universe for Product<A, B>
where
    A: Universe,
    B: Universe<Bool = A::Bool>,
{
    type Elem = (A::Elem, B::Elem);

    type Bool = A::Bool;

    fn logic(&self) -> &Self::Bool {
        self.0.logic()
    }

    fn size(&self) -> Option<usize> {
        if let (Some(size0), Some(size1)) = (self.0.size(), self.1.size()) {
            Some(size0 * size1)
        } else {
            None
        }
    }

    fn contains(&self, elem: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        self.logic()
            .and(self.0.contains(&elem.0), self.1.contains(&elem.1))
    }

    fn equals(&self, elem0: &Self::Elem, elem1: &Self::Elem) -> <Self::Bool as Logic>::Elem {
        self.logic().and(
            self.0.equals(&elem0.0, &elem1.0),
            self.1.equals(&elem0.1, &elem1.1),
        )
    }
}

impl<A, B> PartialOrder for Product<A, B>
where
    A: PartialOrder,
    B: PartialOrder<Bool = A::Bool>,
{
    fn less_or_equals(
        &self,
        elem0: &Self::Elem,
        elem1: &Self::Elem,
    ) -> <Self::Bool as Logic>::Elem {
        self.logic().and(
            self.0.less_or_equals(&elem0.0, &elem1.0),
            self.1.less_or_equals(&elem0.1, &elem1.1),
        )
    }
}
