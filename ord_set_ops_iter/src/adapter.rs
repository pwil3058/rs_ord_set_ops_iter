// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

pub use std::ops::{BitAnd, BitOr, BitXor, Sub};

pub use crate::{OrdSetOpsIter, OrdSetOpsIterator};

pub struct OrdSetOpsIterAdapter<I: Iterator> {
    iter: I,
    peek: Option<I::Item>,
}

impl<I: Iterator> OrdSetOpsIterAdapter<I> {
    pub fn new(mut iter: I) -> Self {
        let peek = iter.next();
        Self { iter, peek }
    }
}

impl<I: Iterator> Iterator for OrdSetOpsIterAdapter<I> {
    type Item = I::Item;

    // NB: next() does all the work as it will get called less often
    // than peek() when doing set operation iterations
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

impl<'a, T, I> OrdSetOpsIterator<'a, T> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
{
    #[inline]
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

impl<'a, T, I, O> BitAnd<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitand(self, other: O) -> Self::Output {
        self.intersection(other)
    }
}

impl<'a, T, I, O> BitOr<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitor(self, other: O) -> Self::Output {
        self.union(other)
    }
}

impl<'a, T, I, O> BitXor<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
    O: OrdSetOpsIterator<'a, T>,
{
    type Output = OrdSetOpsIter<'a, T, Self, O>;

    #[inline]
    fn bitxor(self, other: O) -> Self::Output {
        self.symmetric_difference(other)
    }
}

impl<'a, T, I, O> Sub<O> for OrdSetOpsIterAdapter<I>
where
    T: Ord + 'a,
    I: Iterator<Item = &'a T>,
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
    use super::OrdSetOpsIterAdapter;
    use crate::OrdSetOpsIterator;

    #[test]
    fn set_relations() {
        let iter1 = OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter());
        let iter2 = OrdSetOpsIterAdapter::new(["b", "c", "d"].iter());
        assert!(iter1.is_superset(iter2));
        let iter1 = OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter());
        let iter2 = OrdSetOpsIterAdapter::new(["b", "c", "d"].iter());
        assert!(!iter1.is_subset(iter2));
    }

    #[test]
    fn set_difference() {
        assert_eq!(
            vec!["a"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .difference(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                - OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .difference(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_intersection() {
        assert_eq!(
            vec!["b", "c", "d"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .intersection(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["b", "c", "d"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                & OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![1, 2, 3],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .intersection(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_symmetric_difference() {
        assert_eq!(
            vec!["a", "e"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .symmetric_difference(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a", "e"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                ^ OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0, 4, 5],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .symmetric_difference(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn set_union() {
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                .union(OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
                .map(|v| *v)
                .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["a", "b", "c", "d", "e"],
            (OrdSetOpsIterAdapter::new(["a", "b", "c", "d"].iter())
                | OrdSetOpsIterAdapter::new(["b", "c", "d", "e"].iter()))
            .map(|v| *v)
            .collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5],
            OrdSetOpsIterAdapter::new([0, 1, 2, 3].iter())
                .union(OrdSetOpsIterAdapter::new([1, 2, 3, 4, 5].iter()))
                .cloned()
                .collect::<Vec<i32>>()
        );
    }
}
