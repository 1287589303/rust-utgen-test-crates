// Answer 0

#[test]
fn test_contains_word_unicode_with_word_start_half_unicode() {
    let look_set = LookSet { bits: 1 << 14 }; // only Look::WordStartHalfUnicode is set
    let result = look_set.contains_word_unicode(); // should return true
}

#[test]
fn test_contains_word_unicode_with_word_start_half_unicode_and_no_others() {
    let look_set = LookSet { bits: 1 << 14 }; // only Look::WordStartHalfUnicode is set
    let result = look_set.contains_word_unicode(); // should return true
}

