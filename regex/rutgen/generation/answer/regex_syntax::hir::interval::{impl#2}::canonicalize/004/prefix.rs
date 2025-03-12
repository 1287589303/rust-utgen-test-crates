// Answer 0

#[test]
fn test_canonicalize_with_overlapping_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
    
    impl TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0.saturating_sub(1))
        }
        
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
            self.upper.0 + 1 >= other.lower.0 && self.lower.0 <= other.upper.0 + 1
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.lower() > other.upper() || self.upper() < other.lower()
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false // Simplified for the test
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(2), upper: TestBound(5) },
    ]);

    interval_set.push(TestInterval { lower: TestBound(4), upper: TestBound(6) });
    interval_set.canonicalize();
}

#[test]
fn test_canonicalize_with_multiple_overlapping_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
    
    impl TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0.saturating_sub(1))
        }
        
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
            self.upper.0 + 1 >= other.lower.0 && self.lower.0 <= other.upper.0 + 1
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.lower() > other.upper() || self.upper() < other.lower()
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false // Simplified for the test
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(4) },
        TestInterval { lower: TestBound(3), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(9) },
    ]);

    interval_set.push(TestInterval { lower: TestBound(5), upper: TestBound(7) });
    interval_set.canonicalize();
}

