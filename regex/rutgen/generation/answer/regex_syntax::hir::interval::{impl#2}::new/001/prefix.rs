// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
struct TestBound(u32);

impl core::cmp::PartialEq for TestBound {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl core::cmp::Eq for TestBound {}

impl core::cmp::Ord for TestBound {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl core::cmp::PartialOrd for TestBound {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait Bound {
    fn increment(&self) -> Self;
    fn decrement(&self) -> Self;
}

impl Bound for TestBound {
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }

    fn decrement(&self) -> Self {
        TestBound(self.0 - 1)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
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

    fn case_fold_simple(&self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
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
}

#[test]
fn test_new_non_empty_overlapping_intervals() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(3), upper: TestBound(7) },
    ];
    let set = IntervalSet::new(intervals);
}

#[test]
fn test_new_non_empty_non_overlapping_intervals() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
    ];
    let set = IntervalSet::new(intervals);
}

#[test]
fn test_new_empty_intervals() {
    let intervals: Vec<TestInterval> = Vec::new();
    let set = IntervalSet::new(intervals);
}

#[test]
fn test_new_intervals_with_same_lower_upper_bounds() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(1) },
        TestInterval { lower: TestBound(2), upper: TestBound(2) },
    ];
    let set = IntervalSet::new(intervals);
}

#[test]
fn test_new_intervals_upper_less_than_lower_bounds() {
    let intervals = vec![
        TestInterval { lower: TestBound(2), upper: TestBound(1) },
        TestInterval { lower: TestBound(4), upper: TestBound(3) },
    ];
    let set = IntervalSet::new(intervals);
}

