// Copyright 2019 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

pub use std::ops::Sub;
use std::{cmp::Ordering, marker::PhantomData};

/// Iterator enhancement to provide peek and advance ahead features. This mechanism
/// is used to optimise implementation of set operation (difference, intersection, etc)
/// iterators.
pub trait SkipAheadIterator<'a, T: 'a + Ord>: Iterator<Item = &'a T> {
    /// Peek at the next item in the iterator without advancing the iterator.
    fn peek(&mut self) -> Option<&'a T>;

    /// Advance this iterator to the next item at or after the given item and
    /// return a pointer to this iterator. Default implementation is O(n) but
    /// custom built implementations could be as good as O(log(n)).
    fn advance_until(&mut self, t: &T) -> &mut Self {
        while let Some(item) = self.peek() {
            if t > item {
                self.next();
            } else {
                break;
            }
        }
        self
    }
}

pub struct AdvanceUntilIter<I: Iterator> {
    iter: I,
    peek: Option<I::Item>,
}

impl<I: Iterator> AdvanceUntilIter<I> {
    pub fn new(mut iter: I) -> Self {
        let peek = iter.next();
        Self { iter, peek }
    }
}

impl<I: Iterator> Iterator for AdvanceUntilIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.peek.take() {
            Some(item) => {
                self.peek = self.iter.next();
                Some(item)
            }
            None => None,
        }
    }
}

impl<'a, T, I> SkipAheadIterator<'a, T> for AdvanceUntilIter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
{
    fn peek(&mut self) -> Option<&'a T> {
        self.peek
    }

    fn advance_until(&mut self, t: &T) -> &mut Self {
        if let Some(item) = self.peek {
            if t > item {
                while let Some(inner) = self.iter.next() {
                    if t <= inner {
                        self.peek = Some(inner);
                        return self;
                    }
                }
                self.peek = None;
            }
        }
        self
    }
}

impl<'a, T, I> IterSetOperations<'a, T> for AdvanceUntilIter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
{
}

impl<'a, T, I, O> std::ops::Sub<O> for AdvanceUntilIter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: SkipAheadIterator<'a, T>,
{
    type Output = SetOperationIter<'a, T, Self, O>;

    fn sub(self, other: O) -> Self::Output {
        self.difference(other)
    }
}

pub trait IterSetOperations<'a, T>: SkipAheadIterator<'a, T> + Sized
where
    T: 'a + Ord,
{
    /// Iterate over the set difference of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn difference<I: SkipAheadIterator<'a, T>>(self, iter: I) -> SetOperationIter<'a, T, Self, I> {
        SetOperationIter::Difference(self, iter)
    }

    /// Iterate over the set intersection of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn intersection<I: SkipAheadIterator<'a, T>>(
        self,
        iter: I,
    ) -> SetOperationIter<'a, T, Self, I> {
        SetOperationIter::Intersection(self, iter)
    }
    /// Iterate over the set difference of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn symmetric_difference<I: SkipAheadIterator<'a, T>>(
        self,
        iter: I,
    ) -> SetOperationIter<'a, T, Self, I> {
        SetOperationIter::SymmetricDifference(self, iter)
    }

    /// Iterate over the set union of this Iterator and the given Iterator
    /// in the order defined by their elements `Ord` trait implementation.
    fn union<I: SkipAheadIterator<'a, T>>(self, iter: I) -> SetOperationIter<'a, T, Self, I> {
        SetOperationIter::Union(self, iter)
    }

    /// Is the output of the given Iterator disjoint from the output of
    /// this iterator?
    fn is_disjoint<I: SkipAheadIterator<'a, T>>(mut self, mut other: I) -> bool {
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
    fn is_proper_subset<I: SkipAheadIterator<'a, T>>(mut self, mut other: I) -> bool {
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
    fn is_proper_superset<I: SkipAheadIterator<'a, T>>(mut self, mut other: I) -> bool {
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
    fn is_subset<I: SkipAheadIterator<'a, T>>(mut self, mut other: I) -> bool {
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
    fn is_superset<I: SkipAheadIterator<'a, T>>(mut self, mut other: I) -> bool {
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

pub enum SetOperationIter<'a, T, L, R>
where
    T: 'a + Ord,
    L: SkipAheadIterator<'a, T>,
    R: SkipAheadIterator<'a, T>,
{
    Difference(L, R),
    Intersection(L, R),
    SymmetricDifference(L, R),
    Union(L, R),
    Bogus(PhantomData<&'a T>),
}

impl<'a, T, L, R> Iterator for SetOperationIter<'a, T, L, R>
where
    T: 'a + Ord,
    L: SkipAheadIterator<'a, T>,
    R: SkipAheadIterator<'a, T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        use SetOperationIter::*;
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

impl<'a, T, L, R> SkipAheadIterator<'a, T> for SetOperationIter<'a, T, L, R>
where
    T: 'a + Ord,
    L: SkipAheadIterator<'a, T>,
    R: SkipAheadIterator<'a, T>,
{
    fn peek(&mut self) -> Option<&'a T> {
        use SetOperationIter::*;
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

    fn advance_until(&mut self, t: &T) -> &mut Self {
        use SetOperationIter::*;
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
        self
    }
}

impl<'a, T, L, R> IterSetOperations<'a, T> for SetOperationIter<'a, T, L, R>
where
    T: Ord + 'a,
    L: SkipAheadIterator<'a, T>,
    R: SkipAheadIterator<'a, T>,
{
}

impl<'a, T, L, R, O> std::ops::Sub<O> for SetOperationIter<'a, T, L, R>
where
    T: Ord + 'a,
    L: SkipAheadIterator<'a, T>,
    R: SkipAheadIterator<'a, T>,
    O: SkipAheadIterator<'a, T>,
{
    type Output = SetOperationIter<'a, T, Self, O>;

    fn sub(self, other: O) -> Self::Output {
        self.difference(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AdvanceUntilIter, IterSetOperations};

    #[test]
    fn set_relations() {
        let iter1 = AdvanceUntilIter::new(["a", "b", "c", "d"].iter());
        let iter2 = AdvanceUntilIter::new(["b", "c", "d"].iter());
        assert!(iter1.is_superset(iter2));
        let iter1 = AdvanceUntilIter::new(["a", "b", "c", "d"].iter());
        let iter2 = AdvanceUntilIter::new(["b", "c", "d"].iter());
        assert!(!iter1.is_subset(iter2));
    }

    #[test]
    fn set_difference() {
        assert_eq!(
            vec!["a"],
            AdvanceUntilIter::new(["a", "b", "c", "d"].iter())
                .difference(AdvanceUntilIter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0],
            AdvanceUntilIter::new([0, 1, 2, 3].iter())
                .difference(AdvanceUntilIter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_intersection() {
        assert_eq!(
            vec!["b", "c", "d"],
            AdvanceUntilIter::new(["a", "b", "c", "d"].iter())
                .intersection(AdvanceUntilIter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![1, 2, 3],
            AdvanceUntilIter::new([0, 1, 2, 3].iter())
                .intersection(AdvanceUntilIter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_symmetric_difference() {
        assert_eq!(
            vec!["a", "e"],
            AdvanceUntilIter::new(["a", "b", "c", "d"].iter())
                .symmetric_difference(AdvanceUntilIter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0, 4, 5],
            AdvanceUntilIter::new([0, 1, 2, 3].iter())
                .symmetric_difference(AdvanceUntilIter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_union() {
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            AdvanceUntilIter::new(["a", "b", "c", "d"].iter())
                .union(AdvanceUntilIter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5],
            AdvanceUntilIter::new([0, 1, 2, 3].iter())
                .union(AdvanceUntilIter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }
}
