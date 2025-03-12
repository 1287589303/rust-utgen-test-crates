// Answer 0

#[test]
fn test_intersect_non_empty_sets_with_equal_upper_bounds() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bounds(i32);

    impl core::fmt::Display for Bounds {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Bound for Bounds {
        fn decrement(&self) -> Self {
            Bounds(self.0 - 1)
        }
        fn increment(&self) -> Self {
            Bounds(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct Interval {
        lower: Bounds,
        upper: Bounds,
    }

    impl Interval {
        fn new(lower: Bounds, upper: Bounds) -> Self {
            Interval { lower, upper }
        }
    }

    impl Interval for Interval {
        type Bound = Bounds;

        fn lower(&self) -> Self::Bound {
            self.lower
        }

        fn upper(&self) -> Self::Bound {
            self.upper
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut self_intervals = IntervalSet::new(vec![
        Interval::new(Bounds(1), Bounds(5)),
    ]);

    let other_intervals = IntervalSet::new(vec![
        Interval::new(Bounds(3), Bounds(5)),
    ]);

    self_intervals.intersect(&other_intervals);
}

#[test]
fn test_intersect_non_empty_sets_with_equal_upper_bounds_no_iteration() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bounds(i32);

    impl core::fmt::Display for Bounds {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Bound for Bounds {
        fn decrement(&self) -> Self {
            Bounds(self.0 - 1)
        }
        fn increment(&self) -> Self {
            Bounds(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct Interval {
        lower: Bounds,
        upper: Bounds,
    }

    impl Interval {
        fn new(lower: Bounds, upper: Bounds) -> Self {
            Interval { lower, upper }
        }
    }

    impl Interval for Interval {
        type Bound = Bounds;

        fn lower(&self) -> Self::Bound {
            self.lower
        }

        fn upper(&self) -> Self::Bound {
            self.upper
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut self_intervals = IntervalSet::new(vec![
        Interval::new(Bounds(1), Bounds(5)),
        Interval::new(Bounds(6), Bounds(10)),
    ]);

    let other_intervals = IntervalSet::new(vec![
        Interval::new(Bounds(5), Bounds(5)),
        Interval::new(Bounds(7), Bounds(10)),
    ]);

    self_intervals.intersect(&other_intervals);
}

