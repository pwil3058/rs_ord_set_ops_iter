// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
use dyn_clonable::*;

use std::cmp::Ordering;
use std::collections::{btree_map, btree_set, BTreeMap, BTreeSet};
use std::iter::Peekable;

pub mod difference_iterator;
pub mod intersection_iterator;
pub mod set_relationships;
pub mod symmetric_difference_iterator;
pub mod union_iterator;

pub use difference_iterator::*;
pub use intersection_iterator::*;
pub use symmetric_difference_iterator::*;
pub use union_iterator::*;

/// Ordered Iterator over set operations on the contents of an ordered set.
#[clonable]
pub trait PeepAdvanceIter<'a, T: 'a + Ord>: Iterator<Item = &'a T> + 'a + Clone {
    /// Peep at the next item in the iterator without advancing the iterator.
    fn peep(&mut self) -> Option<&'a T>;

    /// Will the next next() call return None? I.e. is the iterator exhausted?
    #[allow(clippy::wrong_self_convention)]
    fn is_empty(&mut self) -> bool {
        self.peep().is_none()
    }

    /// Advance this iterator to the next item at or after the given item.
    /// Default implementation is O(n) but custom built implementations could be as good as O(log(n)).
    // TODO: try to make advance_until() return &mut Self
    fn advance_until(&mut self, target: &T) {
        while let Some(item) = self.peep() {
            if target > item {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Advance this iterator to the next item after the given item.
    /// Default implementation is O(n) but custom built implementations could be as good as O(log(n)).
    // TODO: try to make advance_until() return &mut Self
    fn advance_after(&mut self, target: &T) {
        while let Some(item) = self.peep() {
            if target >= item {
                self.next();
            } else {
                break;
            }
        }
    }
}

pub trait OrdSetIterSetOpsIterator<'a, T: 'a + Ord + Clone>:
    PeepAdvanceIter<'a, T> + Sized + Clone
{
    #[allow(clippy::wrong_self_convention)]
    fn is_disjoint(mut self, mut other: impl PeepAdvanceIter<'a, T>) -> bool {
        are_disjoint!(self, other)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_subset(mut self, mut other: impl PeepAdvanceIter<'a, T>) -> bool {
        left_is_subset_of_right!(self, other)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_proper_subset(mut self, mut other: impl PeepAdvanceIter<'a, T>) -> bool {
        left_is_proper_subset_of_right!(self, other)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_superset(mut self, mut other: impl PeepAdvanceIter<'a, T>) -> bool {
        left_is_superset_of_right!(self, other)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_proper_superset(mut self, mut other: impl PeepAdvanceIter<'a, T>) -> bool {
        left_is_proper_superset_of_right!(self, other)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_equal(mut self, mut other: impl PeepAdvanceIter<'a, T>) -> bool {
        left_is_equal_to_right!(self, other)
    }

    fn compare(mut self, mut other: impl PeepAdvanceIter<'a, T>) -> Ordering {
        left_cmp_right!(self, other)
    }

    fn difference(
        self,
        other: impl PeepAdvanceIter<'a, T, Item = &'a T>,
    ) -> DifferenceIterator<'a, T> {
        DifferenceIterator::new(self, other)
    }

    fn intersection(
        self,
        other: impl PeepAdvanceIter<'a, T, Item = &'a T>,
    ) -> IntersectionIterator<'a, T> {
        IntersectionIterator::new(self, other)
    }

    fn symmetric_difference(
        self,
        other: impl PeepAdvanceIter<'a, T, Item = &'a T>,
    ) -> SymmetricDifferenceIterator<'a, T> {
        SymmetricDifferenceIterator::new(self, other)
    }

    fn union(self, other: impl PeepAdvanceIter<'a, T, Item = &'a T>) -> UnionIterator<'a, T> {
        UnionIterator::new(self, other)
    }
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Iter<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T> for Peekable<btree_set::Iter<'a, T>> {}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Intersection<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::Intersection<'a, T>>
{
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Difference<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::Difference<'a, T>>
{
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::SymmetricDifference<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::SymmetricDifference<'a, T>>
{
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Union<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::Union<'a, T>>
{
}

pub trait BTreeSetAdaptor<'a, T: 'a + Ord>
where
    T: 'a + Ord + Clone,
{
    fn oso_iter(&'a self) -> Peekable<btree_set::Iter<'a, T>>;

    fn oso_difference(&'a self, other: &'a Self) -> Peekable<btree_set::Difference<'a, T>>;

    fn oso_intersection(&'a self, other: &'a Self) -> Peekable<btree_set::Intersection<'a, T>>;

    fn oso_symmetric_difference(
        &'a self,
        other: &'a Self,
    ) -> Peekable<btree_set::SymmetricDifference<'a, T>>;

    fn oso_union(&'a self, other: &'a Self) -> Peekable<btree_set::Union<'a, T>>;
}

impl<'a, T: 'a + Ord + Clone> BTreeSetAdaptor<'a, T> for BTreeSet<T> {
    fn oso_iter(&'a self) -> Peekable<btree_set::Iter<'a, T>> {
        self.iter().peekable()
    }

    fn oso_difference(&'a self, other: &'a Self) -> Peekable<btree_set::Difference<'a, T>> {
        self.difference(other).peekable()
    }

    fn oso_intersection(&'a self, other: &'a Self) -> Peekable<btree_set::Intersection<'a, T>> {
        self.intersection(other).peekable()
    }

    fn oso_symmetric_difference(
        &'a self,
        other: &'a Self,
    ) -> Peekable<btree_set::SymmetricDifference<'a, T>> {
        self.symmetric_difference(other).peekable()
    }

    fn oso_union(&'a self, other: &'a Self) -> Peekable<btree_set::Union<'a, T>> {
        self.union(other).peekable()
    }
}

impl<'a, K: 'a + Ord, V> PeepAdvanceIter<'a, K> for Peekable<btree_map::Keys<'a, K, V>> {
    fn peep(&mut self) -> Option<&'a K> {
        self.peek().copied()
    }
}

impl<'a, K: 'a + Ord + Clone, V> OrdSetIterSetOpsIterator<'a, K>
    for Peekable<btree_map::Keys<'a, K, V>>
{
}
pub trait BTreeMapAdaptor<'a, K: 'a + Ord, V>
where
    K: 'a + Ord + Clone,
{
    fn oso_keys(&'a self) -> Peekable<btree_map::Keys<'a, K, V>>;
}

impl<'a, K: 'a + Ord + Clone, V> BTreeMapAdaptor<'a, K, V> for BTreeMap<K, V> {
    fn oso_keys(&'a self) -> Peekable<btree_map::Keys<'a, K, V>> {
        self.keys().peekable()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn difference() {
        let set1 = BTreeSet::from(["a", "b", "c", "d", "e", "f"]);
        let set2 = BTreeSet::from(["b", "c", "e", "g"]);
        assert_eq!(
            set1.difference(&set2).collect::<Vec<_>>(),
            set1.iter()
                .peekable()
                .difference(set2.iter().peekable())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn intersection() {
        let set1 = BTreeSet::from(["a", "b", "c", "d", "e", "f"]);
        let set2 = BTreeSet::from(["b", "c", "e", "g"]);
        assert_eq!(
            set1.intersection(&set2).collect::<Vec<_>>(),
            set1.iter()
                .peekable()
                .intersection(set2.iter().peekable())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            set1.iter().peekable().is_disjoint(set2.iter().peekable()),
            set1.intersection(&set2).collect::<Vec<_>>().is_empty()
        );
    }

    #[test]
    fn symmetric_difference() {
        let set1 = BTreeSet::from(["a", "b", "c", "d", "e", "f"]);
        let set2 = BTreeSet::from(["b", "c", "e", "g"]);
        assert_eq!(
            set1.symmetric_difference(&set2).collect::<Vec<_>>(),
            set1.iter()
                .peekable()
                .symmetric_difference(set2.iter().peekable())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn union() {
        let set1 = BTreeSet::from(["a", "b", "c", "d", "e", "f"]);
        let set2 = BTreeSet::from(["b", "c", "e", "g"]);
        assert_eq!(
            set1.union(&set2).collect::<Vec<_>>(),
            set1.iter()
                .peekable()
                .union(set2.iter().peekable())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn expression() {
        let set1 = BTreeSet::from(["a", "b", "c", "d", "e", "f"]);
        let set2 = BTreeSet::from(["b", "c", "e", "g"]);
        let set3 = BTreeSet::from(["e", "f", "g"]);
        let set4 = BTreeSet::from(["b", "d", "h", "i", "j"]);

        assert_eq!(
            &(&(&set1 | &set3) - &(&set2 & &set4)) ^ &set3,
            BTreeSet::from_iter(
                set1.iter()
                    .peekable()
                    .union(set3.iter().peekable())
                    .difference(
                        set2.iter()
                            .peekable()
                            .intersection(set4.iter().peekable())
                            .symmetric_difference(set3.iter().peekable()),
                    )
                    .cloned(),
            )
        );
    }

    #[test]
    fn map() {
        let set1 = BTreeSet::from(["a", "b", "c", "d", "e", "f"]);
        let set2 = BTreeSet::from(["b", "d", "h", "i", "j"]);
        let map = BTreeMap::from([("b", 1), ("c", 3), ("g", 5), ("i", 6)]);
        assert_eq!(
            map.keys()
                .peekable()
                .intersection(set1.iter().peekable())
                .cloned()
                .collect::<Vec<_>>(),
            vec!["b", "c"]
        );
        assert_eq!(
            set2.iter()
                .peekable()
                .difference(map.keys().peekable())
                .cloned()
                .collect::<Vec<_>>(),
            vec!["d", "h", "j"]
        );
    }
}
