// Answer 0

#[test]
fn test_symmetric_difference_non_empty_overlapping_non_overlapping() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct IntInterval {
        lower: i32,
        upper: i32,
    }

    impl Interval for IntInterval {
        type Bound = i32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        IntInterval { lower: 1, upper: 3 },
        IntInterval { lower: 4, upper: 6 },
    ]);

    let set_b = IntervalSet::new(vec![
        IntInterval { lower: 2, upper: 5 },
        IntInterval { lower: 7, upper: 8 },
    ]);

    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_empty_sets() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct IntInterval {
        lower: i32,
        upper: i32,
    }

    impl Interval for IntInterval {
        type Bound = i32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![]);

    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_complete_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct IntInterval {
        lower: i32,
        upper: i32,
    }

    impl Interval for IntInterval {
        type Bound = i32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        IntInterval { lower: 1, upper: 5 },
    ]);

    let set_b = IntervalSet::new(vec![
        IntInterval { lower: 1, upper: 5 },
    ]);

    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_identical_intervals() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct IntInterval {
        lower: i32,
        upper: i32,
    }

    impl Interval for IntInterval {
        type Bound = i32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        IntInterval { lower: 2, upper: 4 },
    ]);

    let set_b = IntervalSet::new(vec![
        IntInterval { lower: 2, upper: 4 },
    ]);

    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_single_point_intervals() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct IntInterval {
        lower: i32,
        upper: i32,
    }

    impl Interval for IntInterval {
        type Bound = i32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        IntInterval { lower: 5, upper: 5 },
    ]);

    let set_b = IntervalSet::new(vec![
        IntInterval { lower: 5, upper: 5 },
        IntInterval { lower: 10, upper: 12 },
    ]);

    set_a.symmetric_difference(&set_b);
}

