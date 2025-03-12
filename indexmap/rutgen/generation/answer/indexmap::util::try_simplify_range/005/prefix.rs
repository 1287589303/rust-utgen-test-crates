// Answer 0

#[test]
fn test_try_simplify_range_unbounded_start_excluded_end_equal_len() {
    use core::ops::RangeBounds;
    use core::ops::Bound;

    struct UnboundedRange;

    impl RangeBounds<usize> for UnboundedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&0)
        }
    }

    let len = 0;
    let range = UnboundedRange;

    let result = try_simplify_range(range, len);
}

#[test]
fn test_try_simplify_range_unbounded_start_excluded_end_greater_than_len() {
    use core::ops::RangeBounds;
    use core::ops::Bound;

    struct UnboundedRange;

    impl RangeBounds<usize> for UnboundedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&1)
        }
    }

    let len = 1;
    let range = UnboundedRange;

    let result = try_simplify_range(range, len);
}

#[test]
fn test_try_simplify_range_unbounded_start_excluded_end_equals_len() {
    use core::ops::RangeBounds;
    use core::ops::Bound;

    struct UnboundedRange;

    impl RangeBounds<usize> for UnboundedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&2)
        }
    }

    let len = 2;
    let range = UnboundedRange;

    let result = try_simplify_range(range, len);
}

