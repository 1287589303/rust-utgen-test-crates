// Answer 0

#[test]
fn test_is_subset_equal_bounds() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: i32,
        upper: i32,
    }
    impl Interval for TestInterval {
        type Bound = i32;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { (self.upper >= other.lower && other.upper >= self.lower) }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || other.upper < self.lower }
    }

    let interval1 = TestInterval { lower: 5, upper: 5 };
    let interval2 = TestInterval { lower: 5, upper: 5 };
    let result = interval1.is_subset(&interval2);
}

#[test]
fn test_is_subset_lower_lesser() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: i32,
        upper: i32,
    }
    impl Interval for TestInterval {
        type Bound = i32;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { (self.upper >= other.lower && other.upper >= self.lower) }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || other.upper < self.lower }
    }

    let interval1 = TestInterval { lower: 6, upper: 7 };
    let interval2 = TestInterval { lower: 6, upper: 6 };
    let result = interval1.is_subset(&interval2);
}

#[test]
fn test_is_subset_upper_lesser() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: i32,
        upper: i32,
    }
    impl Interval for TestInterval {
        type Bound = i32;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { (self.upper >= other.lower && other.upper >= self.lower) }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || other.upper < self.lower }
    }

    let interval1 = TestInterval { lower: 4, upper: 6 };
    let interval2 = TestInterval { lower: 6, upper: 7 };
    let result = interval1.is_subset(&interval2);
}

