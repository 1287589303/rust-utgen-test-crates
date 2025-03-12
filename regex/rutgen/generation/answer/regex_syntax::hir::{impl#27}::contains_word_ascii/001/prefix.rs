// Answer 0

#[test]
fn test_contains_word_ascii_with_WordAscii() {
    let set = LookSet { bits: Look::WordAscii as u32 };
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_WordAsciiNegate() {
    let set = LookSet { bits: Look::WordAsciiNegate as u32 };
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_WordStartAscii() {
    let set = LookSet { bits: Look::WordStartAscii as u32 };
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_WordEndAscii() {
    let set = LookSet { bits: Look::WordEndAscii as u32 };
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_WordStartHalfAscii() {
    let set = LookSet { bits: Look::WordStartHalfAscii as u32 };
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_WordEndHalfAscii() {
    let set = LookSet { bits: Look::WordEndHalfAscii as u32 };
    set.contains_word_ascii();
}

#[test]
fn test_contains_word_ascii_with_multiple_flags() {
    let set = LookSet { bits: Look::WordAscii as u32 | Look::WordStartAscii as u32 | Look::WordEndAscii as u32 };
    set.contains_word_ascii();
}

