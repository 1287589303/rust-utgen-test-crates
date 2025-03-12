// Answer 0

#[test]
fn test_push_wtf8_codepoint_n_equals_0x80() {
    let n = 0x80;
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_in_range_0x80_to_0x7FF() {
    let n = 0x7FF;
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_in_range_0x800_to_0xFFFF() {
    let n = 0xFFFF;
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_in_range_0x10000_to_0x10FFFF() {
    let n = 0x10FFFF;
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(n, &mut scratch);
}

