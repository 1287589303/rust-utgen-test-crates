// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct TestBound(u32);

impl Bound for TestBound {
    fn decrement(&self) -> TestBound {
        TestBound(self.0.saturating_sub(1))
    }

    fn increment(&self) -> TestBound {
        TestBound(self.0 + 1)
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
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
        self.upper() >= other.lower() && self.lower() <= other.upper()
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.lower() > other.upper() || self.upper() < other.lower()
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_push_single_item_interval() {
    let mut set = IntervalSet::new(vec![]);
    let interval = TestInterval {
        lower: TestBound(5),
        upper: TestBound(5),
    };
    set.push(interval);
}

#[test]
fn test_push_non_contiguous_intervals() {
    let mut set = IntervalSet::new(vec![]);
    
    let interval1 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(2),
    };
    let interval2 = TestInterval {
        lower: TestBound(4),
        upper: TestBound(5),
    };
    
    set.push(interval1);
    set.push(interval2);
}

#[test]
fn test_push_contiguous_intervals() {
    let mut set = IntervalSet::new(vec![]);
    
    let interval1 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(3),
    };
    let interval2 = TestInterval {
        lower: TestBound(3),
        upper: TestBound(5),
    };
    
    set.push(interval1);
    set.push(interval2);
}

#[test]
fn test_push_overlapping_intervals() {
    let mut set = IntervalSet::new(vec![]);
    
    let interval1 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(5),
    };
    let interval2 = TestInterval {
        lower: TestBound(3),
        upper: TestBound(7),
    };
    
    set.push(interval1);
    set.push(interval2);
}

#[test]
fn test_push_empty_and_full_intervals() {
    let mut set = IntervalSet::new(vec![]);
    
    let empty_interval = TestInterval {
        lower: TestBound(3),
        upper: TestBound(3),
    };
    let full_interval = TestInterval {
        lower: TestBound(1),
        upper: TestBound(10),
    };
    
    set.push(empty_interval);
    set.push(full_interval);
}

#[test]
fn test_push_min_max_intervals() {
    let mut set = IntervalSet::new(vec![]);
    
    let min_interval = TestInterval {
        lower: TestBound(0),
        upper: TestBound(0),
    };
    let max_interval = TestInterval {
        lower: TestBound(u32::MAX),
        upper: TestBound(u32::MAX),
    };
    
    set.push(min_interval);
    set.push(max_interval);
}

