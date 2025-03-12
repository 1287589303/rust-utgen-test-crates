// Answer 0

#[test]
fn test_reversed_start_crlf() {
    let input = Look::StartCRLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_end_crlf() {
    let input = Look::EndCRLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_start() {
    let input = Look::Start;
    let output = input.reversed();
}

#[test]
fn test_reversed_end() {
    let input = Look::End;
    let output = input.reversed();
}

#[test]
fn test_reversed_startlf() {
    let input = Look::StartLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_endlf() {
    let input = Look::EndLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_ascii() {
    let input = Look::WordAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_ascii_negate() {
    let input = Look::WordAsciiNegate;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_unicode() {
    let input = Look::WordUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_unicode_negate() {
    let input = Look::WordUnicodeNegate;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_start_ascii() {
    let input = Look::WordStartAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_end_ascii() {
    let input = Look::WordEndAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_start_unicode() {
    let input = Look::WordStartUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_end_unicode() {
    let input = Look::WordEndUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_start_half_ascii() {
    let input = Look::WordStartHalfAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_end_half_ascii() {
    let input = Look::WordEndHalfAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_start_half_unicode() {
    let input = Look::WordStartHalfUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_word_end_half_unicode() {
    let input = Look::WordEndHalfUnicode;
    let output = input.reversed();
}

