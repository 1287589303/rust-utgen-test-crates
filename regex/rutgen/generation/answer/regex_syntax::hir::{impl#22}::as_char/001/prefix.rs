// Answer 0

#[test]
fn test_as_char_word_end_half_unicode() {
    let look: Look = Look::WordEndHalfUnicode;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_start_half_unicode() {
    let look: Look = Look::WordStartHalfUnicode;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_end_half_ascii() {
    let look: Look = Look::WordEndHalfAscii;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_start_half_ascii() {
    let look: Look = Look::WordStartHalfAscii;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_end_unicode() {
    let look: Look = Look::WordEndUnicode;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_start_unicode() {
    let look: Look = Look::WordStartUnicode;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_ascii_negate() {
    let look: Look = Look::WordAsciiNegate;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_ascii() {
    let look: Look = Look::WordAscii;
    let _result = look.as_char();
}

