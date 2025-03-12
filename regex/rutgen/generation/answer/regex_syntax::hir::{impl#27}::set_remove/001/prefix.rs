// Answer 0

#[test]
fn test_set_remove_start() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::Start);
}

#[test]
fn test_set_remove_end() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::End);
}

#[test]
fn test_set_remove_start_lf() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::StartLF);
}

#[test]
fn test_set_remove_end_lf() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::EndLF);
}

#[test]
fn test_set_remove_start_crlf() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::StartCRLF);
}

#[test]
fn test_set_remove_end_crlf() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::EndCRLF);
}

#[test]
fn test_set_remove_word_ascii() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordAscii);
}

#[test]
fn test_set_remove_word_ascii_negate() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordAsciiNegate);
}

#[test]
fn test_set_remove_word_unicode() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordUnicode);
}

#[test]
fn test_set_remove_word_unicode_negate() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordUnicodeNegate);
}

#[test]
fn test_set_remove_word_start_ascii() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordStartAscii);
}

#[test]
fn test_set_remove_word_end_ascii() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordEndAscii);
}

#[test]
fn test_set_remove_word_start_unicode() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordStartUnicode);
}

#[test]
fn test_set_remove_word_end_unicode() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordEndUnicode);
}

#[test]
fn test_set_remove_word_start_half_ascii() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordStartHalfAscii);
}

#[test]
fn test_set_remove_word_end_half_ascii() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordEndHalfAscii);
}

#[test]
fn test_set_remove_word_start_half_unicode() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordStartHalfUnicode);
}

#[test]
fn test_set_remove_word_end_half_unicode() {
    let mut set = LookSet { bits: LookSet::full().bits };
    set.set_remove(Look::WordEndHalfUnicode);
}

