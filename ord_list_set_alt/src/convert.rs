// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use super::*;

use ord_set_iter_set_ops::{
    DifferenceIterator, IntersectionIterator, SymmetricDifferenceIterator, UnionIterator,
};

impl<T: Ord, const N: usize> From<[T; N]> for OrdListSet<T> {
    fn from(members: [T; N]) -> Self {
        let mut members = Vec::from(members);
        members.sort_unstable();
        members.dedup();
        Self { members: members.into_boxed_slice() }
    }
}

impl<T: Ord + Clone> From<&[T]> for OrdListSet<T> {
    fn from(members: &[T]) -> Self {
        let mut members = Vec::from(members);
        members.sort_unstable();
        members.dedup();
        Self { members: members.into_boxed_slice() }
    }
}

impl<T: Ord> From<BTreeSet<T>> for OrdListSet<T> {
    fn from(mut set: BTreeSet<T>) -> Self {
        let mut members: Vec<T> = Vec::with_capacity(set.len());
        while let Some(member) = set.pop_first() {
            members.push(member);
        }
        Self { members: members.into_boxed_slice() }
    }
}

#[allow(clippy::from_over_into)] // NB: we can't do from on an imported struct
impl<T: Ord + Clone> Into<BTreeSet<T>> for OrdListSet<T> {
    fn into(self) -> BTreeSet<T> {
        BTreeSet::<T>::from_iter(self.iter().cloned())
    }
}

impl<'a, T: Ord + Clone> From<DifferenceIterator<'a, T>> for OrdListSet<T> {
    fn from(oso_iter: DifferenceIterator<'a, T>) -> Self {
        let members: Vec<T> = oso_iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<IntersectionIterator<'a, T>> for OrdListSet<T> {
    fn from(oso_iter: IntersectionIterator<'a, T>) -> Self {
        let  members: Vec<T> = oso_iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<SymmetricDifferenceIterator<'a, T>> for OrdListSet<T> {
    fn from(oso_iter: SymmetricDifferenceIterator<'a, T>) -> Self {
        let members: Vec<T> = oso_iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<UnionIterator<'a, T>> for OrdListSet<T> {
    fn from(oso_iter: UnionIterator<'a, T>) -> Self {
        let members: Vec<T> = oso_iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<OrdListSetIter<'a, T>> for OrdListSet<T> {
    fn from(iter: OrdListSetIter<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<Union<'a, T>> for OrdListSet<T> {
    fn from(iter: Union<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<Intersection<'a, T>> for OrdListSet<T> {
    fn from(iter: Intersection<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<Difference<'a, T>> for OrdListSet<T> {
    fn from(iter: Difference<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<'a, T: Ord + Clone> From<SymmetricDifference<'a, T>> for OrdListSet<T> {
    fn from(iter: SymmetricDifference<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members: members.into_boxed_slice() }
    }
}

impl<T: Ord> FromIterator<T> for OrdListSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut members: Vec<T> = iter.into_iter().collect();
        members.sort_unstable();
        members.dedup();
        Self { members: members.into_boxed_slice() }
    }
}
