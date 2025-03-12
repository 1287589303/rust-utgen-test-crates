// Answer 0

#[test]
fn test_negate_with_full_range() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 255 }]);
    class_bytes.negate();
}

#[test]
fn test_negate_with_empty_class() {
    let mut class_bytes = ClassBytes::empty();
    class_bytes.negate();
}

#[test]
fn test_negate_with_single_range() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 10, end: 20 }]);
    class_bytes.negate();
}

#[test]
fn test_negate_with_multiple_ranges() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }, ClassBytesRange { start: 10, end: 15 }]);
    class_bytes.negate();
}

#[test]
fn test_negate_boundary_conditions() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 0 }]);
    class_bytes.negate();
    let mut class_bytes_full = ClassBytes::new(vec![ClassBytesRange { start: 255, end: 255 }]);
    class_bytes_full.negate();
}

