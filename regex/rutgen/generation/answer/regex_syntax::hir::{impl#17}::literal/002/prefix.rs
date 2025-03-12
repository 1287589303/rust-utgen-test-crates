// Answer 0

#[test]
fn test_literal_with_multiple_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 1 },
        ClassBytesRange { start: 2, end: 3 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    class_bytes.literal();
}

#[test]
fn test_literal_with_single_range_start_less_than_end() {
    let ranges = vec![ClassBytesRange { start: 0, end: 2 }];
    let class_bytes = ClassBytes::new(ranges);
    class_bytes.literal();
}

#[test]
fn test_literal_with_empty_class_bytes() {
    let class_bytes = ClassBytes::empty();
    class_bytes.literal();
}

