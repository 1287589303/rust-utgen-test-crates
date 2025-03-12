// Answer 0

#[test]
fn test_is_subset_case_1() {
    struct TestInterval {
        lower_bound: char,
        upper_bound: char,
    }

    impl Interval for TestInterval {
        type Bound = char;

        fn lower(&self) -> Self::Bound {
            self.lower_bound
        }

        fn upper(&self) -> Self::Bound {
            self.upper_bound
        }
        
        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower_bound = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper_bound = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, _: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _: &Self) -> bool {
            false
        }
    }

    let interval1 = TestInterval {
        lower_bound: 'a',
        upper_bound: 'c',
    };

    let interval2 = TestInterval {
        lower_bound: 'a',
        upper_bound: 'b',
    };

    interval2.is_subset(&interval1);
}

#[test]
fn test_is_subset_case_2() {
    struct TestInterval {
        lower_bound: char,
        upper_bound: char,
    }

    impl Interval for TestInterval {
        type Bound = char;

        fn lower(&self) -> Self::Bound {
            self.lower_bound
        }

        fn upper(&self) -> Self::Bound {
            self.upper_bound
        }
        
        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower_bound = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper_bound = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, _: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _: &Self) -> bool {
            false
        }
    }

    let interval1 = TestInterval {
        lower_bound: 'b',
        upper_bound: 'd',
    };

    let interval2 = TestInterval {
        lower_bound: 'b',
        upper_bound: 'c',
    };

    interval2.is_subset(&interval1);
}

