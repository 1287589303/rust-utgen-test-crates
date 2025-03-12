// Answer 0

#[test]
fn test_difference_non_empty_overlapping() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 10, end: 15 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 3, end: 12 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_non_empty_adjacent() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 6, end: 10 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 6 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_non_empty_disjoint() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 6, end: 10 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_single_element_case() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 1 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 1 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_with_empty_result() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 10 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 10 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_with_multiple_ranges() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 10, end: 15 },
        ClassBytesRange { start: 20, end: 25 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 4, end: 12 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

