// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

#[macro_export]
macro_rules! are_disjoint {
    ($left_iter: expr, $right_iter: expr) => {{
        loop {
            if let Some(my_item) = $left_iter.peep() {
                if let Some(other_item) = $right_iter.peep() {
                    match my_item.cmp(other_item) {
                        Ordering::Less => {
                            $left_iter.advance_until(other_item);
                        }
                        Ordering::Greater => {
                            $right_iter.advance_until(my_item);
                        }
                        Ordering::Equal => {
                            break false;
                        }
                    }
                } else {
                    break true;
                }
            } else {
                break true;
            }
        }
    }};
}

#[macro_export]
macro_rules! left_is_superset_of_right {
    ($left_iter: expr, $right_iter: expr) => {{
        loop {
            if let Some(my_item) = $left_iter.peep() {
                if let Some(other_item) = $right_iter.peep() {
                    match my_item.cmp(other_item) {
                        Ordering::Less => {
                            $left_iter.advance_until(other_item);
                        }
                        Ordering::Greater => {
                            break false;
                        }
                        Ordering::Equal => {
                            $right_iter.next();
                            $left_iter.next();
                        }
                    }
                } else {
                    break true;
                }
            } else {
                break $right_iter.peep().is_none();
            }
        }
    }};
}

#[macro_export]
macro_rules! left_is_proper_superset_of_right {
    ($left_iter: expr, $right_iter: expr) => {{
        let mut result = false;
        loop {
            if let Some(my_item) = $left_iter.peep() {
                if let Some(other_item) = $right_iter.peep() {
                    match my_item.cmp(other_item) {
                        Ordering::Less => {
                            result = true;
                            $left_iter.advance_until(other_item);
                        }
                        Ordering::Greater => {
                            break false;
                        }
                        Ordering::Equal => {
                            $right_iter.next();
                            $left_iter.next();
                        }
                    }
                } else {
                    break true;
                }
            } else {
                break result && $right_iter.peep().is_none();
            }
        }
    }};
}

#[macro_export]
macro_rules! left_is_subset_of_right {
    ($left_iter: expr, $right_iter: expr) => {{
        loop {
            if let Some(my_item) = $left_iter.peep() {
                if let Some(other_item) = $right_iter.peep() {
                    match my_item.cmp(other_item) {
                        Ordering::Less => {
                            break false;
                        }
                        Ordering::Greater => {
                            $right_iter.advance_until(my_item);
                        }
                        Ordering::Equal => {
                            $right_iter.next();
                            $left_iter.next();
                        }
                    }
                } else {
                    break false;
                }
            } else {
                break true;
            }
        }
    }};
}

#[macro_export]
macro_rules! left_is_proper_subset_of_right {
    ($left_iter: expr, $right_iter: expr) => {{
        let mut result = false;
        loop {
            if let Some(my_item) = $left_iter.peep() {
                if let Some(other_item) = $right_iter.peep() {
                    match my_item.cmp(other_item) {
                        Ordering::Less => {
                            break false;
                        }
                        Ordering::Greater => {
                            result = true;
                            $right_iter.advance_until(my_item);
                        }
                        Ordering::Equal => {
                            $right_iter.next();
                            $left_iter.next();
                        }
                    }
                } else {
                    break false;
                }
            } else {
                break result || $right_iter.peep().is_some();
            }
        }
    }};
}
