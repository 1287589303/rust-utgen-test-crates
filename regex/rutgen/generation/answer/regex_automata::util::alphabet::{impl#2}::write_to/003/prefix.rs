// Answer 0

#[test]
fn test_write_to_with_exact_buffer_length() {
    let byte_classes = ByteClasses::empty();
    let mut dst = vec![0u8; 256];
    let result = byte_classes.write_to(&mut dst);
}

#[test]
fn test_write_to_with_full_buffer() {
    let byte_classes = ByteClasses::singletons();
    let mut dst = vec![0u8; 256];
    let result = byte_classes.write_to(&mut dst);
}

