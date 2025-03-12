// Answer 0

#[test]
fn test_simplify_range_included_start_bound_and_excluded_end_bound() {
    use core::ops::Bound;
    
    let range = 1..Bound::Excluded(3);
    let len = 3;

    let result = simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range end index 3 out of range for slice of length 3")]
fn test_simplify_range_included_start_bound_and_out_of_bounds_excluded_end() {
    use core::ops::Bound;

    let range = 2..Bound::Excluded(4);
    let len = 3;

    let result = simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 4")]
fn test_simplify_range_included_start_bound_excluded_end_out_of_bounds() {
    use core::ops::Bound;

    let range = 1..Bound::Excluded(5);
    let len = 4;

    let result = simplify_range(range, len);
}

