// Answer 0

#[test]
fn test_contains_word_ascii_empty() {
    let set = LookSet { bits: 0b0000000000000000 };
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_full() {
    let set = LookSet { bits: 0b0000000000000000 };
    set.insert(Look::WordAscii);
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_negation() {
    let set = LookSet { bits: 0b0000000000000000 };
    set.insert(Look::WordAsciiNegate);
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_start() {
    let set = LookSet { bits: 0b0000000000000000 };
    set.insert(Look::WordStartAscii);
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_end() {
    let set = LookSet { bits: 0b0000000000000000 };
    set.insert(Look::WordEndAscii);
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_start_half() {
    let set = LookSet { bits: 0b0000000000000000 };
    set.insert(Look::WordStartHalfAscii);
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_end_half() {
    let set = LookSet { bits: 0b0000000000000000 };
    set.insert(Look::WordEndHalfAscii);
    set.contains_word_ascii();
}

