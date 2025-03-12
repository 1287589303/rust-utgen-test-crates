// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
struct TestBound(i32);

impl TestBound {
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }
    
    fn decrement(&self) -> Self {
        TestBound(self.0 - 1)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
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

    fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
        Ok(())
    }

    fn is_contiguous(&self, other: &Self) -> bool {
        self.is_intersection_empty(other)
    }

    fn is_subset(&self, other: &Self) -> bool {
        (self.lower() >= other.lower()) && (self.upper() <= other.upper())
    }
}

#[test]
fn test_is_intersection_empty_overlapping() {
    let interval1 = TestInterval {
        lower_bound: TestBound(1),
        upper_bound: TestBound(5),
    };
    let interval2 = TestInterval {
        lower_bound: TestBound(4),
        upper_bound: TestBound(6),
    };
    interval1.is_intersection_empty(&interval2);
}

#[test]
fn test_is_intersection_empty_non_overlapping() {
    let interval1 = TestInterval {
        lower_bound: TestBound(1),
        upper_bound: TestBound(3),
    };
    let interval2 = TestInterval {
        lower_bound: TestBound(4),
        upper_bound: TestBound(6),
    };
    interval1.is_intersection_empty(&interval2);
}

#[test]
fn test_is_intersection_empty_identical() {
    let interval1 = TestInterval {
        lower_bound: TestBound(2),
        upper_bound: TestBound(4),
    };
    let interval2 = TestInterval {
        lower_bound: TestBound(2),
        upper_bound: TestBound(4),
    };
    interval1.is_intersection_empty(&interval2);
}

