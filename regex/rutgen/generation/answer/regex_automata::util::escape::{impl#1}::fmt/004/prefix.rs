// Answer 0

#[test]
fn test_empty_array() {
    let bytes: &[u8] = &[];
    let haystack = DebugHaystack(bytes);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_single_invalid_utf8_byte() {
    let bytes: &[u8] = &[0x80]; // Invalid UTF-8 byte
    let haystack = DebugHaystack(bytes);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_valid_ascii_control_characters() {
    let bytes: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x08, 0x0b, 0x0c, 0x0e, 0x7f]; // ASCII control characters
    let haystack = DebugHaystack(bytes);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_valid_utf8_characters() {
    let bytes: &[u8] = b"Hello, world!\nThis is a test.\tGoodbye!"; // Valid UTF-8 characters including control characters
    let haystack = DebugHaystack(bytes);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_mixed_valid_and_invalid_utf8_bytes() {
    let bytes: &[u8] = &[0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x80]; // First part valid, last byte invalid
    let haystack = DebugHaystack(bytes);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

