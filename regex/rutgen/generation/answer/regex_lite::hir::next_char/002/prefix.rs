// Answer 0

#[test]
fn test_next_char_d7ff() {
    let result = next_char('\u{D7FF}');
}

#[test]
fn test_next_char_d7fe() {
    let result = next_char('\u{D7FE}');
}

#[test]
fn test_next_char_min_utf8() {
    let result = next_char('\u{0000}');
}

#[test]
fn test_next_char_max() {
    let result = next_char(char::MAX);
}

