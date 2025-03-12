// Answer 0

#[test]
fn test_contains_word_empty() {
    let lookset = LookSet::empty();
    lookset.contains_word();
}

#[test]
fn test_contains_word_full() {
    let lookset = LookSet::full();
    lookset.contains_word();
}

#[test]
fn test_contains_word_single_word_ascii() {
    let lookset = LookSet::singleton(Look::WordAscii);
    lookset.contains_word();
}

#[test]
fn test_contains_word_single_word_ascii_negate() {
    let lookset = LookSet::singleton(Look::WordAsciiNegate);
    lookset.contains_word();
}

#[test]
fn test_contains_word_start_half_ascii() {
    let lookset = LookSet::singleton(Look::WordStartHalfAscii);
    lookset.contains_word();
}

#[test]
fn test_contains_word_end_half_ascii() {
    let lookset = LookSet::singleton(Look::WordEndHalfAscii);
    lookset.contains_word();
}

#[test]
fn test_contains_word_disjoint_look_variants() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::WordAscii);
    lookset.set_insert(Look::WordStartHalfAscii);
    lookset.contains_word();
}

#[test]
fn test_contains_word_overlapping_look_variants() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::WordAscii);
    lookset.set_insert(Look::WordAsciiNegate);
    lookset.contains_word();
}

