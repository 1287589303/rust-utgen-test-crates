// Answer 0

#[test]
fn test_prev_char_e000() {
    let result = prev_char('\u{E000}');
}

#[test]
fn test_prev_char_valid_lower_bound() {
    let result = prev_char('\u{0001}');
}

#[test]
fn test_prev_char_valid_upper_bound() {
    let result = prev_char('\u{D7FF}');
}

#[test]
fn test_prev_char_zero() {
    let result = prev_char('\u{0000}');
}

