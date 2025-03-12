// Answer 0

#[test]
fn test_try_simplify_range_included_bound_excluded_bound() {
    use core::ops::{Bound, RangeBounds};
    
    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }

    let len = 5;
    let range = TestRange {
        start: Bound::Included(&len),
        end: Bound::Excluded(&len),
    };

    let _result = try_simplify_range(range, len);
}

