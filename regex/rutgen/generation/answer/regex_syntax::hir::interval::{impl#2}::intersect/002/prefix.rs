// Answer 0

#[test]
fn test_intersect_non_empty_self_empty_other() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound {
        value: i32,
    }
    
    impl TestBound {
        fn increment(&self) -> Self {
            TestBound { value: self.value + 1 }
        }
        
        fn decrement(&self) -> Self {
            TestBound { value: self.value - 1 }
        }
    }
    
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestInterval {
        lower_bound: TestBound,
        upper_bound: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

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
            _intervals: &mut Vec<Self>,
        ) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, _other: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut interval_set_self = IntervalSet::new(vec![TestInterval {
        lower_bound: TestBound { value: 1 },
        upper_bound: TestBound { value: 5 },
    }]);
    let interval_set_other = IntervalSet::new(Vec::new());

    interval_set_self.intersect(&interval_set_other);
}

#[test]
fn test_intersect_multiple_ranges_empty_other() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound {
        value: i32,
    }

    impl TestBound {
        fn increment(&self) -> Self {
            TestBound { value: self.value + 1 }
        }

        fn decrement(&self) -> Self {
            TestBound { value: self.value - 1 }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestInterval {
        lower_bound: TestBound,
        upper_bound: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

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
            _intervals: &mut Vec<Self>,
        ) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, _other: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut interval_set_self = IntervalSet::new(vec![
        TestInterval {
            lower_bound: TestBound { value: 1 },
            upper_bound: TestBound { value: 3 },
        },
        TestInterval {
            lower_bound: TestBound { value: 5 },
            upper_bound: TestBound { value: 7 },
        },
    ]);
    let interval_set_other = IntervalSet::new(Vec::new());

    interval_set_self.intersect(&interval_set_other);
}

