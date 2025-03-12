// Answer 0

#[test]
fn test_intersect_non_overlapping_ranges() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 10, end: 15 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 6, end: 9 },
        ClassBytesRange { start: 16, end: 20 },
    ]);
    class_bytes_a.intersect(&class_bytes_b);
}

#[test]
fn test_intersect_identical_ranges() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    class_bytes_a.intersect(&class_bytes_b);
}

#[test]
fn test_intersect_self_containing_other() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 10 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 8 },
    ]);
    class_bytes_a.intersect(&class_bytes_b);
}

#[test]
fn test_intersect_other_containing_self() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 8 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 10 },
    ]);
    class_bytes_a.intersect(&class_bytes_b);
}

#[test]
fn test_intersect_empty_self() {
    let mut class_bytes_a = ClassBytes::empty();
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    class_bytes_a.intersect(&class_bytes_b);
}

#[test]
fn test_intersect_empty_other() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    let class_bytes_b = ClassBytes::empty();
    class_bytes_a.intersect(&class_bytes_b);
}

#[test]
fn test_intersect_with_no_ranges() {
    let mut class_bytes_a = ClassBytes::empty();
    let mut class_bytes_b = ClassBytes::empty();
    class_bytes_a.intersect(&class_bytes_b);
}

