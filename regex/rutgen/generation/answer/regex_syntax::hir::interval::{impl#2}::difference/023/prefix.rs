// Answer 0

#[test]
fn test_difference_with_overlap_and_empty_range() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(i32);

    impl TestBound {
        fn upper(&self) -> TestBound { TestBound(self.0) }
        fn lower(&self) -> TestBound { TestBound(self.0) }
        fn increment(&self) -> TestBound { TestBound(self.0 + 1) }
        fn decrement(&self) -> TestBound { TestBound(self.0 - 1) }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { false }
        fn is_intersection_empty(&self, other: &Self) -> bool { false }
        fn is_subset(&self, other: &Self) -> bool { false }

        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            (Some(TestInterval { lower: TestBound(self.lower.0), upper: TestBound(self.upper.0 - 1) }), Some(TestInterval { lower: TestBound(self.upper.0 + 1), upper: TestBound(self.upper.0 + 1) }))
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(8) },
    ]);
    
    let interval_set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(4), upper: TestBound(6) },
        TestInterval { lower: TestBound(8), upper: TestBound(10) },
    ]);
    
    interval_set_a.difference(&interval_set_b);
}

#[test]
fn test_difference_with_overlap_full_consumption() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(i32);

    impl TestBound {
        fn upper(&self) -> TestBound { TestBound(self.0) }
        fn lower(&self) -> TestBound { TestBound(self.0) }
        fn increment(&self) -> TestBound { TestBound(self.0 + 1) }
        fn decrement(&self) -> TestBound { TestBound(self.0 - 1) }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { false }
        fn is_intersection_empty(&self, other: &Self) -> bool { false }
        fn is_subset(&self, other: &Self) -> bool { false }

        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            (Some(TestInterval { lower: TestBound(self.lower.0), upper: TestBound(self.upper.0 - 1) }), None)
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(3), upper: TestBound(7) },
        TestInterval { lower: TestBound(8), upper: TestBound(12) },
    ]);
    
    let interval_set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(8) },
        TestInterval { lower: TestBound(10), upper: TestBound(15) },
    ]);
    
    interval_set_a.difference(&interval_set_b);
}

