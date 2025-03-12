// Answer 0

#[test]
fn test_reversed_word_ascii() {
    let look = Look::WordAscii;
    let result = look.reversed();
}

#[test]
fn test_reversed_word_ascii_negate() {
    let look = Look::WordAsciiNegate;
    let result = look.reversed();
}

#[test]
fn test_reversed_word_unicode() {
    let look = Look::WordUnicode;
    let result = look.reversed();
}

#[test]
fn test_reversed_word_unicode_negate() {
    let look = Look::WordUnicodeNegate;
    let result = look.reversed();
}

