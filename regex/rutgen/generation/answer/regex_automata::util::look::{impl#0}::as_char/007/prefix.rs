// Answer 0

#[test]
fn test_as_char_word_end_ascii() {
    let look = Look::WordEndAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_start_ascii() {
    let look = Look::WordStartAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_end_unicode() {
    let look = Look::WordEndUnicode;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_start_unicode() {
    let look = Look::WordStartUnicode;
    let _ = look.as_char();
}

#[test]
fn test_as_char_start() {
    let look = Look::Start;
    let _ = look.as_char();
}

#[test]
fn test_as_char_end() {
    let look = Look::End;
    let _ = look.as_char();
}

#[test]
fn test_as_char_start_lf() {
    let look = Look::StartLF;
    let _ = look.as_char();
}

#[test]
fn test_as_char_end_lf() {
    let look = Look::EndLF;
    let _ = look.as_char();
}

#[test]
fn test_as_char_start_crlf() {
    let look = Look::StartCRLF;
    let _ = look.as_char();
}

#[test]
fn test_as_char_end_crlf() {
    let look = Look::EndCRLF;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_ascii() {
    let look = Look::WordAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_ascii_negate() {
    let look = Look::WordAsciiNegate;
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

#[test]
fn test_as_char_word_start_half_ascii() {
    let look = Look::WordStartHalfAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_end_half_ascii() {
    let look = Look::WordEndHalfAscii;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_start_half_unicode() {
    let look = Look::WordStartHalfUnicode;
    let _ = look.as_char();
}

#[test]
fn test_as_char_word_end_half_unicode() {
    let look = Look::WordEndHalfUnicode;
    let _ = look.as_char();
}

