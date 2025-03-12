// Answer 0

#[test]
fn test_contains_word_ascii_with_word_ascii_negate() {
    let look_set = LookSet { bits: 0b00000000000000000000000000000010 };
    let _ = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_start_ascii() {
    let look_set = LookSet { bits: 0b00000000000000000000000000000100 };
    let _ = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_end_ascii() {
    let look_set = LookSet { bits: 0b00000000000000000000000000001000 };
    let _ = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_start_half_ascii() {
    let look_set = LookSet { bits: 0b00000000000000000000000000010000 };
    let _ = look_set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_word_end_half_ascii() {
    let look_set = LookSet { bits: 0b00000000000000000000100000000000 };
    let _ = look_set.contains_word_ascii();
}

