use dyn_clonable::*;

use std::{
    cmp::Ordering,
    collections::{
        btree_map::{self, BTreeMap},
        btree_set::BTreeSet,
    },
    iter::Peekable,
    ops::{BitAnd, BitOr, BitXor, Sub},
};

pub mod difference_iterator;
pub mod intersection_iterator;
pub mod symmetric_difference_iterator;
pub mod union_iterator;

use difference_iterator::DifferenceIterator;
use intersection_iterator::IntersectionIterator;
use symmetric_difference_iterator::SymmetricDifferenceIterator;
use union_iterator::UnionIterator;

/// Ordered Iterator over set operations on the contents of an ordered set.
#[clonable]
pub trait PeepAdvanceIter<'a, T: 'a + Ord>: Iterator<Item = &'a T> + Clone {
    /// Peep at the next item in the iterator without advancing the iterator.
    fn peep(&mut self) -> Option<&'a T>;

    /// Will the next next() call return None? I.e. is the iterator exhausted?
    fn is_empty(&mut self) -> bool {
        self.peep().is_none()
    }

    /// Advance this iterator to the next item at or after the given item.
    /// Default implementation is O(n) but custom built implementations could be as good as O(log(n)).
    // TODO: try to make advance_until() return &mut Self
    fn advance_until(&mut self, t: &T) {
        while let Some(item) = self.peep() {
            if t > item {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Advance this iterator to the next item at or after the given item.
    /// Default implementation is O(n) but custom built implementations could be as good as O(log(n)).
    // TODO: try to make advance_until() return &mut Self
    fn advance_after(&mut self, t: &T) {
        while let Some(item) = self.peep() {
            if t >= item {
                self.next();
            } else {
                break;
            }
        }
    }
}

impl<'a, T, I> PeepAdvanceIter<'a, T> for Peekable<I>
where
    T: 'a + Ord,
    I: Iterator<Item = &'a T> + Clone,
{
    fn peep(&mut self) -> Option<&'a T> {
        self.peek().copied()
    }
}

// pub enum OrdSetOpsIter<'a, T>
// where
//     T: Ord + Clone,
// {
//     Difference(Box<DifferenceIterator<'a, T>>),
//     Intersection(Box<IntersectionIterator<'a, T>>),
//     SymmetricDifference(Box<SymmetricDifferenceIterator<'a, T>>),
//     Union(Box<UnionIterator<'a, T>>),
//     Plain(Box<dyn PeepAdvanceIter<'a, T, Item = &'a T> + 'a>),
// }

#[derive(Clone)]
pub struct OrdSetOpsIter<'a, T: Ord + Clone> {
    iter: Box<dyn PeepAdvanceIter<'a, T, Item = &'a T> + 'a>,
}

impl<'a, T: Ord + Clone> OrdSetOpsIter<'a, T> {
    pub fn new(iter: impl PeepAdvanceIter<'a, T, Item = &'a T> + 'a) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<'a, T> Iterator for OrdSetOpsIter<'a, T>
where
    T: 'a + Ord + Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn collect<B: FromIterator<Self::Item>>(self) -> B {
        self.iter.collect()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> PeepAdvanceIter<'a, T> for OrdSetOpsIter<'a, T>
where
    T: 'a + Ord + Clone,
{
    fn peep(&mut self) -> Option<&'a T> {
        self.iter.peep()
    }

    fn advance_after(&mut self, target: &T) {
        self.iter.advance_after(target)
    }

    fn advance_until(&mut self, target: &T) {
        self.iter.advance_until(target)
    }
}

pub trait SetOperations<'a, T: 'a + Ord + Clone>: PeepAdvanceIter<'a, T> + Clone {
    fn difference(&self, other: &Self) -> dyn PeepAdvanceIter<'a, T>;
}

impl<'a, T> OrdSetOpsIter<'a, T>
where
    T: 'a + Ord + Clone,
{
    pub fn difference(&self, other: &Self) -> OrdSetOpsIter<'a, T> {
        let iter = DifferenceIterator {
            left_iter: self.clone(),
            right_iter: other.clone(),
        };
        Self::new(iter)
    }

    pub fn intersection(&self, other: &Self) -> OrdSetOpsIter<'a, T> {
        let iter = IntersectionIterator {
            left_iter: self.clone(),
            right_iter: other.clone(),
        };
        Self::new(iter)
    }

    pub fn symmetric_difference(&self, other: &Self) -> OrdSetOpsIter<'a, T> {
        let iter = SymmetricDifferenceIterator {
            left_iter: self.clone(),
            right_iter: other.clone(),
        };
        Self::new(iter)
    }

    pub fn union(&self, other: &Self) -> OrdSetOpsIter<'a, T> {
        let iter = UnionIterator {
            left_iter: self.clone(),
            right_iter: other.clone(),
        };
        Self::new(iter)
    }
}

pub trait SetRelationships<'a, T: 'a + Ord + Clone>: PeepAdvanceIter<'a, T> + Clone {
    fn is_disjoint(&self, other: &Self) -> bool {
        let mut self_iter = self.clone();
        let mut other_iter = other.clone();
        loop {
            if let Some(my_item) = self_iter.peep() {
                if let Some(other_item) = other_iter.peep() {
                    match my_item.cmp(other_item) {
                        Ordering::Less => {
                            self_iter.advance_until(other_item);
                        }
                        Ordering::Greater => {
                            other_iter.advance_until(my_item);
                        }
                        Ordering::Equal => {
                            return false;
                        }
                    }
                } else {
                    return true;
                }
            } else {
                return true;
            }
        }
    }

    fn is_proper_subset(&self, other: &Self) -> bool {
        let mut self_iter = self.clone();
        let mut other_iter = other.clone();
        let mut result = false;
        while let Some(my_item) = self_iter.peep() {
            if let Some(other_item) = other_iter.peep() {
                match my_item.cmp(other_item) {
                    Ordering::Less => {
                        return false;
                    }
                    Ordering::Greater => {
                        result = true;
                        other_iter.advance_until(my_item);
                    }
                    Ordering::Equal => {
                        other_iter.next();
                        self_iter.next();
                    }
                }
            } else {
                return false;
            }
        }
        result || other_iter.peep().is_some()
    }

    fn is_subset(&self, other: &Self) -> bool {
        let mut self_iter = self.clone();
        let mut other_iter = other.clone();
        while let Some(my_item) = self_iter.peep() {
            if let Some(other_item) = other_iter.peep() {
                match my_item.cmp(other_item) {
                    Ordering::Less => {
                        return false;
                    }
                    Ordering::Greater => {
                        other_iter.advance_until(my_item);
                    }
                    Ordering::Equal => {
                        other_iter.next();
                        self_iter.next();
                    }
                }
            } else {
                return false;
            }
        }
        true
    }

    fn is_proper_superset(&self, other: &Self) -> bool {
        let mut self_iter = self.clone();
        let mut other_iter = other.clone();
        let mut result = false;
        while let Some(my_item) = self_iter.peep() {
            if let Some(other_item) = other_iter.peep() {
                match my_item.cmp(other_item) {
                    Ordering::Less => {
                        result = true;
                        self_iter.advance_until(other_item);
                    }
                    Ordering::Greater => {
                        return false;
                    }
                    Ordering::Equal => {
                        other_iter.next();
                        self_iter.next();
                    }
                }
            } else {
                return true;
            }
        }
        result && other_iter.peep().is_none()
    }

    fn is_superset(&self, other: &Self) -> bool {
        let mut self_iter = self.clone();
        let mut other_iter = other.clone();
        while let Some(my_item) = self_iter.peep() {
            if let Some(other_item) = other_iter.peep() {
                match my_item.cmp(other_item) {
                    Ordering::Less => {
                        self_iter.advance_until(other_item);
                    }
                    Ordering::Greater => {
                        return false;
                    }
                    Ordering::Equal => {
                        other_iter.next();
                        self_iter.next();
                    }
                }
            } else {
                return true;
            }
        }
        other_iter.peep().is_none()
    }
}

impl<'a, T: 'a + Ord + Clone> SetRelationships<'a, T> for OrdSetOpsIter<'a, T> {}

impl<'a, T> BitAnd for &OrdSetOpsIter<'a, T>
where
    T: 'a + Ord + Clone,
{
    type Output = OrdSetOpsIter<'a, T>;

    #[inline]
    fn bitand(self, other: Self) -> Self::Output {
        OrdSetOpsIter::intersection(self, other)
    }
}

impl<'a, T> BitOr for &OrdSetOpsIter<'a, T>
where
    T: 'a + Ord + Clone,
{
    type Output = OrdSetOpsIter<'a, T>;

    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        OrdSetOpsIter::union(self, other)
    }
}

impl<'a, T> BitXor for &OrdSetOpsIter<'a, T>
where
    T: 'a + Ord + Clone,
{
    type Output = OrdSetOpsIter<'a, T>;

    #[inline]
    fn bitxor(self, other: Self) -> Self::Output {
        OrdSetOpsIter::symmetric_difference(self, other)
    }
}

impl<'a, T> Sub for &OrdSetOpsIter<'a, T>
where
    T: 'a + Ord + Clone,
{
    type Output = OrdSetOpsIter<'a, T>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        OrdSetOpsIter::difference(self, other)
    }
}

#[allow(clippy::from_over_into)] // NB: we can't do from on an imported struct
impl<'a, T: Ord + Clone> Into<BTreeSet<T>> for OrdSetOpsIter<'a, T> {
    fn into(self) -> BTreeSet<T> {
        BTreeSet::<T>::from_iter(self.cloned())
    }
}

pub trait SetOsoIter<'a, T: 'a + Ord>
where
    T: 'a + Ord + Clone,
{
    fn oso_iter(&'a self) -> OrdSetOpsIter<'a, T>;

    fn oso_difference(&'a self, other: &'a impl SetOsoIter<'a, T>) -> OrdSetOpsIter<'a, T> {
        self.oso_iter().difference(&other.oso_iter())
    }

    fn oso_intersection(&'a self, other: &'a impl SetOsoIter<'a, T>) -> OrdSetOpsIter<'a, T> {
        self.oso_iter().intersection(&other.oso_iter())
    }

    fn oso_symmetric_difference(
        &'a self,
        other: &'a impl SetOsoIter<'a, T>,
    ) -> OrdSetOpsIter<'a, T> {
        self.oso_iter().symmetric_difference(&other.oso_iter())
    }

    fn oso_union(&'a self, other: &'a impl SetOsoIter<'a, T>) -> OrdSetOpsIter<'a, T> {
        self.oso_iter().union(&other.oso_iter())
    }

    fn is_oso_disjoint(&'a self, other: &'a impl SetOsoIter<'a, T>) -> bool {
        self.oso_iter().is_disjoint(&other.oso_iter())
    }

    fn is_oso_subset(&'a self, other: &'a impl SetOsoIter<'a, T>) -> bool {
        self.oso_iter().is_subset(&other.oso_iter())
    }

    fn is_oso_superset(&'a self, other: &'a impl SetOsoIter<'a, T>) -> bool {
        self.oso_iter().is_superset(&other.oso_iter())
    }

    fn is_oso_proper_subset(&'a self, other: &'a impl SetOsoIter<'a, T>) -> bool {
        self.oso_iter().is_proper_subset(&other.oso_iter())
    }

    fn is_oso_proper_superset(&'a self, other: &'a impl SetOsoIter<'a, T>) -> bool {
        self.oso_iter().is_proper_superset(&other.oso_iter())
    }
}

impl<'a, T: 'a + Ord + Clone> SetOsoIter<'a, T> for BTreeSet<T> {
    fn oso_iter(&'a self) -> OrdSetOpsIter<'a, T> {
        OrdSetOpsIter::new(self.iter().peekable())
    }
}

pub trait MapOsoIter<'a, K, I>
where
    K: 'a + Ord + Clone,
    I: Iterator<Item = &'a K> + Clone,
{
    fn oso_keys(&'a self) -> OrdSetOpsIter<'a, K>;
}

impl<'a, K, V> MapOsoIter<'a, K, btree_map::Keys<'a, K, V>> for BTreeMap<K, V>
where
    K: 'a + Ord + Clone,
{
    fn oso_keys(&'a self) -> OrdSetOpsIter<'a, K> {
        OrdSetOpsIter::new(self.keys().peekable())
    }
}

#[cfg(test)]
mod tests;
