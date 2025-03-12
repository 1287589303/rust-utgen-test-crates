// Answer 0

#[test]
fn test_contains_word_unicode_empty() {
    let look_set = LookSet::empty();
    look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_full_exclusion() {
    let mut look_set = LookSet::full();
    look_set.set_remove(Look::WordUnicode);
    look_set.set_remove(Look::WordUnicodeNegate);
    look_set.set_remove(Look::WordStartUnicode);
    look_set.set_remove(Look::WordEndUnicode);
    look_set.set_remove(Look::WordStartHalfUnicode);
    look_set.set_remove(Look::WordEndHalfUnicode);
    look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_single_word_boundary() {
    let look_set = LookSet::singleton(Look::Start);
    look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_partial() {
    let look_set = LookSet {
        bits: Look::Start as u32 | Look::End as u32,
    };
    look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_some_other_look() {
    let look_set = LookSet {
        bits: Look::EndLF as u32,
    };
    look_set.contains_word_unicode();
}

