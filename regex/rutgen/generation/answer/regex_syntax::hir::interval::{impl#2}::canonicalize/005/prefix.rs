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

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
struct TestInterval {
    lower: TestBound,
    upper: TestBound,
}

impl Interval for TestInterval {
    type Bound = TestBound;

    fn lower(&self) -> Self::Bound { self.lower }
    fn upper(&self) -> Self::Bound { self.upper }
    fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
    fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
    fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
    fn is_contiguous(&self, other: &Self) -> bool { self.upper >= other.lower && self.lower <= other.upper }
    fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
    fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
}

#[test]
fn test_canonicalize_with_overlapping_intervals() {
    let mut set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(3), upper: TestBound(7) },
        TestInterval { lower: TestBound(8), upper: TestBound(10) },
    ]);
    set.canonicalize();
}

#[test]
fn test_canonicalize_with_touching_intervals() {
    let mut set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(3), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(9) },
    ]);
    set.canonicalize();
}

#[test]
fn test_canonicalize_with_completely_overlapping_intervals() {
    let mut set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(2), upper: TestBound(8) },
        TestInterval { lower: TestBound(3), upper: TestBound(5) },
        TestInterval { lower: TestBound(4), upper: TestBound(6) },
    ]);
    set.canonicalize();
}

