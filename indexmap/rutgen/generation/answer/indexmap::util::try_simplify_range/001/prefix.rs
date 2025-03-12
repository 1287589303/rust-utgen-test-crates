// Answer 0

#[test]
fn test_try_simplify_range_start_bound_greater_than_len() {
    struct TestRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = TestRange { start: 5, end: 10 };
    let len = 4;
    let _result = try_simplify_range(range, len);
}

#[test]
fn test_try_simplify_range_start_bound_invalid() {
    struct TestRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = TestRange { start: 6, end: 10 };
    let len = 5;
    let _result = try_simplify_range(range, len);
}

