// Answer 0

#[test]
fn test_contains_word_ascii_with_word_ascii() {
    let look_set = LookSet { bits: 1 << 6 }; // Look::WordAscii
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_ascii_negate() {
    let look_set = LookSet { bits: 1 << 7 }; // Look::WordAsciiNegate
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_start_ascii() {
    let look_set = LookSet { bits: 1 << 10 }; // Look::WordStartAscii
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_end_ascii() {
    let look_set = LookSet { bits: 1 << 11 }; // Look::WordEndAscii
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_start_half_ascii() {
    let look_set = LookSet { bits: 1 << 14 }; // Look::WordStartHalfAscii
    look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_end_half_ascii() {
    let look_set = LookSet { bits: 1 << 15 }; // Look::WordEndHalfAscii
    look_set.contains_word_ascii();
}

