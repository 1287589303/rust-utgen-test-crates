// Answer 0

#[test]
fn test_empty_range_start_greater_than_end() {
    let range: Range<i32> = 5..3;
    let result = range.is_empty();
}

#[test]
fn test_empty_range_start_equal_to_end() {
    let range: Range<i32> = 4..4;
    let result = range.is_empty();
}

#[test]
fn test_non_empty_range_start_less_than_end() {
    let range: Range<i32> = 1..10;
    let result = range.is_empty();
}

#[test]
fn test_empty_float_range_start_greater_than_end() {
    let range: Range<f32> = 3.0..2.0;
    let result = range.is_empty();
}

#[test]
fn test_empty_float_range_start_equal_to_end() {
    let range: Range<f32> = 2.0..2.0;
    let result = range.is_empty();
}

#[test]
fn test_non_empty_float_range_start_less_than_end() {
    let range: Range<f32> = 0.0..1.0;
    let result = range.is_empty();
}

#[test]
fn test_empty_char_range_start_greater_than_end() {
    let range: Range<char> = 'c'..'a';
    let result = range.is_empty();
}

#[test]
fn test_empty_char_range_start_equal_to_end() {
    let range: Range<char> = 'b'..'b';
    let result = range.is_empty();
}

#[test]
fn test_non_empty_char_range_start_less_than_end() {
    let range: Range<char> = 'a'..'z';
    let result = range.is_empty();
}

