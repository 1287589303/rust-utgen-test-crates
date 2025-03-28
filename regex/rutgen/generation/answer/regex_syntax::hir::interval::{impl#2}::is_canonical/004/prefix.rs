// Answer 0

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct TestBound(u32);

impl TestBound {
    fn decrement(&self) -> Self {
        TestBound(self.0 - 1)
    }
    
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }
}

impl Debug for TestBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
struct TestInterval {
    lower: TestBound,
    upper: TestBound,
}

impl Default for TestInterval {
    fn default() -> Self {
        Self {
            lower: TestBound(0),
            upper: TestBound(0),
        }
    }
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
        self.upper.0 + 1 == other.lower.0 || other.upper.0 + 1 == self.lower.0
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || other.upper < self.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_is_canonical_positive() {
    let interval1 = TestInterval {
        lower: TestBound(0),
        upper: TestBound(2),
    };
    let interval2 = TestInterval {
        lower: TestBound(3),
        upper: TestBound(5),
    };
    let interval_set = IntervalSet::<TestInterval> {
        ranges: vec![interval1, interval2],
        folded: false,
    };

    let _ = interval_set.is_canonical();
}

#[test]
fn test_is_canonical_positive_non_contiguous() {
    let interval1 = TestInterval {
        lower: TestBound(10),
        upper: TestBound(20),
    };
    let interval2 = TestInterval {
        lower: TestBound(30),
        upper: TestBound(40),
    };
    let interval_set = IntervalSet::<TestInterval> {
        ranges: vec![interval1, interval2],
        folded: false,
    };

    let _ = interval_set.is_canonical();
}

