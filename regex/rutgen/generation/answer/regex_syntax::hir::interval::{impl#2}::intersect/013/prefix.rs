// Answer 0

#[test]
fn test_intersect_non_empty_with_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    struct Bound(u32);
    
    impl Bound {
        fn decrement(&self) -> Self {
            Bound(self.0 - 1)
        }
        fn increment(&self) -> Self {
            Bound(self.0 + 1)
        }
    }
    
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for TestInterval {
        type Bound = Bound;
        
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
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.lower > other.upper || self.upper < other.lower
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: Bound(1), upper: Bound(5) },
        TestInterval { lower: Bound(6), upper: Bound(10) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: Bound(3), upper: Bound(7) },
        TestInterval { lower: Bound(8), upper: Bound(12) },
    ]);
    
    set_a.folded = true;

    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_non_empty_with_overlap_case_folded() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    struct Bound(u32);
    
    impl Bound {
        fn decrement(&self) -> Self {
            Bound(self.0 - 1)
        }
        fn increment(&self) -> Self {
            Bound(self.0 + 1)
        }
    }
    
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for TestInterval {
        type Bound = Bound;
        
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
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.lower > other.upper || self.upper < other.lower
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: Bound(2), upper: Bound(6) },
        TestInterval { lower: Bound(7), upper: Bound(12) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: Bound(4), upper: Bound(8) },
        TestInterval { lower: Bound(9), upper: Bound(15) },
    ]);
    
    set_a.folded = true;

    set_a.intersect(&set_b);
}

