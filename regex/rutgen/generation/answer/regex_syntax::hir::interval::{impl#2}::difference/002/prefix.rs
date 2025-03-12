// Answer 0

#[test]
fn test_difference_with_non_empty_self_and_empty_other() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(i32);
    
    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }

        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 >= other.lower.0 || other.upper.0 >= self.lower.0
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_subset(other) {
                return (None, None);
            }
            (Some(self.clone()), None)
        }
    }

    let mut intervals = Vec::new();
    intervals.push(TestInterval { lower: TestBound(0), upper: TestBound(10) });
    intervals.push(TestInterval { lower: TestBound(20), upper: TestBound(30) });
    intervals.push(TestInterval { lower: TestBound(40), upper: TestBound(50) });

    let mut set = IntervalSet::new(intervals);

    let other_set = IntervalSet::new(Vec::<TestInterval>::new());

    set.difference(&other_set);
}

#[test]
fn test_difference_with_non_empty_self_and_empty_other_edge_case() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(i32);
    
    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }

        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 >= other.lower.0 || other.upper.0 >= self.lower.0
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_subset(other) {
                return (None, None);
            }
            (Some(self.clone()), None)
        }
    }

    let mut intervals = Vec::new();
    intervals.push(TestInterval { lower: TestBound(90), upper: TestBound(100) });
    intervals.push(TestInterval { lower: TestBound(110), upper: TestBound(120) });

    let mut set = IntervalSet::new(intervals);

    let other_set = IntervalSet::new(Vec::<TestInterval>::new());

    set.difference(&other_set);
}

