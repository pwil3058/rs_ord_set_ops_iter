// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::cmp::Ordering;
use std::collections::BTreeSet;

use super::{OrdSetIterSetOpsIterator, PeepAdvanceIter};

#[macro_export]
macro_rules! intersection_next {
    ($left_iter: expr, $right_iter: expr) => {{
        loop {
            if let Some(l_item) = $left_iter.peep() {
                if let Some(r_item) = $right_iter.peep() {
                    match l_item.cmp(r_item) {
                        Ordering::Less => {
                            $left_iter.advance_until(r_item);
                        }
                        Ordering::Greater => {
                            $right_iter.advance_until(l_item);
                        }
                        Ordering::Equal => {
                            $right_iter.next();
                            break $left_iter.next();
                        }
                    }
                } else {
                    break None;
                }
            } else {
                break None;
            }
        }
    }};
}

#[macro_export]
macro_rules! intersection_peep {
    ($left_iter: expr, $right_iter: expr) => {{
        loop {
            if let Some(l_item) = $left_iter.peep() {
                if let Some(r_item) = $right_iter.peep() {
                    match l_item.cmp(r_item) {
                        Ordering::Less => {
                            $left_iter.advance_until(r_item);
                        }
                        Ordering::Greater => {
                            $right_iter.advance_until(l_item);
                        }
                        Ordering::Equal => break Some(l_item),
                    }
                } else {
                    break None;
                }
            } else {
                break None;
            }
        }
    }};
}

#[derive(Clone)]
pub struct IntersectionIterator<'a, T: Ord + Clone> {
    left_iter: Box<dyn PeepAdvanceIter<'a, T> + 'a>,
    right_iter: Box<dyn PeepAdvanceIter<'a, T> + 'a>,
}

impl<'a, T: Ord + Clone> IntersectionIterator<'a, T> {
    pub fn new(
        left_iter: impl PeepAdvanceIter<'a, T> + 'a,
        right_iter: impl PeepAdvanceIter<'a, T> + 'a,
    ) -> Self {
        Self {
            left_iter: Box::new(left_iter),
            right_iter: Box::new(right_iter),
        }
    }
}

impl<'a, T: Ord + Clone> Iterator for IntersectionIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        intersection_next!(self.left_iter, self.right_iter)
    }
}

impl<'a, T> PeepAdvanceIter<'a, T> for IntersectionIterator<'a, T>
where
    T: 'a + Ord + Clone,
{
    fn peep(&mut self) -> Option<&'a T> {
        intersection_peep!(self.left_iter, self.right_iter)
    }

    fn advance_until(&mut self, target: &T) {
        self.left_iter.advance_until(target);
        self.right_iter.advance_until(target);
    }

    fn advance_after(&mut self, target: &T) {
        self.left_iter.advance_after(target);
        self.right_iter.advance_after(target);
    }
}

impl<'a, T: 'a + Ord + Clone + Default> OrdSetIterSetOpsIterator<'a, T>
    for IntersectionIterator<'a, T>
{
}

#[allow(clippy::from_over_into)] // NB: we can't do From() on an imported struct
impl<'a, T: 'a + Ord + Clone> Into<BTreeSet<T>> for IntersectionIterator<'a, T> {
    fn into(self) -> BTreeSet<T> {
        BTreeSet::<T>::from_iter(self.cloned())
    }
}
