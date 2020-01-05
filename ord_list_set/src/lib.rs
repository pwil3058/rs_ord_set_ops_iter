// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
//! Sets implemented as a sorted list.

use std::{iter::FromIterator, ops::BitOr};

use ord_set_ops_iter::OrdSetOpsIterator;

/// A set of items of type T ordered according to Ord (with no duplicates)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct OrdListSet<T: Ord> {
    members: Vec<T>,
}

impl<T: Ord> OrdListSet<T> {
    pub fn iter(&self) -> OrdListSetIter<'_, T> {
        OrdListSetIter {
            elements: &self.members,
            index: 0,
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
    /// let a: OrdListSet<&str> = ["a", "d", "f", "h"].iter().cloned().collect();
    /// let b: OrdListSet<&str> = ["b", "c", "d", "i", "h"].iter().cloned().collect();
    ///
    /// let union: Vec<&str> = a.union(&b).cloned().collect();
    /// assert_eq!(union, ["a", "b", "c", "d", "f", "h", "i",]);
    /// ```
    pub fn union<'a>(&'a self, other: &'a Self) -> impl OrdSetOpsIterator<'a, T> {
        self.iter().union(other.iter())
    }
}

impl<T: Ord> From<Vec<T>> for OrdListSet<T> {
    fn from(mut members: Vec<T>) -> Self {
        members.sort_unstable();
        members.dedup();
        Self { members }
    }
}

impl<T: Ord> FromIterator<T> for OrdListSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut members: Vec<T> = iter.into_iter().collect();
        members.sort_unstable();
        members.dedup();
        Self { members }
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
    /// let a = OrdListSet::<u32>::from(vec![1, 2, 3]);
    /// let b = OrdListSet::<u32>::from(vec![2, 3, 4]);
    ///
    /// assert_eq!(&a | &b, OrdListSet::<u32>::from(vec![1, 2, 3, 4]));
    /// ```
    fn bitor(self, rhs: &OrdListSet<T>) -> OrdListSet<T> {
        self.union(rhs).cloned().collect()
    }
}

/// An Iterator over the elements in an ordered list
pub struct OrdListSetIter<'a, T: Ord> {
    elements: &'a [T],
    index: usize,
}

impl<'a, T: Ord> Iterator for OrdListSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(element) = self.elements.get(self.index) {
            self.index += 1;
            Some(element)
        } else {
            None
        }
    }
}

impl<'a, T: 'a + Ord> OrdSetOpsIterator<'a, T> for OrdListSetIter<'a, T> {
    /// Peep at the next item in the iterator without advancing the iterator.
    fn peep(&mut self) -> Option<&'a T> {
        self.elements.get(self.index)
    }

    /// Advance this iterator to the next item at or after the given item.
    /// Implementation is O(log(n)).
    fn advance_until(&mut self, t: &T) {
        self.index += match self.elements[self.index..].binary_search(t) {
            Ok(index) => index,
            Err(index) => index,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
