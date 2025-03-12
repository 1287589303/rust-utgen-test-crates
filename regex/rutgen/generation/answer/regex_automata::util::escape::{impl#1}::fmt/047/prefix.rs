// Answer 0

#[test]
fn test_debug_haystack_empty() {
    let haystack = DebugHaystack(&[]);
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_valid_utf8() {
    let haystack = DebugHaystack(b"Hello");
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_null_byte() {
    let haystack = DebugHaystack(b"\0");
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_single_byte_control_character() {
    let haystack = DebugHaystack(b"\x01");
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_utf8_multi_byte() {
    let haystack = DebugHaystack(b"\xe2\x9c\x94"); // Checkmark (U+2714)
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_invalid_utf8() {
    let haystack = DebugHaystack(b"\xff"); // invalid UTF-8 byte
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_exceeding_utf8_length() {
    let haystack = DebugHaystack(b"\xe2\x28"); // Invalid sequence
    let _ = format!("{:?}", haystack);
}

