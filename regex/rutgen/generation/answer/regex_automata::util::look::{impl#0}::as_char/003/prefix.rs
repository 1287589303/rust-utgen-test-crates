// Answer 0

#[test]
fn test_look_word_end_half_ascii() {
    let look = Look::WordEndHalfAscii;
    let result = look.as_char();
}

#[test]
fn test_look_word_start_half_ascii() {
    let look = Look::WordStartHalfAscii;
    let result = look.as_char();
}

#[test]
fn test_look_word_end_ascii() {
    let look = Look::WordEndAscii;
    let result = look.as_char();
}

#[test]
fn test_look_word_start_ascii() {
    let look = Look::WordStartAscii;
    let result = look.as_char();
}

#[test]
fn test_look_word_end_unicode() {
    let look = Look::WordEndUnicode;
    let result = look.as_char();
}

#[test]
fn test_look_word_start_unicode() {
    let look = Look::WordStartUnicode;
    let result = look.as_char();
}

