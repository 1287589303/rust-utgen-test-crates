// Answer 0

#[test]
fn test_union_non_empty_with_empty() {
    let mut class_bytes_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    let class_bytes_b = ClassBytes::empty();
    class_bytes_a.union(&class_bytes_b);
}

#[test]
fn test_union_non_empty_with_different() {
    let mut class_bytes_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    let class_bytes_b = ClassBytes::new(vec![ClassBytesRange { start: 6, end: 10 }]);
    class_bytes_a.union(&class_bytes_b);
}

#[test]
fn test_union_with_identical() {
    let mut class_bytes_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    let class_bytes_b = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    class_bytes_a.union(&class_bytes_b);
}

#[test]
fn test_union_overlapping_ranges() {
    let mut class_bytes_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    let class_bytes_b = ClassBytes::new(vec![ClassBytesRange { start: 4, end: 8 }]);
    class_bytes_a.union(&class_bytes_b);
}

#[test]
fn test_union_multiple_ranges() {
    let mut class_bytes_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 3 }, ClassBytesRange { start: 6, end: 8 }]);
    let class_bytes_b = ClassBytes::new(vec![ClassBytesRange { start: 3, end: 6 }]);
    class_bytes_a.union(&class_bytes_b);
}

