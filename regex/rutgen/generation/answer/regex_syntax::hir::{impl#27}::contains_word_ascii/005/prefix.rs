// Answer 0

#[test]
fn test_contains_word_ascii_with_start_half() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::WordStartHalfAscii);
    let result = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_end_half() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::WordEndHalfAscii);
    let result = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_multiple_half() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::WordStartHalfAscii);
    look_set.set_insert(Look::WordEndHalfAscii);
    let result = look_set.contains_word_ascii();
}

