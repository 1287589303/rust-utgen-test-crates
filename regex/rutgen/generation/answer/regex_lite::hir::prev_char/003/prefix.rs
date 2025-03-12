// Answer 0

#[test]
fn test_prev_char_e000() {
    let ch = '\u{E000}';
    let result = prev_char(ch);
}

#[test]
fn test_prev_char_valid_scalar_min() {
    let ch = '\u{0001}';
    let result = prev_char(ch);
}

#[test]
fn test_prev_char_valid_scalar_boundary() {
    let ch = '\u{D7FF}';
    let result = prev_char(ch);
}

#[test]
fn test_prev_char_valid_scalar_mid_range() {
    let ch = '\u{007F}';
    let result = prev_char(ch);
}

#[test]
fn test_prev_char_valid_scalar_operations() {
    let ch = '\u{0100}';
    let result = prev_char(ch);
}

