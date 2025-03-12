// Answer 0

#[test]
fn test_difference_non_overlapping() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bound(i32);
    
    impl Bound {
        fn upper(&self) -> i32 {
            self.0 + 1
        }

        fn lower(&self) -> i32 {
            self.0
        }

        fn increment(self) -> Bound {
            Bound(self.0 + 1)
        }

        fn decrement(self) -> Bound {
            Bound(self.0 - 1)
        }
    }
    
    impl Interval for Bound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound {
            *self
        }

        fn upper(&self) -> Self::Bound {
            *self
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut self_set = IntervalSet::new(vec![Bound(1), Bound(3)]);
    let other_set = IntervalSet::new(vec![Bound(1), Bound(2)]);

    self_set.difference(&other_set);
}

#[test]
fn test_difference_intersecting_ranges() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bound(i32);
    
    impl Bound {
        fn upper(&self) -> i32 {
            self.0 + 2
        }

        fn lower(&self) -> i32 {
            self.0
        }

        fn increment(self) -> Bound {
            Bound(self.0 + 1)
        }

        fn decrement(self) -> Bound {
            Bound(self.0 - 1)
        }
    }
    
    impl Interval for Bound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound {
            *self
        }

        fn upper(&self) -> Self::Bound {
            *self
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut self_set = IntervalSet::new(vec![Bound(1), Bound(4)]);
    let other_set = IntervalSet::new(vec![Bound(2), Bound(3)]);

    self_set.difference(&other_set);
}

#[test]
fn test_difference_full_coverage() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bound(i32);
    
    impl Bound {
        fn upper(&self) -> i32 {
            self.0 + 3
        }

        fn lower(&self) -> i32 {
            self.0
        }

        fn increment(self) -> Bound {
            Bound(self.0 + 1)
        }

        fn decrement(self) -> Bound {
            Bound(self.0 - 1)
        }
    }
    
    impl Interval for Bound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound {
            *self
        }

        fn upper(&self) -> Self::Bound {
            *self
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut self_set = IntervalSet::new(vec![Bound(1), Bound(5)]);
    let other_set = IntervalSet::new(vec![Bound(5), Bound(6)]);

    self_set.difference(&other_set);
}

