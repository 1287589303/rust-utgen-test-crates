// Answer 0

#[test]
fn test_case_fold_simple_with_empty_class() {
    let mut class_bytes = ClassBytes::empty();
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_lowercase_a_to_z() {
    let ranges = vec![ClassBytesRange { start: 97, end: 122 }]; // a-z
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_uppercase_a_to_z() {
    let ranges = vec![ClassBytesRange { start: 65, end: 90 }]; // A-Z
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_mixed_case() {
    let ranges = vec![
        ClassBytesRange { start: 65, end: 90 }, // A-Z
        ClassBytesRange { start: 97, end: 122 }, // a-z
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_full_ascii() {
    let ranges = vec![ClassBytesRange { start: 0, end: 255 }]; // 0-255
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_gap_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 65, end: 90 }, // A-Z
        ClassBytesRange { start: 97, end: 122 }, // a-z
        ClassBytesRange { start: 0, end: 64 },  // 0-64
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_edge_cases() {
    let ranges = vec![
        ClassBytesRange { start: 32, end: 32 }, // Space
        ClassBytesRange { start: 255, end: 255 }, // Non-ASCII character
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
}

