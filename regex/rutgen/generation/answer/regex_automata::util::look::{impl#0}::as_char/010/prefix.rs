// Answer 0

#[test]
fn test_look_as_char_word_unicode() {
    let input = Look::WordUnicode;
    let _output = input.as_char();
}

#[test]
fn test_look_as_char_word_ascii() {
    let input = Look::WordAscii;
    let _output = input.as_char();
}

#[test]
fn test_look_as_char_start() {
    let input = Look::Start;
    let _output = input.as_char();
}

#[test]
fn test_look_as_char_end() {
    let input = Look::End;
    let _output = input.as_char();
}

#[test]
fn test_look_as_char_start_lf() {
    let input = Look::StartLF;
    let _output = input.as_char();
}

#[test]
fn test_look_as_char_end_lf() {
    let input = Look::EndLF;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_start_crlf() {
    let input = Look::StartCRLF;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_end_crlf() {
    let input = Look::EndCRLF;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_ascii_negate() {
    let input = Look::WordAsciiNegate;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_unicode_negate() {
    let input = Look::WordUnicodeNegate;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_start_ascii() {
    let input = Look::WordStartAscii;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_end_ascii() {
    let input = Look::WordEndAscii;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_start_unicode() {
    let input = Look::WordStartUnicode;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_end_unicode() {
    let input = Look::WordEndUnicode;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_start_half_ascii() {
    let input = Look::WordStartHalfAscii;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_end_half_ascii() {
    let input = Look::WordEndHalfAscii;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_start_half_unicode() {
    let input = Look::WordStartHalfUnicode;
    let _output = input.as_char();
} 

#[test]
fn test_look_as_char_word_end_half_unicode() {
    let input = Look::WordEndHalfUnicode;
    let _output = input.as_char();
}

