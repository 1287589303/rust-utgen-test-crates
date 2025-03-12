// Answer 0

#[test]
fn test_is_canonical_with_equal_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);

    impl crate::Bound for TestBound {
        fn decrement(self) -> Self { TestBound(self.0 - 1) }
        fn increment(self) -> Self { TestBound(self.0 + 1) }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
        fn is_contiguous(&self, other: &Self) -> bool { self.upper == other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || other.upper < self.lower }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let interval1 = TestInterval { lower: TestBound(5), upper: TestBound(5) };
    let interval2 = TestInterval { lower: TestBound(5), upper: TestBound(5) };
    let mut interval_set = IntervalSet::new(vec![interval1, interval2]);

    let result = interval_set.is_canonical();
    assert_eq!(result, false);
}

#[test]
fn test_is_canonical_with_contiguous_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);

    impl crate::Bound for TestBound {
        fn decrement(self) -> Self { TestBound(self.0 - 1) }
        fn increment(self) -> Self { TestBound(self.0 + 1) }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
        fn is_contiguous(&self, other: &Self) -> bool { self.upper == other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || other.upper < self.lower }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let interval1 = TestInterval { lower: TestBound(5), upper: TestBound(6) };
    let interval2 = TestInterval { lower: TestBound(6), upper: TestBound(7) };
    let mut interval_set = IntervalSet::new(vec![interval1, interval2]);

    let result = interval_set.is_canonical();
    assert_eq!(result, false);
}

