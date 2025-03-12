// Answer 0

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    assert_eq!(buffer.bytes.len(), 24);
}

#[test]
fn test_buffer_bytes_initialization() {
    let buffer = Buffer::new();
    for byte in &buffer.bytes {
        assert!(byte.as_ptr().is_null());
    }
}

