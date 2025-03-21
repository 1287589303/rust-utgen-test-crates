// Answer 0

#[test]
fn test_difference_empty_intersection() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleBound(i32);
    
    impl Bound for SimpleBound {
        fn decrement(&self) -> Self {
            SimpleBound(self.0 - 1)
        }
        
        fn increment(&self) -> Self {
            SimpleBound(self.0 + 1)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { self.lower.0 <= other.upper.0 && other.lower.0 <= self.upper.0 }
    }

    let self_interval = SimpleInterval {
        lower: SimpleBound(10),
        upper: SimpleBound(15),
    };

    let other_interval = SimpleInterval {
        lower: SimpleBound(16),
        upper: SimpleBound(20),
    };
    
    let result = self_interval.difference(&other_interval);
}

#[test]
fn test_difference_other_lower_bound_greater() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleBound(i32);
    
    impl Bound for SimpleBound {
        fn decrement(&self) -> Self {
            SimpleBound(self.0 - 1)
        }
        
        fn increment(&self) -> Self {
            SimpleBound(self.0 + 1)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { self.lower.0 <= other.upper.0 && other.lower.0 <= self.upper.0 }
    }

    let self_interval = SimpleInterval {
        lower: SimpleBound(5),
        upper: SimpleBound(10),
    };

    let other_interval = SimpleInterval {
        lower: SimpleBound(11),
        upper: SimpleBound(15),
    };
    
    let result = self_interval.difference(&other_interval);
}

#[test]
fn test_difference_distinct_ranges() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleBound(i32);
    
    impl Bound for SimpleBound {
        fn decrement(&self) -> Self {
            SimpleBound(self.0 - 1)
        }
        
        fn increment(&self) -> Self {
            SimpleBound(self.0 + 1)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> { Ok(()) }
        fn is_contiguous(&self, other: &Self) -> bool { self.lower.0 <= other.upper.0 && other.lower.0 <= self.upper.0 }
    }

    let self_interval = SimpleInterval {
        lower: SimpleBound(1),
        upper: SimpleBound(3),
    };

    let other_interval = SimpleInterval {
        lower: SimpleBound(4),
        upper: SimpleBound(6),
    };
    
    let result = self_interval.difference(&other_interval);
}

