// Answer 0

#[test]
fn test_push_wtf8_codepoint_below_80() {
    let mut scratch = Vec::with_capacity(4);
    for n in 0..=127 {
        push_wtf8_codepoint(n, &mut scratch);
    }
}

#[test]
fn test_push_wtf8_codepoint_edge_case_0() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(0, &mut scratch);
}

#[test]
fn test_push_wtf8_codepoint_edge_case_127() {
    let mut scratch = Vec::with_capacity(4);
    push_wtf8_codepoint(127, &mut scratch);
}

