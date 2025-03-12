// Answer 0

#[test]
fn test_reversed_start() {
    let result = Look::Start.reversed();
}

#[test]
fn test_reversed_end() {
    let result = Look::End.reversed();
}

#[test]
fn test_reversed_startlf() {
    let result = Look::StartLF.reversed();
}

#[test]
fn test_reversed_endlf() {
    let result = Look::EndLF.reversed();
}

#[test]
fn test_reversed_startcrlf() {
    let result = Look::StartCRLF.reversed();
}

#[test]
fn test_reversed_endcrlf() {
    let result = Look::EndCRLF.reversed();
}

#[test]
fn test_reversed_word_ascii() {
    let result = Look::WordAscii.reversed();
}

#[test]
fn test_reversed_word_ascii_negate() {
    let result = Look::WordAsciiNegate.reversed();
}

#[test]
fn test_reversed_word_unicode() {
    let result = Look::WordUnicode.reversed();
}

#[test]
fn test_reversed_word_unicode_negate() {
    let result = Look::WordUnicodeNegate.reversed();
}

#[test]
fn test_reversed_word_start_ascii() {
    let result = Look::WordStartAscii.reversed();
}

#[test]
fn test_reversed_word_end_ascii() {
    let result = Look::WordEndAscii.reversed();
}

#[test]
fn test_reversed_word_start_unicode() {
    let result = Look::WordStartUnicode.reversed();
}

#[test]
fn test_reversed_word_end_unicode() {
    let result = Look::WordEndUnicode.reversed();
}

#[test]
fn test_reversed_word_start_half_ascii() {
    let result = Look::WordStartHalfAscii.reversed();
}

#[test]
fn test_reversed_word_end_half_ascii() {
    let result = Look::WordEndHalfAscii.reversed();
}

#[test]
fn test_reversed_word_start_half_unicode() {
    let result = Look::WordStartHalfUnicode.reversed();
}

#[test]
fn test_reversed_word_end_half_unicode() {
    let result = Look::WordEndHalfUnicode.reversed();
}

