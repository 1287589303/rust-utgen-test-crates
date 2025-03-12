// Answer 0

#[test]
fn test_negate_empty_ranges() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct MockBound;

    impl Bound for MockBound {
        fn min_value() -> Self { MockBound }
        fn max_value() -> Self { MockBound }
        fn decrement(&self) -> Self { MockBound }
        fn increment(&self) -> Self { MockBound }
    }

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![
        MockInterval { lower: MockBound::min_value(), upper: MockBound::max_value() }
    ]);

    interval_set.negate();
}

#[test]
fn test_negate_with_first_lower_bound() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct MockBound;

    impl Bound for MockBound {
        fn min_value() -> Self { MockBound }
        fn max_value() -> Self { MockBound }
        fn decrement(&self) -> Self { MockBound }
        fn increment(&self) -> Self { MockBound }
    }

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![
        MockInterval { lower: MockBound::min_value(), upper: MockBound::max_value() },
        MockInterval { lower: MockBound::min_value(), upper: MockBound::max_value() }
    ]);

    interval_set.negate();
}

#[test]
fn test_negate_with_range_end() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct MockBound;

    impl Bound for MockBound {
        fn min_value() -> Self { MockBound }
        fn max_value() -> Self { MockBound }
        fn decrement(&self) -> Self { MockBound }
        fn increment(&self) -> Self { MockBound }
    }

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![
        MockInterval { lower: MockBound::min_value(), upper: MockBound::max_value() },
        MockInterval { lower: MockBound::min_value(), upper: MockBound::max_value() },
        MockInterval { lower: MockBound::min_value(), upper: MockBound::max_value() }
    ]);

    interval_set.negate();
}

