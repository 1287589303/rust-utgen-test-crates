// Answer 0

#[test]
fn test_contains_word_ascii_with_start_set() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartAscii);
    look_set.set_remove(Look::WordAscii);
    look_set.set_remove(Look::WordAsciiNegate);
    look_set.set_remove(Look::WordEndAscii);
    look_set.set_remove(Look::WordStartHalfAscii);
    look_set.set_remove(Look::WordEndHalfAscii);
    let result = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_start_set() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartAscii);
    look_set.set_remove(Look::WordAscii);
    look_set.set_remove(Look::WordAsciiNegate);
    look_set.set_remove(Look::WordEndAscii);
    look_set.set_remove(Look::WordStartHalfAscii);
    look_set.set_remove(Look::WordEndHalfAscii);
    let result = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_multiple_inserts() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartAscii);
    look_set.set_insert(Look::WordStartHalfUnicode);
    look_set.set_remove(Look::WordAscii);
    look_set.set_remove(Look::WordAsciiNegate);
    look_set.set_remove(Look::WordEndAscii);
    look_set.set_remove(Look::WordStartHalfAscii);
    look_set.set_remove(Look::WordEndHalfAscii);
    let result = look_set.contains_word_ascii();
}

