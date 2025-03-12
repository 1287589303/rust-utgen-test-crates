// Answer 0

#[test]
fn test_new_empty() {
    let ranges: Vec<ClassBytesRange> = vec![];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_single_range() {
    let ranges = vec![ClassBytesRange { start: 0, end: 5 }];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_non_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 10, end: 15 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 3, end: 7 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_full_range() {
    let ranges = vec![ClassBytesRange { start: 0, end: 255 }];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_multiple_full_range_intervals() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 127 },
        ClassBytesRange { start: 128, end: 255 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

