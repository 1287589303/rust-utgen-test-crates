// Answer 0

#[test]
fn test_len_with_valid_range() {
    let range = ClassBytesRange::new(0, 100);
    let result = range.len();
}

#[test]
fn test_len_with_start_equals_end() {
    let range = ClassBytesRange::new(0, 0);
    let result = range.len();
}

#[test]
fn test_len_with_start_one_less_than_end() {
    let range = ClassBytesRange::new(254, 255);
    let result = range.len();
}

#[test]
fn test_len_with_full_range() {
    let range = ClassBytesRange::new(0, 255);
    let result = range.len();
}

#[test]
fn test_len_with_mid_range() {
    let range = ClassBytesRange::new(128, 200);
    let result = range.len();
}

