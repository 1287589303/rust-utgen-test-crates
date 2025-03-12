// Answer 0

#[test]
fn test_difference_overlap_exact_bounds() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct TestBound(u32);

    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }

        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }

        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_subset(other) {
                return (None, None);
            }
            let mut result = (None, None);
            if other.lower > self.lower {
                result.0 = Some(TestInterval {
                    lower: self.lower,
                    upper: other.lower.decrement(),
                });
            }
            if other.upper < self.upper {
                result.1 = Some(TestInterval {
                    lower: other.upper.increment(),
                    upper: self.upper,
                });
            }
            result
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![TestInterval {
        lower: TestBound(5),
        upper: TestBound(10),
    }]);
    
    let interval_set_b = IntervalSet::new(vec![TestInterval {
        lower: TestBound(10),
        upper: TestBound(15),
    }]);

    interval_set_a.difference(&interval_set_b);
}

#[test]
fn test_difference_multiple_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct TestBound(u32);

    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }

        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }

        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_subset(other) {
                return (None, None);
            }
            let mut result = (None, None);
            if other.lower > self.lower {
                result.0 = Some(TestInterval {
                    lower: self.lower,
                    upper: other.lower.decrement(),
                });
            }
            if other.upper < self.upper {
                result.1 = Some(TestInterval {
                    lower: other.upper.increment(),
                    upper: self.upper,
                });
            }
            result
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(5),
            upper: TestBound(10),
        },
        TestInterval {
            lower: TestBound(15),
            upper: TestBound(20),
        },
    ]);
    
    let interval_set_b = IntervalSet::new(vec![TestInterval {
        lower: TestBound(10),
        upper: TestBound(15),
    }]);

    interval_set_a.difference(&interval_set_b);
}

#[test]
fn test_difference_no_intersection() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct TestBound(u32);

    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }

        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }

        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_subset(other) {
                return (None, None);
            }
            let mut result = (None, None);
            if other.lower > self.lower {
                result.0 = Some(TestInterval {
                    lower: self.lower,
                    upper: other.lower.decrement(),
                });
            }
            if other.upper < self.upper {
                result.1 = Some(TestInterval {
                    lower: other.upper.increment(),
                    upper: self.upper,
                });
            }
            result
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![TestInterval {
        lower: TestBound(1),
        upper: TestBound(5),
    }]);
    
    let interval_set_b = IntervalSet::new(vec![TestInterval {
        lower: TestBound(6),
        upper: TestBound(10),
    }]);

    interval_set_a.difference(&interval_set_b);
}

