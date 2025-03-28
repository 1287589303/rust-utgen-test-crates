// Answer 0

#[test]
fn test_difference_self_empty_other_non_empty() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    struct Bound(u32);
    
    impl Bound {
        fn upper(&self) -> u32 { self.0 }
        fn lower(&self) -> u32 { self.0 }
        fn decrement(&self) -> Self { Bound(self.0.saturating_sub(1)) }
        fn increment(&self) -> Self { Bound(self.0 + 1) }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    struct Interval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval {
        fn create(lower: Bound, upper: Bound) -> Self {
            Interval { lower, upper }
        }
        
        fn lower(&self) -> Bound { self.lower }
        fn upper(&self) -> Bound { self.upper }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || other.upper() < self.lower()
        }
        
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            if self.lower() == other.lower() && self.upper() == other.upper() {
                return (None, None);
            }
            let mut left = None;
            let mut right = None;
            if self.lower() < other.lower() {
                left = Some(Interval::create(self.lower(), other.lower().decrement()));
            }
            if self.upper() > other.upper() {
                right = Some(Interval::create(other.upper().increment(), self.upper()));
            }
            (left, right)
        }
    }

    impl Interval {
        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
    }

    let mut empty_set: IntervalSet<Interval> = IntervalSet { ranges: vec![], folded: false };
    let other_set = IntervalSet { ranges: vec![Interval::create(Bound(1), Bound(2)), Interval::create(Bound(4), Bound(5))], folded: false };

    empty_set.difference(&other_set);
}

#[test]
fn test_difference_self_empty_other_single_overlap() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    struct Bound(u32);
    
    impl Bound {
        fn upper(&self) -> u32 { self.0 }
        fn lower(&self) -> u32 { self.0 }
        fn decrement(&self) -> Self { Bound(self.0.saturating_sub(1)) }
        fn increment(&self) -> Self { Bound(self.0 + 1) }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    struct Interval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval {
        fn create(lower: Bound, upper: Bound) -> Self {
            Interval { lower, upper }
        }
        
        fn lower(&self) -> Bound { self.lower }
        fn upper(&self) -> Bound { self.upper }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || other.upper() < self.lower()
        }
        
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            if self.lower() == other.lower() && self.upper() == other.upper() {
                return (None, None);
            }
            let mut left = None;
            let mut right = None;
            if self.lower() < other.lower() {
                left = Some(Interval::create(self.lower(), other.lower().decrement()));
            }
            if self.upper() > other.upper() {
                right = Some(Interval::create(other.upper().increment(), self.upper()));
            }
            (left, right)
        }
    }

    impl Interval {
        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
    }

    let mut empty_set: IntervalSet<Interval> = IntervalSet { ranges: vec![], folded: false };
    let other_set = IntervalSet { ranges: vec![Interval::create(Bound(1), Bound(3))], folded: false };

    empty_set.difference(&other_set);
}

#[test]
fn test_difference_self_empty_other_multiple_overlaps() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    struct Bound(u32);
    
    impl Bound {
        fn upper(&self) -> u32 { self.0 }
        fn lower(&self) -> u32 { self.0 }
        fn decrement(&self) -> Self { Bound(self.0.saturating_sub(1)) }
        fn increment(&self) -> Self { Bound(self.0 + 1) }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    struct Interval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval {
        fn create(lower: Bound, upper: Bound) -> Self {
            Interval { lower, upper }
        }
        
        fn lower(&self) -> Bound { self.lower }
        fn upper(&self) -> Bound { self.upper }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || other.upper() < self.lower()
        }
        
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            if self.lower() == other.lower() && self.upper() == other.upper() {
                return (None, None);
            }
            let mut left = None;
            let mut right = None;
            if self.lower() < other.lower() {
                left = Some(Interval::create(self.lower(), other.lower().decrement()));
            }
            if self.upper() > other.upper() {
                right = Some(Interval::create(other.upper().increment(), self.upper()));
            }
            (left, right)
        }
    }

    impl Interval {
        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
    }

    let mut empty_set: IntervalSet<Interval> = IntervalSet { ranges: vec![], folded: false };
    let other_set = IntervalSet { ranges: vec![Interval::create(Bound(1), Bound(4)), Interval::create(Bound(5), Bound(9))], folded: false };

    empty_set.difference(&other_set);
}

