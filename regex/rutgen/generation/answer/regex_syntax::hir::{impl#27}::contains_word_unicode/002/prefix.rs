// Answer 0

#[test]
fn test_contains_word_unicode_case_1() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::WordUnicodeNegate);
    let result = look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_case_2() {
    let look_set = LookSet { bits: Look::WordUnicodeNegate as u32 };
    let result = look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_case_3() {
    let look_set = LookSet { bits: Look::WordUnicodeNegate as u32 | Look::WordStartUnicode as u32 };
    let result = look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_case_4() {
    let look_set = LookSet { bits: Look::WordUnicodeNegate as u32 | Look::WordEndUnicode as u32 };
    let result = look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_case_5() {
    let look_set = LookSet { bits: Look::WordUnicodeNegate as u32 | Look::WordStartHalfUnicode as u32 };
    let result = look_set.contains_word_unicode();
}

#[test]
fn test_contains_word_unicode_case_6() {
    let look_set = LookSet { bits: Look::WordUnicodeNegate as u32 | Look::WordEndHalfUnicode as u32 };
    let result = look_set.contains_word_unicode();
}

