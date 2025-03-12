// Answer 0

#[test]
fn test_push_wtf8_codepoint_in_range_0x80_to_0x7FF() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0x80, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_in_range_0x80_to_0x7FF_mid() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0x1FF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_in_range_0x80_to_0x7FF_high() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0x7FF, &mut scratch);
}

