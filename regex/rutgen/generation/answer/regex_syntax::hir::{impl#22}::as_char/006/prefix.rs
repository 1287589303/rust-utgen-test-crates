// Answer 0

#[test]
fn test_as_char_word_start_unicode() {
    let look = Look::WordStartUnicode;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_start_ascii() {
    let look = Look::WordStartAscii;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_end_unicode() {
    let look = Look::WordEndUnicode;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_end_ascii() {
    let look = Look::WordEndAscii;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_unicode() {
    let look = Look::WordUnicode;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_unicde_negate() {
    let look = Look::WordUnicodeNegate;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_ascii() {
    let look = Look::WordAscii;
    let _result = look.as_char();
}

#[test]
fn test_as_char_word_ascii_negate() {
    let look = Look::WordAsciiNegate;
    let _result = look.as_char();
}

