// Answer 0

#[test]
fn test_to_unicode_class_single_ascii_range() {
    let range = ClassBytesRange { start: 0x41, end: 0x41 }; // 'A'
    let class_bytes = ClassBytes::new(vec![range]);
    let _result = class_bytes.to_unicode_class();
}

#[test]
fn test_to_unicode_class_multiple_ascii_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0x30, end: 0x39 }, // '0'-'9'
        ClassBytesRange { start: 0x41, end: 0x5A }, // 'A'-'Z'
        ClassBytesRange { start: 0x61, end: 0x7A }, // 'a'-'z'
    ];
    let class_bytes = ClassBytes::new(ranges);
    let _result = class_bytes.to_unicode_class();
}

#[test]
fn test_to_unicode_class_full_ascii_range() {
    let range = ClassBytesRange { start: 0x00, end: 0x7F }; // Full ASCII range
    let class_bytes = ClassBytes::new(vec![range]);
    let _result = class_bytes.to_unicode_class();
}

#[test]
fn test_to_unicode_class_disjoint_ascii_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0x20, end: 0x20 }, // Space
        ClassBytesRange { start: 0x21, end: 0x21 }, // '!'
        ClassBytesRange { start: 0x7E, end: 0x7E }, // '~'
    ];
    let class_bytes = ClassBytes::new(ranges);
    let _result = class_bytes.to_unicode_class();
}

