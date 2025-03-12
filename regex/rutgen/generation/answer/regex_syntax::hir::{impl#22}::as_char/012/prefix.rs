// Answer 0

#[test]
fn test_look_word_ascii_as_char() {
    let look = Look::WordAscii;
    let _result = look.as_char();
}

#[test]
fn test_look_word_ascii_negate_as_char() {
    let look = Look::WordAsciiNegate;
    let _result = look.as_char();
}

#[test]
fn test_look_start_as_char() {
    let look = Look::Start;
    let _result = look.as_char();
}

#[test]
fn test_look_end_as_char() {
    let look = Look::End;
    let _result = look.as_char();
}

#[test]
fn test_look_start_lf_as_char() {
    let look = Look::StartLF;
    let _result = look.as_char();
}

#[test]
fn test_look_end_lf_as_char() {
    let look = Look::EndLF;
    let _result = look.as_char();
}

#[test]
fn test_look_start_crlf_as_char() {
    let look = Look::StartCRLF;
    let _result = look.as_char();
}

#[test]
fn test_look_end_crlf_as_char() {
    let look = Look::EndCRLF;
    let _result = look.as_char();
}

#[test]
fn test_look_word_unicode_as_char() {
    let look = Look::WordUnicode;
    let _result = look.as_char();
}

#[test]
fn test_look_word_unicode_negate_as_char() {
    let look = Look::WordUnicodeNegate;
    let _result = look.as_char();
}

#[test]
fn test_look_word_start_ascii_as_char() {
    let look = Look::WordStartAscii;
    let _result = look.as_char();
}

#[test]
fn test_look_word_end_ascii_as_char() {
    let look = Look::WordEndAscii;
    let _result = look.as_char();
}

#[test]
fn test_look_word_start_unicode_as_char() {
    let look = Look::WordStartUnicode;
    let _result = look.as_char();
}

#[test]
fn test_look_word_end_unicode_as_char() {
    let look = Look::WordEndUnicode;
    let _result = look.as_char();
}

#[test]
fn test_look_word_start_half_ascii_as_char() {
    let look = Look::WordStartHalfAscii;
    let _result = look.as_char();
}

#[test]
fn test_look_word_end_half_ascii_as_char() {
    let look = Look::WordEndHalfAscii;
    let _result = look.as_char();
}

#[test]
fn test_look_word_start_half_unicode_as_char() {
    let look = Look::WordStartHalfUnicode;
    let _result = look.as_char();
}

#[test]
fn test_look_word_end_half_unicode_as_char() {
    let look = Look::WordEndHalfUnicode;
    let _result = look.as_char();
}

