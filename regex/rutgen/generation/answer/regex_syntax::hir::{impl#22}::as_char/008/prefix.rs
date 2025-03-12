// Answer 0

#[test]
fn test_look_word_start_ascii_as_char() {
    let look = Look::WordStartAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_start_as_char() {
    let look = Look::Start;
    let _ = look.as_char();
}

#[test]
fn test_look_end_as_char() {
    let look = Look::End;
    let _ = look.as_char();
}

#[test]
fn test_look_start_lf_as_char() {
    let look = Look::StartLF;
    let _ = look.as_char();
}

#[test]
fn test_look_end_lf_as_char() {
    let look = Look::EndLF;
    let _ = look.as_char();
}

#[test]
fn test_look_start_crlf_as_char() {
    let look = Look::StartCRLF;
    let _ = look.as_char();
}

#[test]
fn test_look_end_crlf_as_char() {
    let look = Look::EndCRLF;
    let _ = look.as_char();
}

#[test]
fn test_look_word_ascii_as_char() {
    let look = Look::WordAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_word_ascii_negate_as_char() {
    let look = Look::WordAsciiNegate;
    let _ = look.as_char();
}

#[test]
fn test_look_word_unicode_as_char() {
    let look = Look::WordUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_word_unicode_negate_as_char() {
    let look = Look::WordUnicodeNegate;
    let _ = look.as_char();
}

#[test]
fn test_look_word_start_unicode_as_char() {
    let look = Look::WordStartUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_word_end_unicode_as_char() {
    let look = Look::WordEndUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_word_start_half_ascii_as_char() {
    let look = Look::WordStartHalfAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_word_end_half_ascii_as_char() {
    let look = Look::WordEndHalfAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_word_start_half_unicode_as_char() {
    let look = Look::WordStartHalfUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_word_end_half_unicode_as_char() {
    let look = Look::WordEndHalfUnicode;
    let _ = look.as_char();
}

