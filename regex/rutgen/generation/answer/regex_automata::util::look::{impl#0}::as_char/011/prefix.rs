// Answer 0

#[test]
fn test_as_char_word_ascii_negate() {
    let look = Look::WordAsciiNegate;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_start_ascii() {
    let look = Look::WordStartAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_end_ascii() {
    let look = Look::WordEndAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_unicode() {
    let look = Look::WordUnicode;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_unicode_negate() {
    let look = Look::WordUnicodeNegate;
    let _ = look.as_char();
}

