// Answer 0

#[test]
fn test_case_fold_simple_empty_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MockInterval;

    impl Interval for MockInterval {
        type Bound = char;
        fn lower(&self) -> Self::Bound { 'a' }
        fn upper(&self) -> Self::Bound { 'z' }
        fn set_lower(&mut self, _: Self::Bound) {}
        fn set_upper(&mut self, _: Self::Bound) {}
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::<MockInterval> {
        ranges: Vec::new(),
        folded: false,
    };
    let result = interval_set.case_fold_simple();
}

#[test]
fn test_case_fold_simple_populated_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MockInterval;

    impl Interval for MockInterval {
        type Bound = char;
        fn lower(&self) -> Self::Bound { 'a' }
        fn upper(&self) -> Self::Bound { 'z' }
        fn set_lower(&mut self, _: Self::Bound) {}
        fn set_upper(&mut self, _: Self::Bound) {}
        fn case_fold_simple(&self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            intervals.push(*self);
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::<MockInterval> {
        ranges: vec![MockInterval::default()],
        folded: false,
    };
    let result = interval_set.case_fold_simple();
}

