// Answer 0

#[test]
fn test_contains_word_ascii_with_word_start_half_ascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartHalfAscii);
    let result = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_start_half_ascii_and_no_others() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartHalfAscii);
    assert!(!look_set.contains(Look::WordAscii));
    assert!(!look_set.contains(Look::WordAsciiNegate));
    assert!(!look_set.contains(Look::WordStartAscii));
    assert!(!look_set.contains(Look::WordEndAscii));
    let result = look_set.contains_word_ascii();
}

