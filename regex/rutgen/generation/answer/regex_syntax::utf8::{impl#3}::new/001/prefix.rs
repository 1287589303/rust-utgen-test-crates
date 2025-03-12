// Answer 0

#[test]
fn test_utf8_range_creation_valid_within_single_byte() {
    let start = 0;
    let end = 127;
    let range = Utf8Range::new(start, end);
}

#[test]
fn test_utf8_range_creation_valid_within_multi_byte() {
    let start = 192;
    let end = 255;
    let range = Utf8Range::new(start, end);
}

#[test]
fn test_utf8_range_creation_valid_with_exact_boundaries() {
    let start = 0;
    let end = 0;
    let range = Utf8Range::new(start, end);
}

#[test]
fn test_utf8_range_creation_valid_with_start_equals_end() {
    let start = 100;
    let end = 100;
    let range = Utf8Range::new(start, end);
}

#[test]
#[should_panic]
fn test_utf8_range_creation_invalid_out_of_bounds_start() {
    let start = 256; // Invalid
    let end = 255;
    let range = Utf8Range::new(start, end);
}

#[test]
#[should_panic]
fn test_utf8_range_creation_invalid_out_of_bounds_end() {
    let start = 0;
    let end = 256; // Invalid
    let range = Utf8Range::new(start, end);
}

#[test]
#[should_panic]
fn test_utf8_range_creation_invalid_start_greater_than_end() {
    let start = 100;
    let end = 99; // Invalid
    let range = Utf8Range::new(start, end);
}

