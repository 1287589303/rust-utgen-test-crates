// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct TestBound(u32);

impl TestBound {
    fn increment(self) -> Self {
        TestBound(self.0 + 1)
    }

    fn decrement(self) -> Self {
        TestBound(self.0 - 1)
    }
}

impl Bound for TestBound {}

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

    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
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

#[test]
fn test_union_with_non_empty_ranges_and_non_overlapping_intervals() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
    ]);
    set_a.folded = true;

    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(4), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(7) },
    ]);

    set_a.union(&set_b);
}

#[test]
fn test_union_with_different_intervals() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(2), upper: TestBound(4) },
    ]);
    set_a.folded = true;

    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(9) },
    ]);

    set_a.union(&set_b);
}

#[test]
fn test_union_with_multiple_intervals() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
        TestInterval { lower: TestBound(5), upper: TestBound(6) },
    ]);
    set_a.folded = true;

    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
        TestInterval { lower: TestBound(7), upper: TestBound(8) },
    ]);

    set_a.union(&set_b);
}

