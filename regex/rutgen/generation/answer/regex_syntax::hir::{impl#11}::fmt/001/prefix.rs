// Answer 0

#[test]
fn test_fmt_class_bytes_single_range() {
    let range = ClassBytesRange { start: 1, end: 2 };
    let class_bytes = ClassBytes::new(vec![range]);
    let mut formatter = core::fmt::Formatter::new();
    let _ = class_bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_class_bytes_multiple_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 1, end: 3 },
        ClassBytesRange { start: 5, end: 7 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    let mut formatter = core::fmt::Formatter::new();
    let _ = class_bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_class_bytes_empty() {
    let class_bytes = ClassBytes::empty();
    let mut formatter = core::fmt::Formatter::new();
    let _ = class_bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_class_bytes_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 1, end: 5 },
        ClassBytesRange { start: 3, end: 7 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    let mut formatter = core::fmt::Formatter::new();
    let _ = class_bytes.fmt(&mut formatter);
}

