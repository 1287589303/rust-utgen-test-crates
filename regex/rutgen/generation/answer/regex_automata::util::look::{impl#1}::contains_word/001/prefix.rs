// Answer 0

#[test]
fn test_contains_word_with_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordUnicode);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_unicode_negate() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordUnicodeNegate);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_start_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartUnicode);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_end_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndUnicode);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_start_half_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartHalfUnicode);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_end_half_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndHalfUnicode);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_ascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordAscii);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_ascii_negate() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordAsciiNegate);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_start_ascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartAscii);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_end_ascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndAscii);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_start_half_ascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartHalfAscii);
    assert!(look_set.contains_word());
}

#[test]
fn test_contains_word_with_end_half_ascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndHalfAscii);
    assert!(look_set.contains_word());
}

