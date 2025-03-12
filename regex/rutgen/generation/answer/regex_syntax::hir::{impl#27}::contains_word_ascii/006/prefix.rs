// Answer 0

#[test]
fn test_contains_word_ascii_empty_set() {
    let look_set = LookSet::empty();
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_full_set() {
    let look_set = LookSet::full();
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_singleton_word_ascii() {
    let look_set = LookSet::singleton(Look::WordAscii);
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_singleton_word_ascii_negate() {
    let look_set = LookSet::singleton(Look::WordAsciiNegate);
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_singleton_word_start_ascii() {
    let look_set = LookSet::singleton(Look::WordStartAscii);
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_singleton_word_end_ascii() {
    let look_set = LookSet::singleton(Look::WordEndAscii);
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_singleton_word_start_half_ascii() {
    let look_set = LookSet::singleton(Look::WordStartHalfAscii);
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_singleton_word_end_half_ascii() {
    let look_set = LookSet::singleton(Look::WordEndHalfAscii);
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_combination() {
    let look_set = LookSet::empty()
        .insert(Look::WordStartHalfAscii)
        .insert(Look::WordEndHalfAscii);
    look_set.contains_word_ascii();
}

