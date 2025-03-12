// Answer 0

#[test]
fn test_push_wtf8_codepoint_n_is_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_is_0x7FF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x7FF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_is_0x800() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_is_0xFFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0xFFFF, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_is_0x10000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10000, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_n_is_0x10FFFF() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10FFFF, &mut scratch);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_n_is_0x110000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x110000, &mut scratch);
}

