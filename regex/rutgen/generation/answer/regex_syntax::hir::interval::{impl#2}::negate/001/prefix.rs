// Answer 0

#[test]
fn test_negate_empty_interval_set() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound;

    impl TestBound {
        pub fn min_value() -> Self { TestBound }
        pub fn max_value() -> Self { TestBound }
        pub fn decrement(self) -> Self { self }
        pub fn increment(self) -> Self { self }
    }

    #[derive(Clone, Debug, Default)]
    struct TestInterval {
        lower_bound: TestBound,
        upper_bound: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound { self.lower_bound }
        fn upper(&self) -> Self::Bound { self.upper_bound }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper_bound = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![]);
    interval_set.negate();
}

#[test]
fn test_negate_non_empty_interval_set() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct AnotherBound;

    impl AnotherBound {
        pub fn min_value() -> Self { AnotherBound }
        pub fn max_value() -> Self { AnotherBound }
        pub fn decrement(self) -> Self { self }
        pub fn increment(self) -> Self { self }
    }

    #[derive(Clone, Debug, Default)]
    struct AnotherInterval {
        lower_bound: AnotherBound,
        upper_bound: AnotherBound,
    }

    impl Interval for AnotherInterval {
        type Bound = AnotherBound;

        fn lower(&self) -> Self::Bound { self.lower_bound }
        fn upper(&self) -> Self::Bound { self.upper_bound }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper_bound = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut interval_set: IntervalSet<AnotherInterval> = IntervalSet::new(vec![AnotherInterval::default()]);
    interval_set.push(AnotherInterval::default());
    interval_set.negate();
}

