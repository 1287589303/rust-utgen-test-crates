// Answer 0

#[test]
fn test_simplify_range_included_start_and_end_equal_len() {
    let len = 5;
    let range = Range {
        start: Bound::Included(len),
        end: Bound::Included(len),
    };
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_included_start_on_boundary() {
    let len = 5;
    let range = Range {
        start: Bound::Included(len),
        end: Bound::Included(len),
    };
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_included_start_zero() {
    let len = 5;
    let range = Range {
        start: Bound::Included(0),
        end: Bound::Included(len),
    };
    simplify_range(range, len);
}

