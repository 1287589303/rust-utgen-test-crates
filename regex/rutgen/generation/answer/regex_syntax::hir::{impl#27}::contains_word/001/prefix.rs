// Answer 0

#[test]
fn test_contains_word_empty() {
    let look_set = LookSet::empty();
    let result = look_set.contains_word();
}

#[test]
fn test_contains_word_unicode_only() {
    let look_set = LookSet::singleton(Look::WordUnicode);
    let result = look_set.contains_word();
}

#[test]
fn test_contains_word_ascii_only() {
    let look_set = LookSet::singleton(Look::WordAscii);
    let result = look_set.contains_word();
}

#[test]
fn test_contains_word_both_unicode_and_ascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordUnicode);
    look_set.set_insert(Look::WordAscii);
    let result = look_set.contains_word();
}

