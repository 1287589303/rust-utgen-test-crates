// Answer 0

#[test]
fn test_push_wtf8_codepoint_1() {
    let mut scratch = Vec::new();
    let n: u32 = 0x10000; // The first valid codepoint in the range
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_2() {
    let mut scratch = Vec::new();
    let n: u32 = 0x20000; // A codepoint in the middle of the range
    push_wtf8_codepoint(n, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_3() {
    let mut scratch = Vec::new();
    let n: u32 = 0x10FFFF; // The last valid codepoint in the range
    push_wtf8_codepoint(n, &mut scratch);
}

