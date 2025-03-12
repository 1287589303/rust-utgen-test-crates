// Answer 0

#[test]
fn test_negate_with_empty_range() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(i32);
    
    impl Bound for TestBound {
        fn min_value() -> Self { TestBound(i32::MIN) }
        fn max_value() -> Self { TestBound(i32::MAX) }
        fn decrement(&self) -> Self { TestBound(self.0 - 1) }
        fn increment(&self) -> Self { TestBound(self.0 + 1) }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        fn case_fold_simple(&self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![TestInterval { lower: TestBound::min_value(), upper: TestBound::max_value() }]);
    interval_set.negate();
}

#[test]
fn test_negate_with_lower_bound_equal_min_value() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(i32);
    
    impl Bound for TestBound {
        fn min_value() -> Self { TestBound(i32::MIN) }
        fn max_value() -> Self { TestBound(i32::MAX) }
        fn decrement(&self) -> Self { TestBound(self.0 - 1) }
        fn increment(&self) -> Self { TestBound(self.0 + 1) }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        fn case_fold_simple(&self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![TestInterval { lower: TestBound::min_value(), upper: TestBound(10) }]);
    interval_set.negate();
}

#[test]
fn test_negate_with_single_interval() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(i32);

    impl Bound for TestBound {
        fn min_value() -> Self { TestBound(i32::MIN) }
        fn max_value() -> Self { TestBound(i32::MAX) }
        fn decrement(&self) -> Self { TestBound(self.0 - 1) }
        fn increment(&self) -> Self { TestBound(self.0 + 1) }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        fn case_fold_simple(&self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![TestInterval { lower: TestBound(10), upper: TestBound(20) }]);
    interval_set.negate();
}

#[test]
fn test_negate_with_multiple_intervals() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(i32);

    impl Bound for TestBound {
        fn min_value() -> Self { TestBound(i32::MIN) }
        fn max_value() -> Self { TestBound(i32::MAX) }
        fn decrement(&self) -> Self { TestBound(self.0 - 1) }
        fn increment(&self) -> Self { TestBound(self.0 + 1) }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        fn case_fold_simple(&self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(10), upper: TestBound(20) },
        TestInterval { lower: TestBound(21), upper: TestBound(30) }
    ]);
    interval_set.negate();
}

