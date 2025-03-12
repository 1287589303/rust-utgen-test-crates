// Answer 0

#[test]
fn test_ranges_with_single_range() {
    let range = ClassBytesRange { start: 0, end: 255 };
    let interval_set = IntervalSet::new(vec![range]);
    let class_bytes = ClassBytes { set: interval_set };
    class_bytes.ranges();
}

#[test]
fn test_ranges_with_multiple_non_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 100 },
        ClassBytesRange { start: 101, end: 200 },
        ClassBytesRange { start: 201, end: 255 },
    ];
    let interval_set = IntervalSet::new(ranges);
    let class_bytes = ClassBytes { set: interval_set };
    class_bytes.ranges();
}

#[test]
fn test_ranges_with_adjacent_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 100 },
        ClassBytesRange { start: 100, end: 200 },
    ];
    let interval_set = IntervalSet::new(ranges);
    let class_bytes = ClassBytes { set: interval_set };
    class_bytes.ranges();
}

#[test]
fn test_ranges_with_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 200 },
        ClassBytesRange { start: 100, end: 255 },
    ];
    let interval_set = IntervalSet::new(ranges);
    let class_bytes = ClassBytes { set: interval_set };
    class_bytes.ranges();
}

#[test]
fn test_ranges_with_empty_set() {
    let interval_set: IntervalSet<ClassBytesRange> = IntervalSet::new(vec![]);
    let class_bytes = ClassBytes { set: interval_set };
    class_bytes.ranges();
}

