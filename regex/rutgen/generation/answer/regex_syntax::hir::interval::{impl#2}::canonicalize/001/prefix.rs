// Answer 0

#[test]
fn test_canonicalize_already_canonical() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleBound(u32);
    
    impl Bound for SimpleBound {
        fn decrement(&self) -> Self {
            SimpleBound(self.0.saturating_sub(1))
        }

        fn increment(&self) -> Self {
            SimpleBound(self.0 + 1)
        }
    }
    
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

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

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 + 1 == other.lower.0 || other.upper.0 + 1 == self.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(3) },
        SimpleInterval { lower: SimpleBound(4), upper: SimpleBound(6) },
        SimpleInterval { lower: SimpleBound(7), upper: SimpleBound(9) },
    ]);

    interval_set.canonicalize();
}


#[test]
fn test_canonicalize_empty_range() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct Bound(u32);
    
    impl Bound for Bound {
        fn decrement(&self) -> Self {
            Bound(self.0.saturating_sub(1))
        }

        fn increment(&self) -> Self {
            Bound(self.0 + 1)
        }
    }
    
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq)]
    struct Interval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval for Interval {
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

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 + 1 == other.lower.0 || other.upper.0 + 1 == self.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        Interval { lower: Bound(2), upper: Bound(4) },
        Interval { lower: Bound(5), upper: Bound(7) },
    ]);

    interval_set.canonicalize();
}

