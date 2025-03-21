// Answer 0

#[test]
fn test_union_with_empty_other() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleBound(u32);

    impl SimpleBound {
        fn decrement(self) -> Self {
            SimpleBound(self.0.saturating_sub(1))
        }

        fn increment(self) -> Self {
            SimpleBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && other.upper >= self.lower
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            !self.is_contiguous(other)
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set1 = IntervalSet::new(vec![SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) }]);
    let set2 = IntervalSet::new(Vec::<SimpleInterval>::new()); // other.ranges is empty
    set1.union(&set2);
}

