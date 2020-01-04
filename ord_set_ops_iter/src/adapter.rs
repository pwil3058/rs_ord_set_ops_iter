// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

pub use std::{
    collections::btree_set,
    iter::Peekable,
    ops::{BitAnd, BitOr, BitXor, Sub},
};

use crate::adapter::btree_set::BTreeSet;
pub use crate::{OrdSetOpsIter, OrdSetOpsIterator};

pub trait OrdSetOpsIterAdaptation: Iterator + Sized {
    fn ord_set_ops(self) -> OrdSetOpsIterAdapter<Self> {
        OrdSetOpsIterAdapter::new(self)
    }
}

impl<'a, T: Ord> OrdSetOpsIterAdaptation for btree_set::Iter<'a, T> {}
impl<'a, T: Ord> OrdSetOpsIterAdaptation for btree_set::Difference<'a, T> {}
impl<'a, T: Ord> OrdSetOpsIterAdaptation for btree_set::Intersection<'a, T> {}
impl<'a, T: Ord> OrdSetOpsIterAdaptation for btree_set::SymmetricDifference<'a, T> {}
impl<'a, T: Ord> OrdSetOpsIterAdaptation for btree_set::Union<'a, T> {}

pub struct OrdSetOpsIterAdapter<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator> OrdSetOpsIterAdapter<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator> Iterator for OrdSetOpsIterAdapter<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.iter.next()
    }
}

impl<'a, T, I> OrdSetOpsIterator<'a, T> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
{
    #[inline]
    fn peek(&mut self) -> Option<&'a T> {
        if let Some(item) = self.iter.peek() {
            Some(*item)
        } else {
            None
        }
    }
}

impl<'a, T, I, O> BitAnd<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitand(self, other: O) -> Self::Output {
        self.intersection(other)
    }
}

impl<'a, T, I, O> BitOr<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitor(self, other: O) -> Self::Output {
        self.union(other)
    }
}

impl<'a, T, I, O> BitXor<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitxor(self, other: O) -> Self::Output {
        self.symmetric_difference(other)
    }
}

impl<'a, T, I, O> Sub<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn sub(self, other: O) -> Self::Output {
        self.difference(other)
    }
}

pub trait OrdSetOpsSetAdaption<'a, T: 'a + Ord, I>
where
    T: 'a + Ord,
    I: Iterator<Item = &'a T>,
{
    fn oso_iter(&'a self) -> OrdSetOpsIterAdapter<I>;

    fn oso_difference(
        &'a self,
        other: &'a Self,
    ) -> OrdSetOpsIter<'a, T, OrdSetOpsIterAdapter<I>, OrdSetOpsIterAdapter<I>> {
        self.oso_iter().difference(other.oso_iter())
    }

    fn oso_intersection(
        &'a self,
        other: &'a Self,
    ) -> OrdSetOpsIter<'a, T, OrdSetOpsIterAdapter<I>, OrdSetOpsIterAdapter<I>> {
        self.oso_iter().intersection(other.oso_iter())
    }

    fn oso_symmetric_difference(
        &'a self,
        other: &'a Self,
    ) -> OrdSetOpsIter<'a, T, OrdSetOpsIterAdapter<I>, OrdSetOpsIterAdapter<I>> {
        self.oso_iter().symmetric_difference(other.oso_iter())
    }

    fn oso_union(
        &'a self,
        other: &'a Self,
    ) -> OrdSetOpsIter<'a, T, OrdSetOpsIterAdapter<I>, OrdSetOpsIterAdapter<I>> {
        self.oso_iter().union(other.oso_iter())
    }
}

impl<'a, T: 'a + Ord> OrdSetOpsSetAdaption<'a, T, btree_set::Iter<'a, T>> for BTreeSet<T> {
    fn oso_iter(&'a self) -> OrdSetOpsIterAdapter<btree_set::Iter<'a, T>> {
        self.iter().ord_set_ops()
    }
}

#[cfg(test)]
mod tests {
    use super::OrdSetOpsIterAdapter;
    use crate::OrdSetOpsIterator;

    #[test]
    fn set_relations() {
        let iter1 = OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter());
        let iter2 = OrdSetOpsIterAdapter::new(["b", "c", "d"].iter());
        assert!(iter1.is_superset(iter2));
        let iter1 = OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter());
        let iter2 = OrdSetOpsIterAdapter::new(["b", "c", "d"].iter());
        assert!(!iter1.is_subset(iter2));
    }

    #[test]
    fn set_difference() {
        assert_eq!(
            vec!["a"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .difference(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                - OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .difference(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_intersection() {
        assert_eq!(
            vec!["b", "c", "d"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .intersection(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["b", "c", "d"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                & OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![1, 2, 3],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .intersection(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_symmetric_difference() {
        assert_eq!(
            vec!["a", "e"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .symmetric_difference(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a", "e"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                ^ OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0, 4, 5],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .symmetric_difference(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_union() {
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .union(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                | OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .union(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }
}

#[cfg(test)]
mod b_tree_set_tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn iterator_adapter() {
        let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
        let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();
        let set3: BTreeSet<&str> = ["c", "e", "i"].iter().cloned().collect();
        assert_eq!(
            vec!["c", "e"],
            (set1.iter().ord_set_ops() & set3.iter().ord_set_ops())
                .cloned()
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["c", "i"],
            (set2.iter().ord_set_ops() & set3.iter().ord_set_ops())
                .cloned()
                .collect::<Vec<&str>>()
        );
        let result = &(&set1 | &set2) & &set3;
        let set4: BTreeSet<&str> = (set1.iter().ord_set_ops() | set2.iter().ord_set_ops())
            .cloned()
            .collect();
        assert_eq!(
            result,
            (set4.iter().ord_set_ops() & set3.iter().ord_set_ops())
                .cloned()
                .collect()
        );
        let iter = set1.iter().ord_set_ops() | set2.iter().ord_set_ops();
        assert_eq!(
            result,
            (iter & set3.iter().ord_set_ops()).cloned().collect()
        );
        assert_eq!(
            result,
            ((set1.iter().ord_set_ops() | set2.iter().ord_set_ops()) & set3.iter().ord_set_ops())
                .cloned()
                .collect()
        );
        assert_eq!(
            result,
            (set1.union(&set2).ord_set_ops() & set3.iter().ord_set_ops())
                .cloned()
                .collect()
        );
    }

    #[test]
    fn set_adapter() {
        let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
        let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();
        let set3: BTreeSet<&str> = ["c", "e", "i"].iter().cloned().collect();
        let result = &(&set1 | &set2) & &set3;
        assert_eq!(
            result,
            ((set1.oso_iter() | set2.oso_iter()) & set3.oso_iter())
                .cloned()
                .collect()
        );
        assert_eq!(
            result,
            (set1.oso_union(&set2) & set3.oso_iter()).cloned().collect()
        );
    }
}
