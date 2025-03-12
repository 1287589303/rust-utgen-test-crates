// Answer 0

#[test]
#[should_panic]
fn test_simplify_range_unbounded_start_included_end() {
    let len = 0;
    let range = Range {
        start: Bound::Unbounded,
        end: Bound::Included(0),
    };
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_unbounded_start_included_end_negative_case() {
    let len = 1;
    let range = Range {
        start: Bound::Unbounded,
        end: Bound::Included(1),
    };
    simplify_range(range, len);
}

#[test]
#[should_panic]
fn test_simplify_range_unbounded_start_excluded_end() {
    let len = 2;
    let range = Range {
        start: Bound::Unbounded,
        end: Bound::Included(1),
    };
    simplify_range(range, len);
}

