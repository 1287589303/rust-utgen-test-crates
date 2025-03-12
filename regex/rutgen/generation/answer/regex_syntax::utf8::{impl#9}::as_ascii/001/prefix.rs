// Answer 0

#[test]
fn test_as_ascii_valid_range_1() {
    let range = ScalarRange { start: 0, end: 10 };
    let result = range.as_ascii();
}

#[test]
fn test_as_ascii_valid_range_2() {
    let range = ScalarRange { start: 50, end: 70 };
    let result = range.as_ascii();
}

#[test]
fn test_as_ascii_valid_range_3() {
    let range = ScalarRange { start: 120, end: 127 };
    let result = range.as_ascii();
}

#[test]
fn test_as_ascii_single_byte_range() {
    let range = ScalarRange { start: 100, end: 100 };
    let result = range.as_ascii();
}

#[test]
fn test_as_ascii_range_with_same_start_end() {
    let range = ScalarRange { start: 0, end: 0 };
    let result = range.as_ascii();
}

