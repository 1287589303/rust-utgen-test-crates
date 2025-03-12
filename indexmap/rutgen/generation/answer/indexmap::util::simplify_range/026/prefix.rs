// Answer 0

#[test]
#[should_panic]
fn test_simplify_range_start_included_greater_than_len() {
    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start.as_ref()
        }
        fn end_bound(&self) -> Bound<&usize> {
            self.end.as_ref()
        }
    }

    let range = TestRange {
        start: Bound::Included(5),
        end: Bound::Excluded(5),
    };
    let len = 4;

    let _result = simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_start_included_out_of_range_end_unbounded() {
    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start.as_ref()
        }
        fn end_bound(&self) -> Bound<&usize> {
            self.end.as_ref()
        }
    }

    let range = TestRange {
        start: Bound::Included(3),
        end: Bound::Unbounded,
    };
    let len = 2;

    let _result = simplify_range(range, len);
}

