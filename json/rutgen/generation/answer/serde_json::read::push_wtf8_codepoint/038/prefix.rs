// Answer 0

#[test]
fn test_push_wtf8_codepoint_in_range_0x800_to_0xFFFF() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0x800, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_bound_0xFFFF() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0xFFFF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_mid_range() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0xCFFF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_invalid_high() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0xD800, &mut scratch);
}

