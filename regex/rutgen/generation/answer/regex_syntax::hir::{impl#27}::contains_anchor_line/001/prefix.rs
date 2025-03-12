// Answer 0

#[test]
fn test_contains_anchor_line_start_lf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_end_lf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_start_crlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_end_crlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndCRLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_combination_lf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_combination_crlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    look_set.set_insert(Look::EndCRLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_combination_all() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndLF);
    look_set.set_insert(Look::StartCRLF);
    look_set.set_insert(Look::EndCRLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_no_anchor() {
    let look_set = LookSet::empty();
    assert!(!look_set.contains_anchor_line());
}

