// Answer 0

#[test]
fn test_try_simplify_range_included_equals_len_unbounded() {
    let len = 5;
    let range = std::ops::RangeInclusive::new(len, len);
    let result = try_simplify_range(range, len);
}

#[test]
fn test_try_simplify_range_included_less_than_len_unbounded() {
    let len = 5;
    let range = std::ops::RangeInclusive::new(len - 1, len);
    let result = try_simplify_range(range, len);
}

