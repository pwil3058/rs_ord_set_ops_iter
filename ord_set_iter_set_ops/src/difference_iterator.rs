// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::cmp::Ordering;

use super::{OrdSetOpsIter, PeepAdvanceIter};

#[macro_export]
macro_rules! difference_next {
    ($left_iter: expr, $right_iter: expr) => {{
        loop {
            if let Some(l_item) = $left_iter.peep() {
                if let Some(r_item) = $right_iter.peep() {
                    match l_item.cmp(r_item) {
                        Ordering::Less => {
                            break $left_iter.next();
                        }
                        Ordering::Greater => {
                            $right_iter.advance_until(l_item);
                        }
                        Ordering::Equal => {
                            $left_iter.next();
                            $right_iter.next();
                        }
                    }
                } else {
                    break $left_iter.next();
                }
            } else {
                break None;
            }
        }
    }};
}

#[macro_export]
macro_rules! difference_peep {
    ($left_iter: expr, $right_iter: expr) => {{
        loop {
            if let Some(l_item) = $left_iter.peep() {
                if let Some(r_item) = $right_iter.peep() {
                    match l_item.cmp(r_item) {
                        Ordering::Less => {
                            break Some(l_item);
                        }
                        Ordering::Greater => {
                            $right_iter.advance_until(l_item);
                        }
                        Ordering::Equal => {
                            $left_iter.next();
                            $right_iter.next();
                        }
                    }
                } else {
                    break Some(l_item);
                }
            } else {
                break None;
            }
        }
    }};
}

#[derive(Clone)]
pub struct DifferenceIterator<'a, T: Ord + Clone> {
    pub left_iter: OrdSetOpsIter<'a, T>,
    pub right_iter: OrdSetOpsIter<'a, T>,
}

impl<'a, T: Ord + Clone> Iterator for DifferenceIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        difference_next!(self.left_iter, self.right_iter)
    }
}

impl<'a, T> PeepAdvanceIter<'a, T> for DifferenceIterator<'a, T>
where
    T: 'a + Ord + Clone,
{
    fn peep(&mut self) -> Option<&'a T> {
        difference_peep!(self.left_iter, self.right_iter)
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
