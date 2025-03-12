// Answer 0

#[test]
fn test_negate_class_bytes_non_empty() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(0, 255)]));
    class_bytes.negate();
}

#[test]
fn test_negate_class_bytes_with_empty() {
    let mut class_bytes = Class::Bytes(ClassBytes::empty());
    class_bytes.negate();
}

#[test]
fn test_negate_class_bytes_multiple_ranges() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0, 50),
        ClassBytesRange::new(100, 150),
    ]));
    class_bytes.negate();
}

#[test]
fn test_negate_class_bytes_single_range() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(10, 20)]));
    class_bytes.negate();
}

#[test]
fn test_negate_class_bytes_boundary_range() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(255, 255), 
    ]));
    class_bytes.negate();
}

#[test]
fn test_negate_class_bytes_full_range() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0, 0), 
        ClassBytesRange::new(1, 255),
    ]));
    class_bytes.negate();
}

