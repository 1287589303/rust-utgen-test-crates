// Answer 0

#[test]
fn test_difference_non_overlapping_upper_lower() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(usize);
    
    impl TestBound {
        fn upper(&self) -> Self {
            self.clone()
        }
        
        fn lower(&self) -> Self {
            self.clone()
        }
        
        fn decrement(self) -> Self {
            TestBound(self.0 - 1)
        }
        
        fn increment(self) -> Self {
            TestBound(self.0 + 1)
        }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower // Assuming non-overlapping
        }
        
        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
        
        fn difference(&self, _other: &Self) -> (Option<Self>, Option<Self>) {
            (Some(self.clone()), None)
        }        
    }
    
    let mut intervals1 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(10) },
    ]);
    
    let intervals2 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(7) },
    ]);
    
    intervals1.difference(&intervals2);
}

#[test]
fn test_difference_no_remaining_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(usize);
    
    impl TestBound {
        fn upper(&self) -> Self {
            self.clone()
        }
        
        fn lower(&self) -> Self {
            self.clone()
        }
        
        fn decrement(self) -> Self {
            TestBound(self.0 - 1)
        }
        
        fn increment(self) -> Self {
            TestBound(self.0 + 1)
        }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower // Assuming non-overlapping
        }
        
        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
        
        fn difference(&self, _other: &Self) -> (Option<Self>, Option<Self>) {
            (None, None)
        }        
    }
    
    let mut intervals1 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    
    let intervals2 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
    ]);
    
    intervals1.difference(&intervals2);
}

#[test]
fn test_difference_final_range_unused() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(usize);
    
    impl TestBound {
        fn upper(&self) -> Self {
            self.clone()
        }
        
        fn lower(&self) -> Self {
            self.clone()
        }
        
        fn decrement(self) -> Self {
            TestBound(self.0 - 1)
        }
        
        fn increment(self) -> Self {
            TestBound(self.0 + 1)
        }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower // Assuming non-overlapping
        }
        
        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
        
        fn difference(&self, _other: &Self) -> (Option<Self>, Option<Self>) {
            (Some(self.clone()), Some(self.clone()))
        }        
    }
    
    let mut intervals1 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(10) },
    ]);
    
    let intervals2 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(2), upper: TestBound(3) },
        TestInterval { lower: TestBound(4), upper: TestBound(5) },
    ]);
    
    intervals1.difference(&intervals2);
}

