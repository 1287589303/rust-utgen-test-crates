// Answer 0

#[test]
fn test_is_utf8_bytes_ascii() {
    let byte_ranges = vec![ClassBytesRange::new(0x00, 0x7F)];
    let class_bytes = ClassBytes::new(byte_ranges);
    let class = Class::Bytes(class_bytes);
    class.is_utf8();
}

#[test]
fn test_is_utf8_bytes_multiple_ascii_ranges() {
    let byte_ranges = vec![
        ClassBytesRange::new(0x20, 0x7F),
        ClassBytesRange::new(0x00, 0x1F),
    ];
    let class_bytes = ClassBytes::new(byte_ranges);
    let class = Class::Bytes(class_bytes);
    class.is_utf8();
}

#[test]
fn test_is_utf8_bytes_empty() {
    let class_bytes = ClassBytes::empty();
    let class = Class::Bytes(class_bytes);
    class.is_utf8();
}

