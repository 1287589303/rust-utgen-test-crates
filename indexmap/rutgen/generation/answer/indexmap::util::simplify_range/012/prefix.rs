// Answer 0

#[test]
fn test_simplify_range_start_excluded_end_excluded_equal() {
    use core::ops::Bound;

    let len = 5;
    let start_range: Range<usize> = Bound::Excluded(len).into();
    let end_range: Range<usize> = Bound::Excluded(len).into();
    let range = (start_range, end_range);
    
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_start_excluded_end_excluded_one_less() {
    use core::ops::Bound;

    let len = 5;
    let start_range: Range<usize> = Bound::Excluded(3).into();
    let end_range: Range<usize> = Bound::Excluded(3).into();
    let range = (start_range, end_range);
    
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_start_excluded_end_excluded_zero() {
    use core::ops::Bound;

    let len = 5;
    let start_range: Range<usize> = Bound::Excluded(0).into();
    let end_range: Range<usize> = Bound::Excluded(0).into();
    let range = (start_range, end_range);
    
    simplify_range(range, len);
}

