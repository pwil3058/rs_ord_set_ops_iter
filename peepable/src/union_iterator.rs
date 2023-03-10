// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::cmp::Ordering;

use super::{OrdSetIterSetOpsIterator, PeepAdvanceIter};

#[macro_export]
macro_rules! union_next {
    ($left_iter: expr, $right_iter: expr) => {{
        if let Some(l_item) = $left_iter.peep() {
            if let Some(r_item) = $right_iter.peep() {
                match l_item.cmp(r_item) {
                    Ordering::Less => $left_iter.next(),
                    Ordering::Greater => $right_iter.next(),
                    Ordering::Equal => {
                        $right_iter.next();
                        $left_iter.next()
                    }
                }
            } else {
                $left_iter.next()
            }
        } else {
            $right_iter.next()
        }
    }};
}

#[macro_export]
macro_rules! union_peep {
    ($left_iter: expr, $right_iter: expr) => {{
        if let Some(l_item) = $left_iter.peep() {
            if let Some(r_item) = $right_iter.peep() {
                match l_item.cmp(r_item) {
                    Ordering::Less | Ordering::Equal => Some(l_item),
                    Ordering::Greater => Some(r_item),
                }
            } else {
                Some(l_item)
            }
        } else {
            $right_iter.peep()
        }
    }};
}

#[derive(Clone)]
pub struct UnionIterator<'a, T: Ord + Clone> {
    left_iter: Box<dyn PeepAdvanceIter<'a, T> + 'a>,
    right_iter: Box<dyn PeepAdvanceIter<'a, T> + 'a>,
}

impl<'a, T: Ord + Clone> UnionIterator<'a, T> {
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

impl<'a, T: Ord + Clone> Iterator for UnionIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        union_next!(self.left_iter, self.right_iter)
    }
}

impl<'a, T> PeepAdvanceIter<'a, T> for UnionIterator<'a, T>
where
    T: 'a + Ord + Clone,
{
    fn peep(&mut self) -> Option<&'a T> {
        union_peep!(self.left_iter, self.right_iter)
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

impl<'a, T: 'a + Ord + Clone + Default> OrdSetIterSetOpsIterator<'a, T> for UnionIterator<'a, T> {}
