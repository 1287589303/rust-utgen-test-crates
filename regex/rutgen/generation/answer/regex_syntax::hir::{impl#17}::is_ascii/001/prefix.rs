// Answer 0

#[test]
fn test_is_ascii_empty() {
    let class_bytes = ClassBytes::empty();
    class_bytes.is_ascii();
}

#[test]
fn test_is_ascii_only_ascii() {
    let class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 127 }]);
    class_bytes.is_ascii();
}

#[test]
fn test_is_ascii_non_ascii() {
    let class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 128, end: 255 }]);
    class_bytes.is_ascii();
}

#[test]
fn test_is_ascii_mixed() {
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 127 },
        ClassBytesRange { start: 128, end: 255 },
    ]);
    class_bytes.is_ascii();
}

#[test]
fn test_is_ascii_boundary() {
    let class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 127, end: 128 }]);
    class_bytes.is_ascii();
}

#[test]
fn test_is_ascii_multiple_ranges() {
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 64 },
        ClassBytesRange { start: 65, end: 127 },
    ]);
    class_bytes.is_ascii();
}

