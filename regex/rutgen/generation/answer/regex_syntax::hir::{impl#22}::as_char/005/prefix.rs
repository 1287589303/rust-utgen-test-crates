// Answer 0

#[test]
fn test_look_as_char_word_end_unicode() {
    let look = Look::WordEndUnicode;
    let _result = look.as_char();
}

#[test]
fn test_look_as_char_start_half_unicode() {
    let look = Look::WordStartHalfUnicode;
    let _result = look.as_char();
}

#[test]
fn test_look_as_char_word_end_half_unicode() {
    let look = Look::WordEndHalfUnicode;
    let _result = look.as_char();
}

