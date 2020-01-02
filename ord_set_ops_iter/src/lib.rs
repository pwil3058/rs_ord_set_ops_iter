// Copyright 2019 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

pub use std::ops::{BitAnd, BitOr, BitXor, Sub};
use std::{cmp::Ordering, marker::PhantomData};

pub mod adapter;

/// Ordered Iterator over set operations on the contents of an ordered set.
pub trait OrdSetOpsIterator<'a, T: 'a + Ord>: Iterator<Item = &'a T> + Sized {
    /// Peek at the next item in the iterator without advancing the iterator.
    fn peek(&mut self) -> Option<&'a T>;

    /// Advance this iterator to the next item at or after the given item and
    /// return a pointer to this iterator. Default implementation is O(n) but
    /// custom built implementations could be as good as O(log(n)).
    fn advance_until(&mut self, t: &T) {
        while let Some(item) = self.peek() {
            if t > item {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Iterate over the set difference of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn difference<I: OrdSetOpsIterator<'a, T>>(self, iter: I) -> OrdSetOpsIter<'a, T, Self, I> {
        OrdSetOpsIter::Difference(self, iter)
    }

    /// Iterate over the set intersection of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn intersection<I: OrdSetOpsIterator<'a, T>>(self, iter: I) -> OrdSetOpsIter<'a, T, Self, I> {
        OrdSetOpsIter::Intersection(self, iter)
    }
    /// Iterate over the set difference of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn symmetric_difference<I: OrdSetOpsIterator<'a, T>>(
        self,
        iter: I,
    ) -> OrdSetOpsIter<'a, T, Self, I> {
        OrdSetOpsIter::SymmetricDifference(self, iter)
    }

    /// Iterate over the set union of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn union<I: OrdSetOpsIterator<'a, T>>(self, iter: I) -> OrdSetOpsIter<'a, T, Self, I> {
        OrdSetOpsIter::Union(self, iter)
    }

    /// Is the output of the given Iterator disjoint from the output of
    /// this iterator?
    fn is_disjoint<I: OrdSetOpsIterator<'a, T>>(mut self, mut other: I) -> bool {
        loop {
            if let Some(my_item) = self.peek() {
                if let Some(other_item) = other.peek() {
                    match my_item.cmp(&other_item) {
                        Ordering::Less => {
                            self.advance_until(other_item);
                        }
                        Ordering::Greater => {
                            other.advance_until(my_item);
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

    /// Is the output of the given Iterator a proper subset of the output of
    /// this iterator?
    fn is_proper_subset<I: OrdSetOpsIterator<'a, T>>(mut self, mut other: I) -> bool {
        let mut result = false;
        while let Some(my_item) = self.peek() {
            if let Some(other_item) = other.peek() {
                match my_item.cmp(&other_item) {
                    Ordering::Less => {
                        return false;
                    }
                    Ordering::Greater => {
                        result = true;
                        other.advance_until(my_item);
                    }
                    Ordering::Equal => {
                        other.next();
                        self.next();
                    }
                }
            } else {
                return false;
            }
        }
        result
    }

    /// Is the output of the given Iterator a proper superset of the output of
    /// this iterator?
    fn is_proper_superset<I: OrdSetOpsIterator<'a, T>>(mut self, mut other: I) -> bool {
        let mut result = false;
        while let Some(my_item) = self.peek() {
            if let Some(other_item) = other.peek() {
                match my_item.cmp(&other_item) {
                    Ordering::Less => {
                        result = true;
                        self.advance_until(other_item);
                    }
                    Ordering::Greater => {
                        return false;
                    }
                    Ordering::Equal => {
                        other.next();
                        self.next();
                    }
                }
            } else {
                return false;
            }
        }
        result
    }

    /// Is the output of the given Iterator a subset of the output of
    /// this iterator?
    fn is_subset<I: OrdSetOpsIterator<'a, T>>(mut self, mut other: I) -> bool {
        while let Some(my_item) = self.peek() {
            if let Some(other_item) = other.peek() {
                match my_item.cmp(&other_item) {
                    Ordering::Less => {
                        return false;
                    }
                    Ordering::Greater => {
                        other.advance_until(my_item);
                    }
                    Ordering::Equal => {
                        other.next();
                        self.next();
                    }
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Is the output of the given Iterator a superset of the output of
    /// this iterator?
    fn is_superset<I: OrdSetOpsIterator<'a, T>>(mut self, mut other: I) -> bool {
        while let Some(my_item) = self.peek() {
            if let Some(other_item) = other.peek() {
                match my_item.cmp(&other_item) {
                    Ordering::Less => {
                        self.advance_until(other_item);
                    }
                    Ordering::Greater => {
                        return false;
                    }
                    Ordering::Equal => {
                        other.next();
                        self.next();
                    }
                }
            } else {
                return false;
            }
        }
        true
    }
}

pub enum OrdSetOpsIter<'a, T, L, R>
where
    T: 'a + Ord,
    L: OrdSetOpsIterator<'a, T>,
    R: OrdSetOpsIterator<'a, T>,
{
    Difference(L, R),
    Intersection(L, R),
    SymmetricDifference(L, R),
    Union(L, R),
    Bogus(PhantomData<&'a T>),
}

impl<'a, T, L, R> Iterator for OrdSetOpsIter<'a, T, L, R>
where
    T: 'a + Ord,
    L: OrdSetOpsIterator<'a, T>,
    R: OrdSetOpsIterator<'a, T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        use OrdSetOpsIter::*;
        match self {
            Difference(l_iter, r_iter) => {
                while let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less => {
                                return l_iter.next();
                            }
                            Ordering::Greater => {
                                r_iter.advance_until(l_item);
                            }
                            Ordering::Equal => {
                                l_iter.next();
                                r_iter.next();
                            }
                        }
                    } else {
                        return l_iter.next();
                    }
                }
                None
            }
            Intersection(l_iter, r_iter) => {
                if let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less => {
                                l_iter.advance_until(r_item);
                                l_iter.next()
                            }
                            Ordering::Greater => {
                                r_iter.advance_until(l_item);
                                r_iter.next()
                            }
                            Ordering::Equal => l_iter.next(),
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            SymmetricDifference(l_iter, r_iter) => {
                while let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less => {
                                return l_iter.next();
                            }
                            Ordering::Greater => {
                                return r_iter.next();
                            }
                            Ordering::Equal => {
                                l_iter.next();
                                r_iter.next();
                            }
                        }
                    } else {
                        return l_iter.next();
                    }
                }
                r_iter.next()
            }
            Union(l_iter, r_iter) => {
                if let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less => l_iter.next(),
                            Ordering::Greater => r_iter.next(),
                            Ordering::Equal => {
                                r_iter.next();
                                l_iter.next()
                            }
                        }
                    } else {
                        l_iter.next()
                    }
                } else {
                    r_iter.next()
                }
            }
            Bogus(_) => panic!("'Bogus' should never be used"),
        }
    }
}

impl<'a, T, L, R> OrdSetOpsIterator<'a, T> for OrdSetOpsIter<'a, T, L, R>
where
    T: 'a + Ord,
    L: OrdSetOpsIterator<'a, T>,
    R: OrdSetOpsIterator<'a, T>,
{
    fn peek(&mut self) -> Option<&'a T> {
        use OrdSetOpsIter::*;
        match self {
            Difference(l_iter, r_iter) => {
                while let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less => {
                                return Some(l_item);
                            }
                            Ordering::Greater => {
                                r_iter.advance_until(l_item);
                            }
                            Ordering::Equal => {
                                l_iter.next();
                                r_iter.next();
                            }
                        }
                    } else {
                        return Some(l_item);
                    }
                }
                None
            }
            Intersection(l_iter, r_iter) => {
                if let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less => {
                                l_iter.advance_until(r_item);
                                l_iter.peek()
                            }
                            Ordering::Greater => {
                                r_iter.advance_until(l_item);
                                r_iter.peek()
                            }
                            Ordering::Equal => Some(l_item),
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            SymmetricDifference(l_iter, r_iter) => {
                while let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less => {
                                return Some(l_item);
                            }
                            Ordering::Greater => {
                                return Some(r_item);
                            }
                            Ordering::Equal => {
                                l_iter.next();
                                r_iter.next();
                            }
                        }
                    } else {
                        return Some(l_item);
                    }
                }
                r_iter.peek()
            }
            Union(l_iter, r_iter) => {
                if let Some(l_item) = l_iter.peek() {
                    if let Some(r_item) = r_iter.peek() {
                        match l_item.cmp(r_item) {
                            Ordering::Less | Ordering::Equal => Some(l_item),
                            Ordering::Greater => Some(r_item),
                        }
                    } else {
                        Some(l_item)
                    }
                } else {
                    r_iter.peek()
                }
            }
            Bogus(_) => panic!("'Bogus' should never be used"),
        }
    }

    fn advance_until(&mut self, t: &T) {
        use OrdSetOpsIter::*;
        match self {
            Difference(l_iter, r_iter) => {
                l_iter.advance_until(t);
                r_iter.advance_until(t);
            }
            Intersection(l_iter, r_iter) => {
                l_iter.advance_until(t);
                r_iter.advance_until(t);
            }
            SymmetricDifference(l_iter, r_iter) => {
                l_iter.advance_until(t);
                r_iter.advance_until(t);
            }
            Union(l_iter, r_iter) => {
                l_iter.advance_until(t);
                r_iter.advance_until(t);
            }
            Bogus(_) => panic!("'Bogus' should never be used"),
        };
    }
}

impl<'a, T, L, R, O> BitAnd<O> for OrdSetOpsIter<'a, T, L, R>
where
    T: Ord + 'a,
    L: OrdSetOpsIterator<'a, T>,
    R: OrdSetOpsIterator<'a, T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitand(self, other: O) -> Self::Output {
        self.intersection(other)
    }
}

impl<'a, T, L, R, O> BitOr<O> for OrdSetOpsIter<'a, T, L, R>
where
    T: Ord + 'a,
    L: OrdSetOpsIterator<'a, T>,
    R: OrdSetOpsIterator<'a, T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitor(self, other: O) -> Self::Output {
        self.union(other)
    }
}

impl<'a, T, L, R, O> BitXor<O> for OrdSetOpsIter<'a, T, L, R>
where
    T: Ord + 'a,
    L: OrdSetOpsIterator<'a, T>,
    R: OrdSetOpsIterator<'a, T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitxor(self, other: O) -> Self::Output {
        self.symmetric_difference(other)
    }
}

impl<'a, T, L, R, O> Sub<O> for OrdSetOpsIter<'a, T, L, R>
where
    T: Ord + 'a,
    L: OrdSetOpsIterator<'a, T>,
    R: OrdSetOpsIterator<'a, T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn sub(self, other: O) -> Self::Output {
        self.difference(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::OrdSetOpsIterator;

    struct Set<T: Ord>(Vec<T>);

    impl<T: Ord + Clone> From<Vec<T>> for Set<T> {
        fn from(mut elements: Vec<T>) -> Self {
            elements.sort();
            elements.dedup();
            Self(elements)
        }
    }

    struct SetIter<'a, T: Ord> {
        elements: &'a [T],
        index: usize,
    }

    impl<'a, T: Ord> Iterator for SetIter<'a, T> {
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

    impl<'a, T: 'a + Ord> OrdSetOpsIterator<'a, T> for SetIter<'a, T> {
        fn advance_until(&mut self, t: &T) {
            self.index += match self.elements[self.index..].binary_search(t) {
                Ok(index) => index,
                Err(index) => index,
            };
        }

        fn peek(&mut self) -> Option<&'a T> {
            self.elements.get(self.index)
        }
    }

    impl<T: Ord> Set<T> {
        pub fn iter(&self) -> SetIter<T> {
            SetIter {
                elements: &self.0,
                index: 0,
            }
        }

        pub fn is_superset(&self, other: &Self) -> bool {
            self.iter().is_superset(other.iter())
        }

        pub fn is_subset(&self, other: &Self) -> bool {
            self.iter().is_subset(other.iter())
        }
    }

    #[test]
    fn set_relations() {
        let set1 = Set::<&str>::from(vec!["a", "b", "c", "d"]);
        let set2 = Set::<&str>::from(vec!["b", "c", "d"]);
        assert!(set1.is_superset(&set2));
        assert!(!set1.is_subset(&set2));
    }

    #[test]
    fn set_difference() {
        let set1 = Set::<&str>::from(vec!["a", "b", "c", "d"]);
        let set2 = Set::<&str>::from(vec!["b", "c", "d", "e"]);
        assert_eq!(
            vec!["a"],
            (set1.iter().difference(set2.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["e"],
            (set2.iter().difference(set1.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn set_intersection() {
        let set1 = Set::<&str>::from(vec!["a", "b", "c", "d"]);
        let set2 = Set::<&str>::from(vec!["b", "c", "d", "e"]);
        assert_eq!(
            vec!["b", "c", "d"],
            (set1.iter().intersection(set2.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["b", "c", "d"],
            (set2.iter().intersection(set1.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn set_symmetric_difference() {
        let set1 = Set::<&str>::from(vec!["a", "b", "c", "d"]);
        let set2 = Set::<&str>::from(vec!["b", "c", "d", "e"]);
        assert_eq!(
            vec!["a", "e"],
            (set1.iter().symmetric_difference(set2.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a", "e"],
            (set2.iter().symmetric_difference(set1.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn set_union() {
        let set1 = Set::<&str>::from(vec!["a", "b", "c", "d"]);
        let set2 = Set::<&str>::from(vec!["b", "c", "d", "e"]);
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            (set1.iter().union(set2.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            (set2.iter().union(set1.iter()))
                .cloned()
                .collect::<Vec<&str>>()
        );
    }
}
