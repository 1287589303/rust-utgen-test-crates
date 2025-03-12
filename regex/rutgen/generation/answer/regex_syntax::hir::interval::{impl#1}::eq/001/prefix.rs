// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord)]
struct TestBound(u32);

impl TestBound {
    fn decrement(self) -> Self {
        TestBound(self.0.saturating_sub(1))
    }

    fn increment(self) -> Self {
        TestBound(self.0 + 1)
    }
}

#[derive(Clone, Debug)]
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
        false
    }

    fn is_intersection_empty(&self, _other: &Self) -> bool {
        false
    }

    fn is_subset(&self, _other: &Self) -> bool {
        false
    }
}

#[test]
fn test_eq_both_empty() {
    let set1 = IntervalSet::<TestInterval> { ranges: Vec::new(), folded: false };
    let set2 = IntervalSet::<TestInterval> { ranges: Vec::new(), folded: false };
    let _ = set1.eq(&set2);
}

#[test]
fn test_eq_identical_non_overlapping() {
    let interval1 = TestInterval { lower: TestBound(1), upper: TestBound(2) };
    let interval2 = TestInterval { lower: TestBound(3), upper: TestBound(4) };
    let set1 = IntervalSet::<TestInterval> { ranges: vec![interval1, interval2], folded: true };
    let set2 = IntervalSet::<TestInterval> { ranges: vec![interval1, interval2], folded: true };
    let _ = set1.eq(&set2);
}

#[test]
fn test_eq_identical_overlapping() {
    let interval1 = TestInterval { lower: TestBound(1), upper: TestBound(5) };
    let interval2 = TestInterval { lower: TestBound(3), upper: TestBound(7) };
    let set1 = IntervalSet::<TestInterval> { ranges: vec![interval1, interval2], folded: true };
    let set2 = IntervalSet::<TestInterval> { ranges: vec![interval1, interval2], folded: true };
    let _ = set1.eq(&set2);
}

#[test]
fn test_eq_different() {
    let interval1 = TestInterval { lower: TestBound(1), upper: TestBound(3) };
    let interval2 = TestInterval { lower: TestBound(5), upper: TestBound(7) };
    let set1 = IntervalSet::<TestInterval> { ranges: vec![interval1], folded: true };
    let set2 = IntervalSet::<TestInterval> { ranges: vec![interval2], folded: true };
    let _ = set1.eq(&set2);
}

#[test]
fn test_eq_different_folded_states() {
    let interval1 = TestInterval { lower: TestBound(1), upper: TestBound(3) };
    let set1 = IntervalSet::<TestInterval> { ranges: vec![interval1], folded: true };
    let set2 = IntervalSet::<TestInterval> { ranges: vec![interval1], folded: false };
    let _ = set1.eq(&set2);
}

