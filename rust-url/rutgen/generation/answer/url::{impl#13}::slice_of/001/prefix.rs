// Answer 0

#[test]
fn test_slice_of_valid_range() {
    let range = 0..5;
    let s = "Hello, World!";
    let result = range.slice_of(s);
}

#[test]
fn test_slice_of_empty_string() {
    let range = 0..0;
    let s = "";
    let result = range.slice_of(s);
}

#[test]
fn test_slice_of_start_zero_end_length() {
    let range = 0..13;
    let s = "Hello, World!";
    let result = range.slice_of(s);
}

#[test]
fn test_slice_of_middle_segment() {
    let range = 7..12;
    let s = "Hello, World!";
    let result = range.slice_of(s);
}

#[test]
#[should_panic]
fn test_slice_of_out_of_bounds_start() {
    let range = 14..15;
    let s = "Hello, World!";
    let result = range.slice_of(s);
}

#[test]
#[should_panic]
fn test_slice_of_out_of_bounds_end() {
    let range = 10..20;
    let s = "Hello, World!";
    let result = range.slice_of(s);
}

#[test]
#[should_panic]
fn test_slice_of_start_equals_end() {
    let range = 5..5;
    let s = "Hello, World!";
    let result = range.slice_of(s);
}

