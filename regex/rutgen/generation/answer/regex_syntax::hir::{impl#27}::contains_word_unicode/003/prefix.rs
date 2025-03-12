// Answer 0

#[test]
fn test_contains_word_unicode_with_word_start_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartUnicode);
    assert!(look_set.contains_word_unicode());
}

#[test]
fn test_contains_word_unicode_with_word_start_unicode_no_conflicts() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartUnicode);
    look_set.set_remove(Look::WordUnicode);
    look_set.set_remove(Look::WordUnicodeNegate);
    look_set.set_remove(Look::WordEndUnicode);
    look_set.set_remove(Look::WordStartHalfUnicode);
    look_set.set_remove(Look::WordEndHalfUnicode);
    assert!(look_set.contains_word_unicode());
}

