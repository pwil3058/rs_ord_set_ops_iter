// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use super::*;

use ord_set_iter_set_ops::{
    DifferenceIterator, IntersectionIterator, SymmetricDifferenceIterator, UnionIterator,
};

impl<T: Ord, const N: usize> From<[T; N]> for OrdListSet<T> {
    /// Create an OrdListSet<T> from [T; N]
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = ["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let vec = vec!["a", "b", "c", "d", "e", "f", "x", "y", "z"];
    /// let set = OrdListSet::from(data);
    /// assert_eq!(vec, set.iter().cloned().collect::<Vec<_>>());
    /// ```
    fn from(members: [T; N]) -> Self {
        let mut members = Vec::from(members);
        members.sort_unstable();
        members.dedup();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<T: Ord + Clone> From<&[T]> for OrdListSet<T> {
    /// Create an OrdListSet<T> from &[T]
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let vec = vec!["a", "b", "c", "d", "e", "f", "x", "y", "z"];
    /// let set = OrdListSet::from(&data[..]);
    /// assert_eq!(vec, set.iter().cloned().collect::<Vec<_>>());
    /// ```
    fn from(members: &[T]) -> Self {
        let mut members = Vec::from(members);
        members.sort_unstable();
        members.dedup();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<T: Ord + Clone> From<Vec<T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from Vec<T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let vec = vec!["a", "b", "c", "d", "e", "f", "x", "y", "z"];
    /// let set = OrdListSet::from(data);
    /// assert_eq!(vec, set.iter().cloned().collect::<Vec<_>>());
    /// ```
    fn from(mut members: Vec<T>) -> Self {
        members.sort_unstable();
        members.dedup();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<T: Ord> From<BTreeSet<T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from BTreeSet<T>
    ///
    /// Example:
    /// ```
    /// use std::collections::BTreeSet;
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = ["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let vec = vec!["a", "b", "c", "d", "e", "f", "x", "y", "z"];
    /// let btree_set = BTreeSet::from(data);
    /// let set = OrdListSet::from(btree_set);
    /// assert_eq!(vec, set.iter().cloned().collect::<Vec<_>>());
    /// ```
    fn from(mut set: BTreeSet<T>) -> Self {
        let mut members: Vec<T> = Vec::with_capacity(set.len());
        while let Some(member) = set.pop_first() {
            members.push(member);
        }
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

#[allow(clippy::from_over_into)] // NB: we can't do from on an imported struct
impl<T: Ord + Clone> Into<BTreeSet<T>> for OrdListSet<T> {
    /// Convert an OrdListSet<T> into a BTreeSet<T>
    ///
    /// Example:
    /// ```
    /// use std::collections::BTreeSet;
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = ["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let vec = vec!["a", "b", "c", "d", "e", "f", "x", "y", "z"];
    /// let set = OrdListSet::from(data);
    /// let btree_set: BTreeSet<_> = set.into();
    /// assert_eq!(vec, btree_set.iter().cloned().collect::<Vec<_>>());
    /// ```
    fn into(self) -> BTreeSet<T> {
        BTreeSet::<T>::from_iter(self.iter().cloned())
    }
}

impl<'a, T: Ord + Clone> From<DifferenceIterator<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from DifferenceIterator<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    /// use ord_set_iter_set_ops::*;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.iter().difference(set2.iter());
    /// assert_eq!(vec!["b", "f", "x"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.iter().difference(set1.iter());
    /// assert_eq!(vec!["m", "n", "o"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(oso_iter: DifferenceIterator<'a, T>) -> Self {
        let members: Vec<T> = oso_iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<IntersectionIterator<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from IntersectionIterator<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    /// use ord_set_iter_set_ops::*;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.iter().intersection(set2.iter());
    /// assert_eq!(vec!["a", "c", "d", "e", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.iter().intersection(set1.iter());
    /// assert_eq!(vec!["a", "c", "d", "e", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(oso_iter: IntersectionIterator<'a, T>) -> Self {
        let members: Vec<T> = oso_iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<SymmetricDifferenceIterator<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from SymmetricDifferenceIterator<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    /// use ord_set_iter_set_ops::*;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.iter().symmetric_difference(set2.iter());
    /// assert_eq!(vec!["b", "f", "m", "n", "o", "x"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.iter().symmetric_difference(set1.iter());
    /// assert_eq!(vec!["b", "f", "m", "n", "o", "x"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(oso_iter: SymmetricDifferenceIterator<'a, T>) -> Self {
        let members: Vec<T> = oso_iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<UnionIterator<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from UnionIterator<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    /// use ord_set_iter_set_ops::*;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.iter().union(set2.iter());
    /// assert_eq!(vec!["a", "b", "c", "d", "e", "f", "m", "n", "o", "x", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.iter().union(set1.iter());
    /// assert_eq!(vec!["a", "b", "c", "d", "e", "f", "m", "n", "o", "x", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(oso_iter: UnionIterator<'a, T>) -> Self {
        let members: Vec<T> = oso_iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<OrdListSetIter<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from OrdListSetIter<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let vec = vec!["a", "b", "c", "d", "e", "f", "x", "y", "z"];
    /// let set1 = OrdListSet::from(data);
    /// assert_eq!(vec, set1.iter().cloned().collect::<Vec<_>>());
    /// let set2 = OrdListSet::from(set1.iter());
    /// assert_eq!(&set1, &set2);
    /// ```
    fn from(iter: OrdListSetIter<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<Union<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from Union<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.union(&set2);
    /// assert_eq!(vec!["a", "b", "c", "d", "e", "f", "m", "n", "o", "x", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.union(&set1);
    /// assert_eq!(vec!["a", "b", "c", "d", "e", "f", "m", "n", "o", "x", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(iter: Union<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<Intersection<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from Intersection<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.intersection(&set2);
    /// assert_eq!(vec!["a", "c", "d", "e", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.intersection(&set1);
    /// assert_eq!(vec!["a", "c", "d", "e", "y", "z"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(iter: Intersection<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<Difference<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from Difference<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.difference(&set2);
    /// assert_eq!(vec!["b", "f", "x"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.difference(&set1);
    /// assert_eq!(vec!["m", "n", "o"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(iter: Difference<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<'a, T: Ord + Clone> From<SymmetricDifference<'a, T>> for OrdListSet<T> {
    /// Create an OrdListSet<T> from SymmetricDifference<'a, T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    ///
    /// let data = vec!["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"];
    /// let set1 = OrdListSet::from(["a", "b", "c", "d", "e", "f", "x", "y", "z"]);
    /// let set2 = OrdListSet::from(["a", "c", "d", "e", "m", "n", "o","y", "z"]);
    /// let iter = set1.symmetric_difference(&set2);
    /// assert_eq!(vec!["b", "f", "m", "n", "o", "x"], iter.cloned().collect::<Vec<_>>());
    /// let iter = set2.symmetric_difference(&set1);
    /// assert_eq!(vec!["b", "f", "m", "n", "o", "x"], iter.cloned().collect::<Vec<_>>());
    /// ```
    fn from(iter: SymmetricDifference<'a, T>) -> Self {
        let members: Vec<T> = iter.cloned().collect();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}

impl<T: Ord> FromIterator<T> for OrdListSet<T> {
    /// Create an OrdListSet<T> from Iterator<T>
    ///
    /// Example:
    /// ```
    /// use ord_list_set::OrdListSet;
    /// use std::iter::FromIterator;
    ///
    /// let data = ("z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y");
    /// let vec = vec!["a", "b", "c", "d", "e", "f", "x", "y", "z"];
    /// let iter = ["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"].iter();
    /// let set: OrdListSet<_> = iter.cloned().collect();
    /// assert_eq!(vec, set.items(..));
    /// let iter = ["z", "y", "x", "b", "c", "a", "d", "e", "f", "b", "y"].iter();
    /// let set = OrdListSet::from_iter(iter.cloned());
    /// assert_eq!(vec, set.items(..));
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut members = Vec::new();
        for m in iter {
            members.push(m);
        }
        members.sort_unstable();
        members.dedup();
        Self {
            members: members.into_boxed_slice(),
        }
    }
}
