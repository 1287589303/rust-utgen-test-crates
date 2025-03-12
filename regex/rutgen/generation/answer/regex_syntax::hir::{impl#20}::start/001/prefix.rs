// Answer 0

#[test]
fn test_class_bytes_range_start_min() {
    let range = ClassBytesRange::new(0, 255);
    let start = range.start();
}

#[test]
fn test_class_bytes_range_start_mid() {
    let range = ClassBytesRange::new(128, 200);
    let start = range.start();
}

#[test]
fn test_class_bytes_range_start_max() {
    let range = ClassBytesRange::new(255, 255);
    let start = range.start();
}

#[test]
fn test_class_bytes_range_start_equal_end() {
    let range = ClassBytesRange::new(100, 100);
    let start = range.start();
}

#[test]
fn test_class_bytes_range_start_less_than_end() {
    let range = ClassBytesRange::new(50, 150);
    let start = range.start();
}

