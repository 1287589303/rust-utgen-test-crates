// Answer 0

#[test]
fn test_difference_case1() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(i32);
    
    impl TestBound {
        fn upper(&self) -> i32 {
            self.0
        }
        fn lower(&self) -> i32 {
            self.0
        }
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _other: &Self) -> bool { false }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper <= other.lower || self.lower >= other.upper }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(6) },
    ]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_case2() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(i32);
    
    impl TestBound {
        fn upper(&self) -> i32 {
            self.0
        }
        fn lower(&self) -> i32 {
            self.0
        }
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _other: &Self) -> bool { false }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper <= other.lower || self.lower >= other.upper }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(2), upper: TestBound(3) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
    ]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_case3() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(i32);
    
    impl TestBound {
        fn upper(&self) -> i32 {
            self.0
        }
        fn lower(&self) -> i32 {
            self.0
        }
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, _other: &Self) -> bool { false }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper <= other.lower || self.lower >= other.upper }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(10), upper: TestBound(20) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(20), upper: TestBound(30) },
    ]);

    set_a.difference(&set_b);
}

