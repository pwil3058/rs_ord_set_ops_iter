// Copyright 2019 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::cmp::Ordering;

/// Iterator enhancement to provide peek and advance ahead features. This mechanism
/// is used to optimise implementation of set operation (difference, intersection, etc)
/// iterators.
pub trait SkipAheadIterator<'a, T: 'a + Ord>: Iterator<Item = &'a T> {
    /// Peek at the next item in the iterator without advancing the iterator.
    fn peek(&mut self) -> Option<&'a T>;

    /// Advance this iterator to the next item after the given item and
    /// return a pointer to this iterator.
    fn advance_past(&mut self, t: &T) -> &mut Self {
        while let Some(item) = self.peek() {
            if t >= item {
                self.next();
            } else {
                break;
            }
        }
        self
    }

    /// Advance this iterator to the next item at or after the given item and
    /// return a pointer to this iterator.
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

pub trait IterSetRelations<'a, T>: SkipAheadIterator<'a, T> + Sized
where
    T: 'a + Ord,
{
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
    fn is_proper_superset<I: IterSetRelations<'a, T>>(self, other: I) -> bool {
        other.is_proper_subset(self)
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
    fn is_superset<I: IterSetRelations<'a, T>>(self, other: I) -> bool {
        other.is_subset(self)
    }
}

pub enum SetOperationType {
    Difference,
    Intersection,
    SymmetricDifference,
    Union,
}

#[cfg(test)]
mod tests {
    use crate::{IterSetRelations, SkipAheadIterator};

    struct SkipAheadIter<I: Iterator> {
        iter: I,
        peeked: Option<Option<I::Item>>,
    }

    impl<I: Iterator> SkipAheadIter<I> {
        pub fn new(iter: I) -> Self {
            Self { iter, peeked: None }
        }
    }

    impl<I: Iterator> Iterator for SkipAheadIter<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<I::Item> {
            match self.peeked.take() {
                Some(item) => item,
                None => self.iter.next(),
            }
        }
    }

    impl<'a, T, I> SkipAheadIterator<'a, T> for SkipAheadIter<I>
    where
        T: Ord + 'a,
        I: Iterator<Item = &'a T>,
    {
        fn peek(&mut self) -> Option<&'a T> {
            let iter = &mut self.iter;
            *self.peeked.get_or_insert_with(|| iter.next())
        }
    }

    impl<'a, T, I> IterSetRelations<'a, T> for SkipAheadIter<I>
    where
        T: Ord + 'a,
        I: Iterator<Item = &'a T>,
    {
    }

    #[test]
    fn set_relations() {
        let iter1 = SkipAheadIter::new(["a", "b", "c", "d"].iter());
        let iter2 = SkipAheadIter::new(["b", "c", "d"].iter());
        assert!(iter1.is_superset(iter2));
        let iter1 = SkipAheadIter::new(["a", "b", "c", "d"].iter());
        let iter2 = SkipAheadIter::new(["b", "c", "d"].iter());
        assert!(!iter1.is_subset(iter2));
    }
}
