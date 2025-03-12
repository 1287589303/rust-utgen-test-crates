// Answer 0

#[test]
fn test_case_fold_simple_already_folded() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
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

        fn case_fold_simple(
            &self,
            intervals: &mut Vec<Self>,
        ) -> Result<(), unicode::CaseFoldError> {
            let new_lower = self.lower()
                .to_ascii_lowercase();
            let new_upper = self.upper()
                .to_ascii_uppercase();
            intervals.push(TestInterval {
                lower_bound: new_lower,
                upper_bound: new_upper,
            });
            Ok(())
        }

        fn is_contiguous(&self, _: &Self) -> bool { unimplemented!() }
        fn is_intersection_empty(&self, _: &Self) -> bool { unimplemented!() }
        fn is_subset(&self, _: &Self) -> bool { unimplemented!() }
    }

    let mut interval_set = IntervalSet {
        ranges: vec![TestInterval { lower_bound: 'a', upper_bound: 'z' }],
        folded: true,
    };

    let result = interval_set.case_fold_simple();
}

#[test]
fn test_case_fold_simple_non_empty_folded() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
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

        fn case_fold_simple(
            &self,
            intervals: &mut Vec<Self>,
        ) -> Result<(), unicode::CaseFoldError> {
            let new_lower = self.lower()
                .to_ascii_lowercase();
            let new_upper = self.upper()
                .to_ascii_uppercase();
            intervals.push(TestInterval {
                lower_bound: new_lower,
                upper_bound: new_upper,
            });
            Ok(())
        }

        fn is_contiguous(&self, _: &Self) -> bool { unimplemented!() }
        fn is_intersection_empty(&self, _: &Self) -> bool { unimplemented!() }
        fn is_subset(&self, _: &Self) -> bool { unimplemented!() }
    }

    let mut interval_set = IntervalSet {
        ranges: vec![
            TestInterval { lower_bound: 'a', upper_bound: 'z' },
            TestInterval { lower_bound: 'A', upper_bound: 'Z' },
        ],
        folded: true,
    };

    let result = interval_set.case_fold_simple();
}

