// Answer 0

#[test]
fn test_simplify_range_excluded_start_boundary() {
    use core::ops::{Bound, RangeBounds};

    struct TestRange;

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&1)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let range = TestRange;
    let len = 2;
    let _result = simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_start_exact_length() {
    use core::ops::{Bound, RangeBounds};

    struct TestRange;

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&2)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let range = TestRange;
    let len = 2;
    let _result = simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_start_invalid_length() {
    use core::ops::{Bound, RangeBounds};

    struct TestRange;

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&3)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&4)
        }
    }

    let range = TestRange;
    let len = 3;
    let _result = simplify_range(range, len);
}

