// Answer 0

#[test]
fn test_is_empty_with_empty_range() {
    let range: RangeInclusive<i32> = 5..=5;
    let result = range.is_empty();
}

#[test]
fn test_is_empty_with_valid_range() {
    let range: RangeInclusive<i32> = 1..=10;
    let result = range.is_empty();
}

#[test]
fn test_is_empty_with_backward_range() {
    let range: RangeInclusive<i32> = 10..=1;
    let result = range.is_empty();
}

