// Answer 0

#[test]
fn test_available_with_word_unicode_assertion() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordUnicode);
    let result = look_set.available();
}

#[test]
fn test_available_with_word_unicode_negate() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordUnicodeNegate);
    let result = look_set.available();
}

#[test]
fn test_available_with_word_start_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartUnicode);
    let result = look_set.available();
}

#[test]
fn test_available_with_word_end_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndUnicode);
    let result = look_set.available();
}

#[test]
fn test_available_with_word_start_half_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartHalfUnicode);
    let result = look_set.available();
}

#[test]
fn test_available_with_word_end_half_unicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndHalfUnicode);
    let result = look_set.available();
}

