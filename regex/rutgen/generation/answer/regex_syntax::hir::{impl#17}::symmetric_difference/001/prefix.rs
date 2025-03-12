// Answer 0

#[test]
fn test_symmetric_difference_non_overlapping_ranges() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 3 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 4, end: 5 }]);
    class_a.symmetric_difference(&class_b);
}

#[test]
fn test_symmetric_difference_overlapping_ranges() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 3, end: 7 }]);
    class_a.symmetric_difference(&class_b);
}

#[test]
fn test_symmetric_difference_identical_ranges() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 3 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 3 }]);
    class_a.symmetric_difference(&class_b);
}

#[test]
fn test_symmetric_difference_empty_class_a() {
    let mut class_a = ClassBytes::empty();
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 4, end: 5 }]);
    class_a.symmetric_difference(&class_b);
}

#[test]
fn test_symmetric_difference_empty_class_b() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 4, end: 5 }]);
    let class_b = ClassBytes::empty();
    class_a.symmetric_difference(&class_b);
}

#[test]
fn test_symmetric_difference_single_range_equal() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 2, end: 2 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 2, end: 2 }]);
    class_a.symmetric_difference(&class_b);
}

#[test]
fn test_symmetric_difference_single_range_non_equal() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 2, end: 2 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 3, end: 3 }]);
    class_a.symmetric_difference(&class_b);
}

#[test]
fn test_symmetric_difference_multiple_ranges() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 2 }, ClassBytesRange { start: 3, end: 4 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 2, end: 3 }, ClassBytesRange { start: 5, end: 6 }]);
    class_a.symmetric_difference(&class_b);
}

