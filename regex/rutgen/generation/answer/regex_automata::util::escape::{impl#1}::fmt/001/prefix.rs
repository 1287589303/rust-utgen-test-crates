// Answer 0

#[test]
fn test_empty_byte_array() {
    let empty_bytes: &[u8] = &[];
    let debug_haystack = DebugHaystack(empty_bytes);
    let _ = core::fmt::format(format_args!("{:?}", debug_haystack));
}

#[test]
fn test_valid_utf8() {
    let valid_bytes = b"Hello, World!";
    let debug_haystack = DebugHaystack(valid_bytes);
    let _ = core::fmt::format(format_args!("{:?}", debug_haystack));
}

#[test]
fn test_mixed_valid_and_invalid_utf8() {
    let mixed_bytes = b"Valid \x80 Invalid";
    let debug_haystack = DebugHaystack(mixed_bytes);
    let _ = core::fmt::format(format_args!("{:?}", debug_haystack));
}

#[test]
fn test_single_control_character() {
    let control_bytes = [0x01];
    let debug_haystack = DebugHaystack(&control_bytes);
    let _ = core::fmt::format(format_args!("{:?}", debug_haystack));
}

#[test]
fn test_multiple_control_characters() {
    let control_bytes = [0x00, 0x01, 0x02, 0x07, 0x08, 0x09, 0x0A, 0x7F];
    let debug_haystack = DebugHaystack(&control_bytes);
    let _ = core::fmt::format(format_args!("{:?}", debug_haystack));
}

#[test]
fn test_length_greater_than_byte_count() {
    let long_bytes = b"Short";
    let debug_haystack = DebugHaystack(long_bytes);
    let _ = core::fmt::format(format_args!("{:?}", debug_haystack));
}

