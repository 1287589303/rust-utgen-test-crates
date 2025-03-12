// Answer 0

#[test]
fn test_as_char_word_end_half_ascii() {
    let look_around = Look::WordEndHalfAscii;
    let _ = look_around.as_char();
}

#[test]
fn test_as_char_word_start_half_ascii() {
    let look_around = Look::WordStartHalfAscii;
    let _ = look_around.as_char();
}

#[test]
fn test_as_char_word_start_half_unicode() {
    let look_around = Look::WordStartHalfUnicode;
    let _ = look_around.as_char();
}

#[test]
fn test_as_char_word_end_half_unicode() {
    let look_around = Look::WordEndHalfUnicode;
    let _ = look_around.as_char();
}

#[test]
fn test_as_char_word_start_ascii() {
    let look_around = Look::WordStartAscii;
    let _ = look_around.as_char();
}

#[test]
fn test_as_char_word_end_ascii() {
    let look_around = Look::WordEndAscii;
    let _ = look_around.as_char();
}

