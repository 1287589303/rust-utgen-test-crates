// Answer 0

#[test]
fn test_contains_word_unicode_with_word_end_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndUnicode);
    assert!(look_set.contains_word_unicode());
}

#[test]
fn test_contains_word_unicode_without_other_unicode_looks() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndUnicode);
    assert!(!look_set.contains(Look::WordUnicode));
    assert!(!look_set.contains(Look::WordUnicodeNegate));
    assert!(!look_set.contains(Look::WordStartUnicode));
} 

#[test]
fn test_contains_word_unicode_with_multiple_inserts() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndUnicode);
    look_set.set_insert(Look::Start);
    assert!(look_set.contains_word_unicode());
}

#[test]
fn test_contains_word_unicode_with_no_lookup() {
    let look_set = LookSet::empty();
    assert!(!look_set.contains_word_unicode());
}

