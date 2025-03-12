// Answer 0

#[test]
fn test_canonicalize_with_non_overlapping_intervals() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(u32);
    
    impl Debug for TestBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

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
            self.upper < other.lower || other.upper < self.lower
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(0), upper: TestBound(1) },
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
    ]);

    interval_set.canonicalize();
}

#[test]
fn test_canonicalize_with_mergeable_intervals() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(u32);

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

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
            (self.lower < other.upper && self.upper > other.lower)
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            !self.is_contiguous(other)
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(0), upper: TestBound(2) },
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(5), upper: TestBound(6) },
    ]);

    interval_set.canonicalize();
}

