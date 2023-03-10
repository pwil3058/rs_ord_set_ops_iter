// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
use dyn_clonable::*;

use std::cmp::Ordering;
use std::collections::{btree_map, btree_set};
use std::iter::Peekable;

pub mod difference_iterator;
pub mod intersection_iterator;
pub mod set_relationships;
pub mod symmetric_difference_iterator;
pub mod union_iterator;

use difference_iterator::*;
use intersection_iterator::*;
use symmetric_difference_iterator::*;
use union_iterator::*;

/// Ordered Iterator over set operations on the contents of an ordered set.
#[clonable]
pub trait PeepAdvanceIter<'a, T: 'a + Ord>: Iterator<Item = &'a T> + 'a + Clone {
    /// Peep at the next item in the iterator without advancing the iterator.
    fn peep(&mut self) -> Option<&'a T>;

    /// Will the next next() call return None? I.e. is the iterator exhausted?
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

//#[clonable]
pub trait OrdSetIterSetOpsIterator<'a, T: 'a + Ord + Clone + std::default::Default>:
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

    fn difference(
        self,
        other: impl PeepAdvanceIter<'a, T, Item = &'a T>,
    ) -> Box<dyn PeepAdvanceIter<'a, T, Item = &'a T> + 'a> {
        Box::new(DifferenceIterator::new(self, other))
    }

    fn intersection(
        self,
        other: impl PeepAdvanceIter<'a, T, Item = &'a T>,
    ) -> Box<dyn PeepAdvanceIter<'a, T, Item = &'a T> + 'a> {
        Box::new(IntersectionIterator::new(self, other))
    }

    fn symmetric_difference(
        self,
        other: impl PeepAdvanceIter<'a, T, Item = &'a T>,
    ) -> Box<dyn PeepAdvanceIter<'a, T, Item = &'a T> + 'a> {
        Box::new(SymmetricDifferenceIterator::new(self, other))
    }

    fn union(
        self,
        other: impl PeepAdvanceIter<'a, T, Item = &'a T>,
    ) -> Box<dyn PeepAdvanceIter<'a, T, Item = &'a T> + 'a> {
        Box::new(UnionIterator::new(self, other))
    }
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Iter<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone + Default> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::Iter<'a, T>>
{
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Intersection<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone + Default> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::Intersection<'a, T>>
{
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Difference<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone + Default> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::Difference<'a, T>>
{
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::SymmetricDifference<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone + Default> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::SymmetricDifference<'a, T>>
{
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for Peekable<btree_set::Union<'a, T>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone + Default> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_set::Union<'a, T>>
{
}

impl<'a, T: 'a + Ord, V> PeepAdvanceIter<'a, T> for Peekable<btree_map::Keys<'a, T, V>> {
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

impl<'a, T: 'a + Ord + Clone + Default, V> OrdSetIterSetOpsIterator<'a, T>
    for Peekable<btree_map::Keys<'a, T, V>>
{
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeSet;

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

    // #[test]
    // fn expression() {
    //     let set1 = BTreeSet::from(["a", "b", "c", "d", "e", "f"]);
    //     let set2 = BTreeSet::from(["b", "c", "e", "g"]);
    //     let set3 = BTreeSet::from(["e", "f", "g"]);
    //
    //     let op_result = &(&set1 | &set3) - &set2;
    //     let oso_result = BTreeSet::from_iter(
    //         set1.iter()
    //             .peekable()
    //             .union(set3.iter().peekable())
    //             .difference(set2.iter().peekable()),
    //     );
    //     assert_eq!(op_result, oso_result);
    // }
}
