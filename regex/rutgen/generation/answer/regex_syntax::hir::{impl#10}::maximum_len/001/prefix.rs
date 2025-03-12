// Answer 0

#[test]
fn test_maximum_len_bytes_single_byte_range() {
    let byte_class = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(0x61, 0x61)]));
    let _ = byte_class.maximum_len();
}

#[test]
fn test_maximum_len_bytes_multiple_byte_ranges() {
    let byte_class = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0x00, 0x1F),
        ClassBytesRange::new(0x20, 0x7E),
        ClassBytesRange::new(0x80, 0xFF),
    ]));
    let _ = byte_class.maximum_len();
}

#[test]
fn test_maximum_len_bytes_full_range() {
    let byte_class = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(0x00, 0xFF)]));
    let _ = byte_class.maximum_len();
}

#[test]
fn test_maximum_len_bytes_empty_class() {
    let byte_class = Class::Bytes(ClassBytes::empty());
    let _ = byte_class.maximum_len();
}

