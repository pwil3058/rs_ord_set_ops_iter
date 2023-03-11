// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
//! Sets implemented as an immutable sorted list.

use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::Debug,
    iter::FromIterator,
    ops::{BitAnd, BitOr, BitXor, Bound, RangeBounds, Sub},
};

use ord_set_iter_set_ops::{
    difference_next, difference_peep, intersection_next, intersection_peep,
    symmetric_difference_next, symmetric_difference_peep, union_next, union_peep,
    OrdSetIterSetOpsIterator, PeepAdvanceIter,
};

/// A set of items of type T ordered according to Ord (with no duplicates)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrdListSet<T: Ord> {
    members: Vec<T>,
}

impl<T: Ord> Default for OrdListSet<T> {
    fn default() -> Self {
        Self {
            members: Vec::new(),
        }
    }
}

impl<T: Ord> OrdListSet<T> {
    pub fn empty_set() -> Self {
        Self::default()
    }
}

impl<T: Ord> OrdListSet<T> {
    /// Return number of members in this set.
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Return `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Return an iterator over the members in the `OrdListSet` in ascending order.
    pub fn iter(&self) -> OrdListSetIter<T> {
        OrdListSetIter {
            elements: &self.members,
            index: 0,
        }
    }
}

// impl<'a, T: 'a + Ord + Clone> SetOsoIter<'a, T> for OrdListSet<T> {
//     /// Return a set operations iterator over the members in the `OrdListSet` in ascending order.
//     fn oso_iter(&'a self) -> OrdSetOpsIter<'a, T> {
//         OrdSetOpsIter::new(OrdListSetIter {
//             elements: &self.members,
//             index: 0,
//         })
//     }
// }

enum UsizeRangeBounds {
    Range(usize, usize),
    RangeFrom(usize),
    RangeFull,
    RangeInclusive(usize, usize),
    RangeTo(usize),
    RangeToInclusive(usize),
}

impl UsizeRangeBounds {
    fn for_range_bounds(range_bounds: impl RangeBounds<usize>) -> UsizeRangeBounds {
        use UsizeRangeBounds::*;
        match range_bounds.start_bound() {
            Bound::Included(start) => match range_bounds.end_bound() {
                Bound::Included(end) => RangeInclusive(*start, *end),
                Bound::Excluded(end) => Range(*start, *end),
                Bound::Unbounded => RangeFrom(*start),
            },
            // shouldn't happen as there's no way to express it
            Bound::Excluded(start) => match range_bounds.end_bound() {
                Bound::Included(end) => RangeInclusive(*start, *end),
                Bound::Excluded(end) => Range(*start, *end),
                Bound::Unbounded => RangeFrom(*start),
            },
            Bound::Unbounded => match range_bounds.end_bound() {
                Bound::Included(end) => RangeToInclusive(*end),
                Bound::Excluded(end) => RangeTo(*end),
                Bound::Unbounded => RangeFull,
            },
        }
    }
}

// set functions that don't modify the set
impl<'a, T: 'a + Ord + Clone> OrdListSet<T> {
    ///Returns true if the set contains an element equal to the value.
    pub fn contains(&self, item: &T) -> bool {
        self.members.binary_search(item).is_ok()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.members.get(index)
    }

    fn items_private(&self, usize_range_bounds: &UsizeRangeBounds) -> &[T] {
        use UsizeRangeBounds::*;
        if let Some(items) = match usize_range_bounds {
            Range(start, end) => self.members.get(*start..*end),
            RangeFrom(start) => self.members.get(*start..),
            RangeFull => self.members.get(..),
            RangeInclusive(start, end) => self.members.get(*start..=*end),
            RangeTo(end) => self.members.get(..*end),
            RangeToInclusive(end) => self.members.get(..=*end),
        } {
            items
        } else {
            &[]
        }
    }

    fn start_bound_for(&self, bound: &Bound<&'a T>) -> Bound<usize> {
        match bound {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(target) => match self.members.binary_search(target) {
                Ok(index) => Bound::Included(index),
                Err(index) => Bound::Included(index),
            },
            Bound::Excluded(target) => match self.members.binary_search(target) {
                Ok(index) => Bound::Excluded(index),
                Err(index) => Bound::Included(index),
            },
        }
    }

    fn end_bound_for(&self, bound: &Bound<&'a T>) -> Bound<usize> {
        match bound {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(start) => match self.members.binary_search(start) {
                Ok(index) => Bound::Included(index),
                Err(index) => Bound::Excluded(index),
            },
            Bound::Excluded(start) => match self.members.binary_search(start) {
                Ok(index) => Bound::Excluded(index),
                Err(index) => Bound::Excluded(index),
            },
        }
    }

    fn usize_range_bounds(&self, range: impl RangeBounds<T>) -> UsizeRangeBounds {
        use UsizeRangeBounds::*;
        match self.start_bound_for(&range.start_bound()) {
            Bound::Unbounded => match self.end_bound_for(&range.end_bound()) {
                Bound::Unbounded => RangeFull,
                Bound::Included(end) => RangeToInclusive(end),
                Bound::Excluded(end) => RangeTo(end),
            },
            Bound::Included(start) => match self.end_bound_for(&range.end_bound()) {
                Bound::Unbounded => RangeFrom(start),
                Bound::Included(end) => RangeInclusive(start, end),
                Bound::Excluded(end) => Range(start, end),
            },
            // This should never happen
            Bound::Excluded(start) => match self.end_bound_for(&range.end_bound()) {
                Bound::Unbounded => RangeFrom(start),
                Bound::Included(end) => RangeInclusive(start, end),
                Bound::Excluded(end) => Range(start, end),
            },
        }
    }

    /// Returns a reference to a subslice of the set's elements using indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let set = OrdListSet::<&str>::from(["a", "d", "f", "h", "j", "k", "l"]);
    ///
    /// assert!(set.items(set.len()..).is_empty());
    /// assert_eq!(set.items(..=2), ["a", "d", "f",]);
    /// assert_eq!(set.items(..2), ["a", "d", ]);
    /// assert_eq!(set.items(1..5), ["d", "f", "h", "j"]);
    /// assert_eq!(set.items(1..=5), ["d", "f", "h", "j", "k"]);
    /// assert_eq!(set.items(..), ["a", "d", "f", "h", "j", "k", "l"]);
    /// assert_eq!(set.items(2..), ["f", "h", "j", "k", "l"]);
    /// ```
    pub fn items(&self, range: impl RangeBounds<usize>) -> &[T] {
        self.items_private(&UsizeRangeBounds::for_range_bounds(range))
    }

    /// Returns a reference to a subslice of the set's elements using items.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let set = OrdListSet::<&str>::from(["a", "d", "f", "h", "j", "k", "l"]);
    ///
    /// assert!(set.item_items("m"..).is_empty());
    /// assert_eq!(set.item_items(..="f"), ["a", "d", "f",]);
    /// assert_eq!(set.item_items(..="g"), ["a", "d", "f",]);
    /// assert_eq!(set.item_items(.."f"), ["a", "d", ]);
    /// assert_eq!(set.item_items(.."g"), ["a", "d", "f"]);
    /// assert_eq!(set.item_items("d".."k"), ["d", "f", "h", "j"]);
    /// assert_eq!(set.item_items("c".."k"), ["d", "f", "h", "j"]);
    /// assert_eq!(set.item_items("d"..="k"), ["d", "f", "h", "j", "k"]);
    /// assert_eq!(set.item_items(..), ["a", "d", "f", "h", "j", "k", "l"]);
    /// assert_eq!(set.item_items("f"..), ["f", "h", "j", "k", "l"]);
    /// assert_eq!(set.item_items("e"..), ["f", "h", "j", "k", "l"]);
    /// ```
    pub fn item_items(&self, range: impl RangeBounds<T>) -> &[T] {
        self.items_private(&self.usize_range_bounds(range))
    }

    /// Returns an OrdListSet<T> subset of using indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let set = OrdListSet::<&str>::from(["a", "d", "f", "h", "j", "k", "l"]);
    ///
    /// assert!(set.get_subset(set.len()..).is_empty());
    /// assert_eq!(set.get_subset(..=2), ["a", "d", "f",].into());
    /// assert_eq!(set.get_subset(..2), ["a", "d", ].into());
    /// assert_eq!(set.get_subset(1..5), ["d", "f", "h", "j"].into());
    /// assert_eq!(set.get_subset(1..=5), ["d", "f", "h", "j", "k"].into());
    /// assert_eq!(set.get_subset(..), ["a", "d", "f", "h", "j", "k", "l"].into());
    /// assert_eq!(set.get_subset(2..), ["f", "h", "j", "k", "l"].into());
    /// ```
    pub fn get_subset(&self, range: impl RangeBounds<usize>) -> OrdListSet<T> {
        Self::from(self.items(range))
    }

    /// Returns an OrdListSet<T> subset of using items.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let set = OrdListSet::<&str>::from(["a", "d", "f", "h", "j", "k", "l"]);
    ///
    /// assert!(set.get_item_subset("m"..).is_empty());
    /// assert_eq!(set.get_item_subset(..="f"), ["a", "d", "f",].into());
    /// assert_eq!(set.get_item_subset(..="g"), ["a", "d", "f",].into());
    /// assert_eq!(set.get_item_subset(.."f"), ["a", "d", ].into());
    /// assert_eq!(set.get_item_subset(.."g"), ["a", "d", "f"].into());
    /// assert_eq!(set.get_item_subset("d".."k"), ["d", "f", "h", "j"].into());
    /// assert_eq!(set.get_item_subset("c".."k"), ["d", "f", "h", "j"].into());
    /// assert_eq!(set.get_item_subset("d"..="k"), ["d", "f", "h", "j", "k"].into());
    /// assert_eq!(set.get_item_subset(..), ["a", "d", "f", "h", "j", "k", "l"].into());
    /// assert_eq!(set.get_item_subset("f"..), ["f", "h", "j", "k", "l"].into());
    /// assert_eq!(set.get_item_subset("e"..), ["f", "h", "j", "k", "l"].into());
    /// ```
    pub fn get_item_subset(&self, range: impl RangeBounds<T>) -> OrdListSet<T> {
        Self::from(self.item_items(range))
    }

    /// Returns a reference to the first element in the set, if any. This element is always the minimum of all elements in the set.
    pub fn first(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.members.first()
    }

    pub fn first_and_tail(&self) -> Option<(&T, OrdListSet<T>)> {
        let first = self.members.first()?;
        Some((first, self.get_subset(1..)))
    }

    /// Returns a reference to the last element in the set, if any. This element is always the maximum of all elements in the set.
    pub fn last(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.members.last()
    }
}

#[derive(Clone)]
pub struct Union<'a, T: Ord> {
    left_iter: OrdListSetIter<'a, T>,
    right_iter: OrdListSetIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Union<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        union_next!(self.left_iter, self.right_iter)
    }
}

impl<'a, T: 'a + Ord + Clone> PeepAdvanceIter<'a, T> for Union<'a, T> {
    fn peep(&mut self) -> Option<&'a T> {
        union_peep!(self.left_iter, self.right_iter)
    }

    fn advance_until(&mut self, target: &T) {
        self.left_iter.advance_until(target);
        self.right_iter.advance_until(target)
    }

    fn advance_after(&mut self, target: &T) {
        self.left_iter.advance_after(target);
        self.right_iter.advance_after(target)
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T> for Union<'a, T> {}

#[derive(Clone)]
pub struct Intersection<'a, T: Ord> {
    left_iter: OrdListSetIter<'a, T>,
    right_iter: OrdListSetIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Intersection<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        intersection_next!(self.left_iter, self.right_iter)
    }
}

impl<'a, T: 'a + Ord + Clone> PeepAdvanceIter<'a, T> for Intersection<'a, T> {
    fn peep(&mut self) -> Option<&'a T> {
        intersection_peep!(self.left_iter, self.right_iter)
    }

    fn advance_until(&mut self, target: &T) {
        self.left_iter.advance_until(target);
        self.right_iter.advance_until(target)
    }

    fn advance_after(&mut self, target: &T) {
        self.left_iter.advance_after(target);
        self.right_iter.advance_after(target)
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T> for Intersection<'a, T> {}

#[derive(Clone)]
pub struct Difference<'a, T: Ord> {
    left_iter: OrdListSetIter<'a, T>,
    right_iter: OrdListSetIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Difference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        difference_next!(self.left_iter, self.right_iter)
    }
}

impl<'a, T: 'a + Ord + Clone> PeepAdvanceIter<'a, T> for Difference<'a, T> {
    fn peep(&mut self) -> Option<&'a T> {
        difference_peep!(self.left_iter, self.right_iter)
    }

    fn advance_until(&mut self, target: &T) {
        self.left_iter.advance_until(target);
        self.right_iter.advance_until(target)
    }

    fn advance_after(&mut self, target: &T) {
        self.left_iter.advance_after(target);
        self.right_iter.advance_after(target)
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T> for Difference<'a, T> {}

#[derive(Clone)]
pub struct SymmetricDifference<'a, T: Ord> {
    left_iter: OrdListSetIter<'a, T>,
    right_iter: OrdListSetIter<'a, T>,
}

impl<'a, T: Ord> Iterator for SymmetricDifference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        symmetric_difference_next!(self.left_iter, self.right_iter)
    }
}

impl<'a, T: 'a + Ord + Clone> PeepAdvanceIter<'a, T> for SymmetricDifference<'a, T> {
    fn peep(&mut self) -> Option<&'a T> {
        symmetric_difference_peep!(self.left_iter, self.right_iter)
    }

    fn advance_until(&mut self, target: &T) {
        self.left_iter.advance_until(target);
        self.right_iter.advance_until(target)
    }

    fn advance_after(&mut self, target: &T) {
        self.left_iter.advance_after(target);
        self.right_iter.advance_after(target)
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T> for SymmetricDifference<'a, T> {}

impl<'a, T: 'a + Ord + Clone> OrdListSet<T> {
    /// Visits the values representing the difference, i.e., all the values in `self` but not in
    /// `other`,without duplicates, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a = OrdListSet::<&str>::from(["a", "d", "f", "h"]);
    /// let b = OrdListSet::<&str>::from(["b", "c", "d", "i", "h"]);
    ///
    /// let difference: Vec<&str> = a.difference(&b).cloned().collect();
    /// assert_eq!(difference, ["a", "f",]);
    /// ```
    pub fn difference(&'a self, other: &'a Self) -> Difference<'a, T> {
        Difference {
            left_iter: self.iter(),
            right_iter: other.iter(),
        }
    }

    /// Visits the values representing the intersectio, i.e., all the values in both `self` and
    /// `other`,without duplicates, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a = OrdListSet::<&str>::from(["a", "d", "f", "h"]);
    /// let b = OrdListSet::<&str>::from(["b", "c", "d", "i", "h"]);
    ///
    /// let intersection: Vec<&str> = a.intersection(&b).cloned().collect();
    /// assert_eq!(intersection, ["d", "h",]);
    /// ```
    pub fn intersection(&'a self, other: &'a Self) -> Intersection<'a, T> {
        Intersection {
            left_iter: self.iter(),
            right_iter: other.iter(),
        }
    }

    /// Visits the values representing the symmetric difference, i.e., all the values in `self` or
    /// `other` but not in both,without duplicates, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a = OrdListSet::<&str>::from(["a", "d", "f", "h"]);
    /// let b = OrdListSet::<&str>::from(["b", "c", "d", "i", "h"]);
    ///
    /// let symmetric_difference: Vec<&str> = a.symmetric_difference(&b).cloned().collect();
    /// assert_eq!(symmetric_difference, ["a", "b", "c", "f", "i"]);
    /// ```
    pub fn symmetric_difference(&'a self, other: &'a Self) -> SymmetricDifference<'a, T> {
        SymmetricDifference {
            left_iter: self.iter(),
            right_iter: other.iter(),
        }
    }

    /// Visits the values representing the union, i.e., all the values in `self` or `other`,
    /// without duplicates, in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a: OrdListSet<&str> = ["a", "d", "f", "h"].into();
    /// let b: OrdListSet<&str> = ["b", "c", "d", "i", "h"].into();
    ///
    /// let union: Vec<&str> = a.union(&b).cloned().collect();
    /// assert_eq!(union, ["a", "b", "c", "d", "f", "h", "i",]);
    /// ```
    pub fn union(&'a self, other: &'a Self) -> Union<'a, T> {
        Union {
            left_iter: self.iter(),
            right_iter: other.iter(),
        }
    }

    /// Is the output of the given Iterator disjoint from the output of
    /// this iterator?
    pub fn is_disjoint(&self, other: &'a Self) -> bool {
        self.iter().is_disjoint(other.iter())
    }

    /// Is the output of the given Iterator a proper subset of the output of
    /// this iterator?
    pub fn is_proper_subset(&self, other: &'a Self) -> bool {
        self.iter().is_proper_subset(other.iter())
    }

    /// Is the output of the given Iterator a proper superset of the output of
    /// this iterator?
    pub fn is_proper_superset(&self, other: &'a Self) -> bool {
        self.iter().is_proper_superset(other.iter())
    }

    /// Is the output of the given Iterator a subset of the output of
    /// this iterator?
    pub fn is_subset(&self, other: &'a Self) -> bool {
        self.iter().is_subset(other.iter())
    }

    /// Is the output of the given Iterator a superset of the output of
    /// this iterator?
    pub fn is_superset(&self, other: &'a Self) -> bool {
        self.iter().is_superset(other.iter())
    }
}

fn is_sorted_and_no_dups<T: Ord>(list: &[T]) -> bool {
    if !list.is_empty() {
        let mut last = &list[0];
        for element in list[1..].iter() {
            if element <= last {
                return false;
            } else {
                last = element;
            }
        }
    }
    true
}

impl<T: Ord, const N: usize> From<[T; N]> for OrdListSet<T> {
    fn from(members: [T; N]) -> Self {
        let mut members = Vec::from(members);
        members.sort_unstable();
        debug_assert!(is_sorted_and_no_dups(&members));
        Self { members }
    }
}

impl<T: Ord + Clone> From<&[T]> for OrdListSet<T> {
    fn from(members: &[T]) -> Self {
        let mut members = Vec::from(members);
        members.sort_unstable();
        debug_assert!(is_sorted_and_no_dups(&members));
        Self { members }
    }
}

impl<T: Ord> From<BTreeSet<T>> for OrdListSet<T> {
    fn from(mut set: BTreeSet<T>) -> Self {
        let mut members: Vec<T> = Vec::with_capacity(set.len());
        while let Some(member) = set.pop_first() {
            members.push(member);
        }
        Self { members }
    }
}

#[allow(clippy::from_over_into)] // NB: we can't do from on an imported struct
impl<T: Ord + Clone> Into<BTreeSet<T>> for OrdListSet<T> {
    fn into(self) -> BTreeSet<T> {
        BTreeSet::<T>::from_iter(self.iter().cloned())
    }
}

// TODO: implement from for all known iterators
// impl<'a, T: Ord + Clone> From<dyn PeepAdvanceIter<'a, T>> for OrdListSet<T> {
//     fn from(oso_iter: dyn PeepAdvanceIter<T>) -> Self {
//         let members: Vec<T> = oso_iter.cloned().collect();
//         Self { members }
//     }
// }

impl<'a, T: Ord + Clone> From<Union<'a, T>> for OrdListSet<T> {
    fn from(iter: Union<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members }
    }
}

impl<'a, T: Ord + Clone> From<Intersection<'a, T>> for OrdListSet<T> {
    fn from(iter: Intersection<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members }
    }
}

impl<'a, T: Ord + Clone> From<Difference<'a, T>> for OrdListSet<T> {
    fn from(iter: Difference<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members }
    }
}

impl<'a, T: Ord + Clone> From<SymmetricDifference<'a, T>> for OrdListSet<T> {
    fn from(iter: SymmetricDifference<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self { members }
    }
}

impl<T: Ord> FromIterator<T> for OrdListSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut members: Vec<T> = iter.into_iter().collect();
        members.sort_unstable();
        debug_assert!(is_sorted_and_no_dups(&members));
        Self { members }
    }
}

impl<T: Ord + Clone> Sub<&OrdListSet<T>> for &OrdListSet<T> {
    type Output = OrdListSet<T>;

    /// Returns the difference of `self` and `rhs` as a new `OrdListSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a = OrdListSet::<u32>::from([1, 2, 3, 5]);
    /// let b = OrdListSet::<u32>::from([2, 3, 4]);
    ///
    /// assert_eq!(&a - &b, OrdListSet::<u32>::from([1, 5]));
    /// ```
    fn sub(self, rhs: &OrdListSet<T>) -> OrdListSet<T> {
        self.difference(rhs).into()
    }
}

impl<T: Ord + Clone> BitAnd<&OrdListSet<T>> for &OrdListSet<T> {
    type Output = OrdListSet<T>;

    /// Returns the intersection of `self` and `rhs` as a new `OrdListSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a = OrdListSet::<u32>::from([1, 2, 3, 5]);
    /// let b = OrdListSet::<u32>::from([2, 3, 4]);
    ///
    /// assert_eq!(&a & &b, OrdListSet::<u32>::from([2, 3,]));
    /// ```
    fn bitand(self, rhs: &OrdListSet<T>) -> OrdListSet<T> {
        self.intersection(rhs).into()
    }
}

impl<T: Ord + Clone> BitXor<&OrdListSet<T>> for &OrdListSet<T> {
    type Output = OrdListSet<T>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `OrdListSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a = OrdListSet::<u32>::from([1, 2, 3, 5]);
    /// let b = OrdListSet::<u32>::from([2, 3, 4]);
    ///
    /// assert_eq!(&a ^ &b, OrdListSet::<u32>::from([1, 4, 5]));
    /// ```
    fn bitxor(self, rhs: &OrdListSet<T>) -> OrdListSet<T> {
        self.symmetric_difference(rhs).into()
    }
}

impl<T: Ord + Clone> BitOr<&OrdListSet<T>> for &OrdListSet<T> {
    type Output = OrdListSet<T>;

    /// Returns the union of `self` and `rhs` as a new `OrdListSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let a = OrdListSet::<u32>::from([1, 2, 3]);
    /// let b = OrdListSet::<u32>::from([2, 3, 4]);
    ///
    /// assert_eq!(&a | &b, OrdListSet::<u32>::from([1, 2, 3, 4]));
    /// ```
    fn bitor(self, rhs: &OrdListSet<T>) -> OrdListSet<T> {
        self.union(rhs).into()
    }
}

/// An Iterator over the elements in an ordered list in ascending order.  Implements the
/// `PeepAdvanceIter` trait enable it to be used in set expressions (or chained functions)
/// obviating the need for the creation of temporary sets to hold intermediate results.
///
/// # Examples
/// ```
/// use ord_list_set::OrdListSet;
/// use ord_set_iter_set_ops::OrdSetIterSetOpsIterator;
///
/// let a = OrdListSet::<u32>::from([1, 2, 3, 7, 8, 9]);
/// let mut iter = a.iter();
/// assert_eq!(iter.next(), Some(&1));
/// assert_eq!(iter.next(), Some(&2));
/// assert_eq!(iter.next(), Some(&3));
/// assert_eq!(iter.next(), Some(&7));
/// assert_eq!(iter.next(), Some(&8));
/// assert_eq!(iter.next(), Some(&9));
/// assert_eq!(iter.next(), None);
/// let b = OrdListSet::<u32>::from([ 7, 8, 9, 10, 11]);
/// assert!(!a.iter().is_disjoint(b.iter()));
/// ```
#[derive(Default)]
pub struct OrdListSetIter<'a, T: Ord> {
    elements: &'a [T],
    index: usize,
}

impl<'a, T: Ord> Clone for OrdListSetIter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            elements: self.elements,
            index: self.index,
        }
    }
}

impl<'a, T: Ord> Iterator for OrdListSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        self.elements.get(index)
    }

    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        self.elements[self.index..].iter().collect()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n;
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.index < self.elements.len() {
            (self.index, Some(self.elements.len() - self.index))
        } else {
            (self.index, None)
        }
    }
}

impl<'a, T: Ord> OrdListSetIter<'a, T> {
    pub fn len(&self) -> usize {
        // avoid subtraction error
        if self.is_empty() {
            0
        } else {
            self.elements.len() - self.index
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.elements.len()
    }
}

impl<'a, T: 'a + Ord> PeepAdvanceIter<'a, T> for OrdListSetIter<'a, T> {
    /// Peep at the next item in the iterator without advancing the iterator.
    fn peep(&mut self) -> Option<&'a T> {
        self.elements.get(self.index)
    }

    /// Advance this iterator to the next item at or after the given item.
    /// Implementation is O(log(n)).
    fn advance_until(&mut self, t: &T) {
        // Make sure we don't go backwards
        if let Some(item) = self.peep() {
            if item < t {
                self.index += match self.elements[self.index..].binary_search(t) {
                    Ok(index) => index,
                    Err(index) => index,
                };
            }
        }
    }

    /// Advance this iterator to the next item at or after the given item.
    /// Default implementation is O(n) but custom built implementations could be as good as O(log(n)).
    fn advance_after(&mut self, t: &T) {
        // Make sure we don't go backwards
        if let Some(item) = self.peep() {
            if item <= t {
                self.index += match self.elements[self.index..].binary_search(t) {
                    Ok(index) => index + 1,
                    Err(index) => index,
                };
            }
        }
    }
}

impl<'a, T: 'a + Ord + Clone> OrdSetIterSetOpsIterator<'a, T> for OrdListSetIter<'a, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_and_tail() {
        let mut set1 = OrdListSet::<&str>::from(["a", "b", "c"]);
        while let Some((key, tail)) = set1.first_and_tail() {
            assert_eq!(set1.len(), tail.len() + 1);
            assert!(!tail.contains(key));
            set1 = tail;
        }
    }

    #[test]
    fn union() {
        let set1: OrdListSet<&str> = ["a", "b", "c"].iter().cloned().collect();
        let set2: OrdListSet<&str> = ["d", "e", "b", "c"].iter().cloned().collect();
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            set1.union(&set2).cloned().collect::<Vec<&str>>()
        );
    }
}
