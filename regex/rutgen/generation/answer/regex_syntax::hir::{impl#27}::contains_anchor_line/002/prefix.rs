// Answer 0

#[test]
fn test_contains_anchor_line_with_endlf_only() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_with_only_startlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
    assert!(!look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_with_start_crlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    assert!(!look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_with_end_crlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndCRLF);
    assert!(look_set.contains_anchor_line());
}

#[test]
fn test_contains_anchor_line_with_all_except_endlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndCRLF);
    assert!(look_set.contains_anchor_line());
}

