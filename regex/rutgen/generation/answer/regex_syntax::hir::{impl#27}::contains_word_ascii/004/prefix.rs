// Answer 0

#[test]
fn test_contains_word_ascii_case1() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::WordEndAscii);
    assert!(lookset.contains_word_ascii());
}

#[test]
fn test_contains_word_ascii_case2() {
    let mut lookset = LookSet::full();
    lookset.set_remove(Look::WordAscii);
    lookset.set_remove(Look::WordAsciiNegate);
    lookset.set_remove(Look::WordStartAscii);
    lookset.set_remove(Look::WordStartHalfAscii);
    lookset.set_remove(Look::WordEndHalfAscii);
    lookset.set_insert(Look::WordEndAscii);
    assert!(lookset.contains_word_ascii());
}

#[test]
fn test_contains_word_ascii_case3() {
    let lookset = LookSet::singleton(Look::WordEndAscii);
    assert!(lookset.contains_word_ascii());
}

#[test]
#[should_panic]
fn test_contains_word_ascii_fail_case() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::WordAscii);
    let _ = lookset.contains_word_ascii();
}

