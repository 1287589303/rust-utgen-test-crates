// Answer 0

#[test]
fn test_look_start() {
    let result = Hir::look(Look::Start);
}

#[test]
fn test_look_end() {
    let result = Hir::look(Look::End);
}

#[test]
fn test_look_start_lf() {
    let result = Hir::look(Look::StartLF);
}

#[test]
fn test_look_end_lf() {
    let result = Hir::look(Look::EndLF);
}

#[test]
fn test_look_start_crlf() {
    let result = Hir::look(Look::StartCRLF);
}

#[test]
fn test_look_end_crlf() {
    let result = Hir::look(Look::EndCRLF);
}

#[test]
fn test_look_word_ascii() {
    let result = Hir::look(Look::WordAscii);
}

#[test]
fn test_look_word_ascii_negate() {
    let result = Hir::look(Look::WordAsciiNegate);
}

#[test]
fn test_look_word_unicode() {
    let result = Hir::look(Look::WordUnicode);
}

#[test]
fn test_look_word_unicode_negate() {
    let result = Hir::look(Look::WordUnicodeNegate);
}

#[test]
fn test_look_word_start_ascii() {
    let result = Hir::look(Look::WordStartAscii);
}

#[test]
fn test_look_word_end_ascii() {
    let result = Hir::look(Look::WordEndAscii);
}

#[test]
fn test_look_word_start_unicode() {
    let result = Hir::look(Look::WordStartUnicode);
}

#[test]
fn test_look_word_end_unicode() {
    let result = Hir::look(Look::WordEndUnicode);
}

#[test]
fn test_look_word_start_half_ascii() {
    let result = Hir::look(Look::WordStartHalfAscii);
}

#[test]
fn test_look_word_end_half_ascii() {
    let result = Hir::look(Look::WordEndHalfAscii);
}

#[test]
fn test_look_word_start_half_unicode() {
    let result = Hir::look(Look::WordStartHalfUnicode);
}

#[test]
fn test_look_word_end_half_unicode() {
    let result = Hir::look(Look::WordEndHalfUnicode);
}

