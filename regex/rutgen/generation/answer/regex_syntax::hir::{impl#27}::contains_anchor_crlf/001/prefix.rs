// Answer 0

#[test]
fn test_contains_anchor_crlf_with_start_crlf_only() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    let _ = look_set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_end_crlf_only() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndCRLF);
    let _ = look_set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_both_start_and_end_crlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    look_set.set_insert(Look::EndCRLF);
    let _ = look_set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_no_crlf() {
    let mut look_set = LookSet::empty();
    let _ = look_set.contains_anchor_crlf();
}

