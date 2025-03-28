// Answer 0

#[test]
fn test_is_canonical_contiguous_intervals() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: i32,
        upper: i32,
    }

    impl TestInterval {
        fn new(lower: i32, upper: i32) -> Self {
            TestInterval { lower, upper }
        }
    }

    impl Interval for TestInterval {
        type Bound = i32;

        fn lower(&self) -> Self::Bound {
            self.lower
        }

        fn upper(&self) -> Self::Bound {
            self.upper
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper + 1 == other.lower
        }

        fn is_intersection_empty(&self, _: &Self) -> bool {
            false
        }

        fn is_subset(&self, _: &Self) -> bool {
            false
        }
    }

    let interval1 = TestInterval::new(1, 3);
    let interval2 = TestInterval::new(4, 6);
    let mut interval_set = IntervalSet::new(vec![interval1, interval2]);

    let result = interval_set.is_canonical();
}

