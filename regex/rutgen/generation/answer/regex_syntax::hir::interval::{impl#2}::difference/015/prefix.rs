// Answer 0

#[test]
fn test_difference_with_intersecting_ranges() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: usize,
        upper: usize,
    }

    impl Bound for usize {}

    impl Interval for TestInterval {
        type Bound = usize;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper + 1 >= other.lower && self.lower <= other.upper + 1
        }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 5 },
    ]);

    let set_b = IntervalSet::new(vec![
        TestInterval { lower: 5, upper: 10 },
    ]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_with_non_intersecting_ranges() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: usize,
        upper: usize,
    }

    impl Bound for usize {}

    impl Interval for TestInterval {
        type Bound = usize;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper + 1 >= other.lower && self.lower <= other.upper + 1
        }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 3 },
        TestInterval { lower: 4, upper: 6 },
    ]);

    let set_b = IntervalSet::new(vec![
        TestInterval { lower: 7, upper: 10 },
    ]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_with_adjacent_ranges() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: usize,
        upper: usize,
    }

    impl Bound for usize {}

    impl Interval for TestInterval {
        type Bound = usize;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper + 1 >= other.lower && self.lower <= other.upper + 1
        }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 2 },
    ]);

    let set_b = IntervalSet::new(vec![
        TestInterval { lower: 2, upper: 3 },
    ]);

    set_a.difference(&set_b);
}

