// Answer 0

#[test]
fn test_difference_non_overlapping() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct Bound(u32);
    
    impl Bound {
        fn upper(&self) -> u32 { self.0 }
        fn lower(&self) -> u32 { self.0 }
        fn decrement(&self) -> Bound { Bound(self.0 - 1) }
        fn increment(&self) -> Bound { Bound(self.0 + 1) }
    }
    
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for TestInterval {
        type Bound = Bound;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _: &Self) -> bool { false }
        fn is_intersection_empty(&self, _: &Self) -> bool { true }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut self_intervals = vec![TestInterval { lower: Bound(5), upper: Bound(10) }];
    let other_intervals = vec![TestInterval { lower: Bound(0), upper: Bound(4) }];
    
    let mut self_set = IntervalSet::new(self_intervals);
    let other_set = IntervalSet::new(other_intervals);
    
    self_set.difference(&other_set);
}

#[test]
fn test_difference_exceeding_bounds() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct Bound(u32);
    
    impl Bound {
        fn upper(&self) -> u32 { self.0 }
        fn lower(&self) -> u32 { self.0 }
        fn decrement(&self) -> Bound { Bound(self.0 - 1) }
        fn increment(&self) -> Bound { Bound(self.0 + 1) }
    }
    
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for TestInterval {
        type Bound = Bound;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _: &Self) -> bool { false }
        fn is_intersection_empty(&self, _: &Self) -> bool { true }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut self_intervals = vec![TestInterval { lower: Bound(5), upper: Bound(10) }];
    let other_intervals = vec![TestInterval { lower: Bound(10), upper: Bound(15) }];
    
    let mut self_set = IntervalSet::new(self_intervals);
    let other_set = IntervalSet::new(other_intervals);
    
    self_set.difference(&other_set);
}

