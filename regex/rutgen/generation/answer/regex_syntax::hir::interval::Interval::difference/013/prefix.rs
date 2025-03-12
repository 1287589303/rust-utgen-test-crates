// Answer 0

#[test]
fn test_difference_case1() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: usize,
        upper: usize,
    }

    impl Interval for TestInterval {
        type Bound = usize;

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

        fn case_fold_simple(
            &self,
            intervals: &mut Vec<Self>,
        ) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper + 1 >= other.lower && other.upper + 1 >= self.lower
        }

        fn decrement(&self) -> Self::Bound {
            self.lower.saturating_sub(1)
        }

        fn increment(&self) -> Self::Bound {
            self.upper.saturating_add(1)
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            let (lower1, upper1) = (self.lower(), self.upper());
            let (lower2, upper2) = (other.lower(), other.upper());
            cmp::max(lower1, lower2) > cmp::min(upper1, upper2)
        }

        fn is_subset(&self, other: &Self) -> bool {
            (other.lower() <= self.lower() && self.upper() <= other.upper())
        }
    }

    let mut a = TestInterval { lower: 10, upper: 20 };
    let b = TestInterval { lower: 20, upper: 30 };
    let result = a.difference(&b);
}

#[test]
fn test_difference_case2() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: usize,
        upper: usize,
    }

    impl Interval for TestInterval {
        type Bound = usize;

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

        fn case_fold_simple(
            &self,
            intervals: &mut Vec<Self>,
        ) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper + 1 >= other.lower && other.upper + 1 >= self.lower
        }

        fn decrement(&self) -> Self::Bound {
            self.lower.saturating_sub(1)
        }

        fn increment(&self) -> Self::Bound {
            self.upper.saturating_add(1)
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            let (lower1, upper1) = (self.lower(), self.upper());
            let (lower2, upper2) = (other.lower(), other.upper());
            cmp::max(lower1, lower2) > cmp::min(upper1, upper2)
        }

        fn is_subset(&self, other: &Self) -> bool {
            (other.lower() <= self.lower() && self.upper() <= other.upper())
        }
    }

    let mut a = TestInterval { lower: 5, upper: 15 };
    let b = TestInterval { lower: 15, upper: 25 };
    let result = a.difference(&b);
}

