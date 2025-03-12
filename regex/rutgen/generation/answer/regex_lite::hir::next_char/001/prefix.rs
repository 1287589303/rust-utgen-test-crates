// Answer 0

#[test]
fn test_next_char_with_valid_char_below_surrogate() {
    let result = next_char('\u{D7FE}');
}

#[test]
fn test_next_char_with_valid_char_above_surrogate() {
    let result = next_char('\u{E000}');
}

#[test]
fn test_next_char_with_minimum_char() {
    let result = next_char('\u{0000}');
}

#[test]
fn test_next_char_with_maximum_char() {
    let result = next_char('\u{FFFF}');
}

