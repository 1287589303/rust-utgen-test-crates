// Answer 0

#[test]
fn test_as_char_word_ascii_negate() {
    let look = Look::WordAsciiNegate;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_start_ascii() {
    let look = Look::WordStartAscii;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_end_ascii() {
    let look = Look::WordEndAscii;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_unicode() {
    let look = Look::WordUnicode;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_unicode_negate() {
    let look = Look::WordUnicodeNegate;
    let result = look.as_char();
} 

#[test]
fn test_as_char_start() {
    let look = Look::Start;
    let result = look.as_char();
} 

#[test]
fn test_as_char_end() {
    let look = Look::End;
    let result = look.as_char();
} 

#[test]
fn test_as_char_start_lf() {
    let look = Look::StartLF;
    let result = look.as_char();
} 

#[test]
fn test_as_char_end_lf() {
    let look = Look::EndLF;
    let result = look.as_char();
} 

#[test]
fn test_as_char_start_crlf() {
    let look = Look::StartCRLF;
    let result = look.as_char();
} 

#[test]
fn test_as_char_end_crlf() {
    let look = Look::EndCRLF;
    let result = look.as_char();
} 

#[test]
fn test_as_char_word_start_unicode() {
    let look = Look::WordStartUnicode;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_end_unicode() {
    let look = Look::WordEndUnicode;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_start_half_ascii() {
    let look = Look::WordStartHalfAscii;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_end_half_ascii() {
    let look = Look::WordEndHalfAscii;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_start_half_unicode() {
    let look = Look::WordStartHalfUnicode;
    let result = look.as_char();
}

#[test]
fn test_as_char_word_end_half_unicode() {
    let look = Look::WordEndHalfUnicode;
    let result = look.as_char();
}

