// Answer 0

#[test]
fn test_singleton_start() {
    let look_set = LookSet::singleton(Look::Start);
}

#[test]
fn test_singleton_end() {
    let look_set = LookSet::singleton(Look::End);
}

#[test]
fn test_singleton_start_lf() {
    let look_set = LookSet::singleton(Look::StartLF);
}

#[test]
fn test_singleton_end_lf() {
    let look_set = LookSet::singleton(Look::EndLF);
}

#[test]
fn test_singleton_start_crlf() {
    let look_set = LookSet::singleton(Look::StartCRLF);
}

#[test]
fn test_singleton_end_crlf() {
    let look_set = LookSet::singleton(Look::EndCRLF);
}

#[test]
fn test_singleton_word_ascii() {
    let look_set = LookSet::singleton(Look::WordAscii);
}

#[test]
fn test_singleton_word_ascii_negate() {
    let look_set = LookSet::singleton(Look::WordAsciiNegate);
}

#[test]
fn test_singleton_word_unicode() {
    let look_set = LookSet::singleton(Look::WordUnicode);
}

#[test]
fn test_singleton_word_unicode_negate() {
    let look_set = LookSet::singleton(Look::WordUnicodeNegate);
}

#[test]
fn test_singleton_word_start_ascii() {
    let look_set = LookSet::singleton(Look::WordStartAscii);
}

#[test]
fn test_singleton_word_end_ascii() {
    let look_set = LookSet::singleton(Look::WordEndAscii);
}

#[test]
fn test_singleton_word_start_unicode() {
    let look_set = LookSet::singleton(Look::WordStartUnicode);
}

#[test]
fn test_singleton_word_end_unicode() {
    let look_set = LookSet::singleton(Look::WordEndUnicode);
}

#[test]
fn test_singleton_word_start_half_ascii() {
    let look_set = LookSet::singleton(Look::WordStartHalfAscii);
}

#[test]
fn test_singleton_word_end_half_ascii() {
    let look_set = LookSet::singleton(Look::WordEndHalfAscii);
}

#[test]
fn test_singleton_word_start_half_unicode() {
    let look_set = LookSet::singleton(Look::WordStartHalfUnicode);
}

#[test]
fn test_singleton_word_end_half_unicode() {
    let look_set = LookSet::singleton(Look::WordEndHalfUnicode);
}

