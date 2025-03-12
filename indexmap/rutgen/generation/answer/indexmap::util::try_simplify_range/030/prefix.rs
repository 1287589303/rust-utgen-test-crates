// Answer 0

#[test]
fn test_try_simplify_range_bound_included_start_greater_than_len() {
    use core::ops::{Bound, RangeBounds};
    struct IncludedRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for IncludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let range = IncludedRange { start: 5, end: 10 };
    let len = 4;
    let _result = try_simplify_range(range, len);
}

#[test]
fn test_try_simplify_range_bound_included_end_greater_than_len() {
    use core::ops::{Bound, RangeBounds};
    struct IncludedRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for IncludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = IncludedRange { start: 5, end: 6 };
    let len = 4;
    let _result = try_simplify_range(range, len);
}

