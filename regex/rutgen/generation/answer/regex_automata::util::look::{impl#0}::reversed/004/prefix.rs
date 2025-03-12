// Answer 0

#[test]
fn test_reversed_start() {
    let look = Look::Start;
    let _ = look.reversed();
}

#[test]
fn test_reversed_end() {
    let look = Look::End;
    let _ = look.reversed();
}

#[test]
fn test_reversed_start_lf() {
    let look = Look::StartLF;
    let _ = look.reversed();
}

#[test]
fn test_reversed_end_lf() {
    let look = Look::EndLF;
    let _ = look.reversed();
}

#[test]
fn test_reversed_start_crlf() {
    let look = Look::StartCRLF;
    let _ = look.reversed();
}

#[test]
fn test_reversed_end_crlf() {
    let look = Look::EndCRLF;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_ascii() {
    let look = Look::WordAscii;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_ascii_negate() {
    let look = Look::WordAsciiNegate;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_unicode() {
    let look = Look::WordUnicode;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_unicode_negate() {
    let look = Look::WordUnicodeNegate;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_start_ascii() {
    let look = Look::WordStartAscii;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_end_ascii() {
    let look = Look::WordEndAscii;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_start_unicode() {
    let look = Look::WordStartUnicode;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_end_unicode() {
    let look = Look::WordEndUnicode;
    let _ = look.reversed();
}

#[test]
fn test_reversed_word_start_half_ascii() {
    let look = Look::WordStartHalfAscii;
    let result = look.reversed();
    assert_eq!(result, Look::WordEndHalfAscii);
}

#[test]
fn test_reversed_word_end_half_ascii() {
    let look = Look::WordEndHalfAscii;
    let result = look.reversed();
    assert_eq!(result, Look::WordStartHalfAscii);
}

#[test]
fn test_reversed_word_start_half_unicode() {
    let look = Look::WordStartHalfUnicode;
    let result = look.reversed();
    assert_eq!(result, Look::WordEndHalfUnicode);
}

#[test]
fn test_reversed_word_end_half_unicode() {
    let look = Look::WordEndHalfUnicode;
    let result = look.reversed();
    assert_eq!(result, Look::WordStartHalfUnicode);
}

