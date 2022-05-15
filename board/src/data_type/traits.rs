use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

pub trait Overlapable {
    /// Iterate over the overlap of 2 ranges.
    fn iter_overlap(&self, other: &Self) -> Self;
}

impl Overlapable for RangeInclusive<usize> {
    fn iter_overlap(&self, other: &Self) -> Self {
        let lower = max(*self.start(), *other.start());
        let upper = min(*self.end(), *other.end());

        // RangeInclusive has a nice *feature* that it generates nothing if
        // lower bound is greater than upper bound
        lower..=upper
    }
}

#[cfg(test)]
mod bound_aware_test {
    use super::*;

    #[test]
    fn no_overlap() {
        let low = 0..=5;
        let high = 6..=10;
        let low_rev = (0..=5).rev();
        let high_rev = (6..=10).rev();

        let l_h = low.iter_overlap(&high);
        let l_hr = low.iter_overlap(&high_rev);
        let lr_h = low_rev.iter_overlap(&high);
        let lr_hr = low_rev.iter_overlap(&high_rev);

        assert_eq!(l_h.count(), 0);
        assert_eq!(lr_h.count(), 0);
        assert_eq!(l_hr.count(), 0);
        assert_eq!(lr_hr.count(), 0);
    }

    #[test]
    fn single_point() {
        let single = 5..=5;
        let another = 10..=10;
        let same = 5..=5;
        let not_contain = 0..=4;
        let contain = 2..=6;

        assert_eq!(single.iter_overlap(&not_contain).count(), 0);
        assert_eq!(single.iter_overlap(&another).count(), 0);
        assert_eq!(single.iter_overlap(&same).count(), 1);
        assert_eq!(single.iter_overlap(&contain).count(), 1);
    }

    #[test]
    fn overlap() {
        let first = 0..=5;
        let first_rev = (0..=5).rev();
        let second = 5..=10;
        let second_rev = (5..=10).rev();

        assert_eq!(first.iter_overlap(&second).count(), 1);
        assert_eq!(first.iter_overlap(&second_rev).count(), 1);
        assert_eq!(first_rev.iter_overlap(&second).count(), 1);
        assert_eq!(first_rev.iter_overlap(&second_rev).count(), 1);
    }
}