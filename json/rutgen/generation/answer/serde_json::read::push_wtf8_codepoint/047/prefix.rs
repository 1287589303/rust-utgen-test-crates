// Answer 0

#[test]
fn test_push_wtf8_codepoint_beyond_valid_range() {
    let mut scratch = Vec::new();
    let n: u32 = 0x11_0000; // Minimum value that matches the precondition for unsuccessful encoding.
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_exceeding_maximum() {
    let mut scratch = Vec::new();
    let n: u32 = 0x10_FFFFFF; // Maximum value that matches the precondition for unsuccessful encoding.
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_above_upper_limit() {
    let mut scratch = Vec::new();
    let n: u32 = 0x20_0000; // A value just above 0x10_FFFF to ensure encoding is still not valid.
    push_wtf8_codepoint(n, &mut scratch);
}

