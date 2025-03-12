// Answer 0

#[test]
fn test_as_char_word_start_half_unicode() {
    let look = Look::WordStartHalfUnicode;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_start_half_ascii() {
    let look = Look::WordStartHalfAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_end_half_unicode() {
    let look = Look::WordEndHalfUnicode;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_end_half_ascii() {
    let look = Look::WordEndHalfAscii;
    let _ = look.as_char();
}

