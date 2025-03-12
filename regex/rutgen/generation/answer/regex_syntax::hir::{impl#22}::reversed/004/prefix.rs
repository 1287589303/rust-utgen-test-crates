// Answer 0

#[test]
fn test_reversed_start() {
    let look = Look::Start;
    let _result = look.reversed();
}

#[test]
fn test_reversed_end() {
    let look = Look::End;
    let _result = look.reversed();
}

#[test]
fn test_reversed_start_lf() {
    let look = Look::StartLF;
    let _result = look.reversed();
}

#[test]
fn test_reversed_end_lf() {
    let look = Look::EndLF;
    let _result = look.reversed();
}

#[test]
fn test_reversed_start_crlf() {
    let look = Look::StartCRLF;
    let _result = look.reversed();
}

#[test]
fn test_reversed_end_crlf() {
    let look = Look::EndCRLF;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_ascii() {
    let look = Look::WordAscii;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_ascii_negate() {
    let look = Look::WordAsciiNegate;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_unicode() {
    let look = Look::WordUnicode;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_unicode_negate() {
    let look = Look::WordUnicodeNegate;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_start_ascii() {
    let look = Look::WordStartAscii;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_end_ascii() {
    let look = Look::WordEndAscii;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_start_unicode() {
    let look = Look::WordStartUnicode;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_end_unicode() {
    let look = Look::WordEndUnicode;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_start_half_ascii() {
    let look = Look::WordStartHalfAscii;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_end_half_ascii() {
    let look = Look::WordEndHalfAscii;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_start_half_unicode() {
    let look = Look::WordStartHalfUnicode;
    let _result = look.reversed();
}

#[test]
fn test_reversed_word_end_half_unicode() {
    let look = Look::WordEndHalfUnicode;
    let _result = look.reversed();
}

