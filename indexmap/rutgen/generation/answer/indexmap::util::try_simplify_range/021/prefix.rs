// Answer 0

#[test]
fn test_try_simplify_range_included_start_equal_len_excluded_end_greater_len() {
    struct TestRange {
        start_value: usize,
        end_value: usize,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start_value)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.end_value)
        }
    }

    let range = TestRange {
        start_value: 5,
        end_value: 10,
    };
    let len = 5;

    let _result = try_simplify_range(range, len);
}

#[test]
fn test_try_simplify_range_included_start_equal_len_excluded_end_equal_len_plus_one() {
    struct TestRange {
        start_value: usize,
        end_value: usize,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start_value)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.end_value)
        }
    }

    let range = TestRange {
        start_value: 5,
        end_value: 6,
    };
    let len = 5;

    let _result = try_simplify_range(range, len);
}

