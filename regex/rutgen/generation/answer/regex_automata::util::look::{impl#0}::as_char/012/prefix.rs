// Answer 0

#[test]
fn test_look_as_char_word_ascii() {
    let look_value = Look::WordAscii;
    let _ = look_value.as_char();
}

#[test]
fn test_look_as_char_word_ascii_negate() {
    let look_value = Look::WordAsciiNegate;
    let _ = look_value.as_char();
}

#[test]
fn test_look_as_char_start() {
    let look_value = Look::Start;
    let _ = look_value.as_char();
}

#[test]
fn test_look_as_char_end() {
    let look_value = Look::End;
    let _ = look_value.as_char();
}

