// Answer 0

#[test]
fn test_class_bytes_range_new_valid_range() {
    let range = ClassBytesRange::new(0, 255);
}

#[test]
fn test_class_bytes_range_new_single_value_range() {
    let range = ClassBytesRange::new(100, 100);
}

#[test]
fn test_class_bytes_range_new_minimum_range() {
    let range = ClassBytesRange::new(0, 0);
}

#[test]
#[should_panic]
fn test_class_bytes_range_new_invalid_range_negative() {
    let range = ClassBytesRange::new(256, 255);
}

#[test]
#[should_panic]
fn test_class_bytes_range_new_invalid_range_reverse() {
    let range = ClassBytesRange::new(255, 0);
}

#[test]
#[should_panic]
fn test_class_bytes_range_new_invalid_range_above_limit() {
    let range = ClassBytesRange::new(100, 256);
}

