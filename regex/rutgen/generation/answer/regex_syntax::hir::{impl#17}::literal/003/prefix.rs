// Answer 0

#[test]
fn test_literal_empty_class_bytes() {
    let class_bytes = ClassBytes::empty();
    let _result = class_bytes.literal();
}

#[test]
fn test_literal_multiple_byte_ranges() {
    let range1 = ClassBytesRange { start: 0u8, end: 1u8 };
    let range2 = ClassBytesRange { start: 2u8, end: 3u8 };
    let class_bytes = ClassBytes::new(vec![range1, range2]);
    let _result = class_bytes.literal();
}

