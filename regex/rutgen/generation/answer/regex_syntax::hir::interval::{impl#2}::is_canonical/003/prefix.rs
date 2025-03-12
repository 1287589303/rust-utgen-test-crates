// Answer 0

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct TestInterval {
    lower: u32,
    upper: u32,
}

impl TestInterval {
    fn new(lower: u32, upper: u32) -> Self {
        Self { lower, upper }
    }
}

impl super::Interval for TestInterval {
    type Bound = u32;

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
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.lower > other.upper || self.upper < other.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_is_canonical_true() {
    let interval1 = TestInterval::new(1, 2);
    let interval2 = TestInterval::new(3, 4);
    let intervals = vec![interval1, interval2];
    let interval_set = super::IntervalSet::<TestInterval> { ranges: intervals, folded: false };
    assert!(interval_set.is_canonical());
}

#[test]
fn test_is_canonical_true_with_non_overlapping_intervals() {
    let interval1 = TestInterval::new(10, 15);
    let interval2 = TestInterval::new(20, 25);
    let intervals = vec![interval1, interval2];
    let interval_set = super::IntervalSet::<TestInterval> { ranges: intervals, folded: false };
    assert!(interval_set.is_canonical());
}

