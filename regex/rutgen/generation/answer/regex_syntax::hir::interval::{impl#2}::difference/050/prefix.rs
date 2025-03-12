// Answer 0

#[test]
fn test_difference_self_empty_other_non_empty() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bound(u32);
    
    impl Bound {
        fn upper(&self) -> u32 { self.0 }
        fn lower(&self) -> u32 { self.0 }
        fn decrement(&self) -> Bound { Bound(self.0 - 1) }
        fn increment(&self) -> Bound { Bound(self.0 + 1) }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || self.lower() > other.upper()
        }
        fn difference(&self, other: &Self) -> (Option<Bound>, Option<Bound>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            (None, None)
        }
    }

    #[derive(Clone, Debug, Default)]
    struct MyInterval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval for MyInterval {
        type Bound = Bound;
        
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.lower().is_intersection_empty(&other.lower()) }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![MyInterval { lower: Bound(5), upper: Bound(10) }]);
    let set_b = IntervalSet::new(vec![MyInterval { lower: Bound(7), upper: Bound(12) }]);
    set_a.folded = true;

    set_a.difference(&set_b);
}

#[test]
fn test_difference_b_reaches_end() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bound(u32);
    
    impl Bound {
        fn upper(&self) -> u32 { self.0 }
        fn lower(&self) -> u32 { self.0 }
        fn decrement(&self) -> Bound { Bound(self.0 - 1) }
        fn increment(&self) -> Bound { Bound(self.0 + 1) }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || self.lower() > other.upper()
        }
        fn difference(&self, other: &Self) -> (Option<Bound>, Option<Bound>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            (None, None)
        }
    }

    #[derive(Clone, Debug, Default)]
    struct MyInterval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval for MyInterval {
        type Bound = Bound;
        
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.lower().is_intersection_empty(&other.lower()) }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![MyInterval { lower: Bound(5), upper: Bound(10) }]);
    let set_b = IntervalSet::new(vec![MyInterval { lower: Bound(1), upper: Bound(4) }, MyInterval { lower: Bound(8), upper: Bound(12) }]);
    set_a.folded = true;

    set_a.difference(&set_b);
}

