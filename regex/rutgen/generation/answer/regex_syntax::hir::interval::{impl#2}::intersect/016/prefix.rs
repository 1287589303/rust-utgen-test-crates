// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct TestBound(u32);

impl TestBound {
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }
    fn decrement(&self) -> Self {
        TestBound(self.0 - 1)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

    fn case_fold_simple(
        &self,
        intervals: &mut Vec<Self>,
    ) -> Result<(), unicode::CaseFoldError> {
        Ok(())
    }

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_intersect_non_empty_sets_with_overlapping_intervals() {
    let mut self_intervals = vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
        TestInterval {
            lower: TestBound(6),
            upper: TestBound(10),
        },
    ];
    
    let other_intervals = vec![
        TestInterval {
            lower: TestBound(4),
            upper: TestBound(10),
        },
        TestInterval {
            lower: TestBound(11),
            upper: TestBound(15),
        },
    ];

    let mut self_set = IntervalSet::new(self_intervals);
    let other_set = IntervalSet::new(other_intervals);

    self_set.intersect(&other_set);
}

#[test]
fn test_intersect_with_same_upper_bounds() {
    let mut self_intervals = vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
        TestInterval {
            lower: TestBound(6),
            upper: TestBound(5),
        },
    ];

    let other_intervals = vec![
        TestInterval {
            lower: TestBound(4),
            upper: TestBound(5),
        },
        TestInterval {
            lower: TestBound(5),
            upper: TestBound(10),
        },
    ];

    let mut self_set = IntervalSet::new(self_intervals);
    let other_set = IntervalSet::new(other_intervals);

    self_set.intersect(&other_set);
}

#[test]
fn test_intersect_with_folding_false() {
    let mut self_intervals = vec![
        TestInterval {
            lower: TestBound(2),
            upper: TestBound(5),
        },
        TestInterval {
            lower: TestBound(5),
            upper: TestBound(7),
        },
    ];

    let other_intervals = vec![
        TestInterval {
            lower: TestBound(5),
            upper: TestBound(8),
        },
        TestInterval {
            lower: TestBound(6),
            upper: TestBound(10),
        },
    ];

    let mut self_set = IntervalSet::new(self_intervals);
    let other_set = IntervalSet::new(other_intervals);
    self_set.folded = false;

    self_set.intersect(&other_set);
}

