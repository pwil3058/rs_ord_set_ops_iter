// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
//! Sets implemented as a sorted list.

use std::iter::FromIterator;

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

impl<T: Ord> FromIterator<T> for OrdListSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut members: Vec<T> = iter.into_iter().collect();
        members.sort();
        members.dedup();
        Self { members }
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
    fn advance_until(&mut self, t: &T) {
        self.index += match self.elements[self.index..].binary_search(t) {
            Ok(index) => index,
            Err(index) => index,
        };
    }

    fn peep(&mut self) -> Option<&'a T> {
        self.elements.get(self.index)
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
