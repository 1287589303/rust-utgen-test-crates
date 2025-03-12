// Answer 0

#[test]
fn test_contains_anchor_crlf_start_only() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    let result = look_set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_start_and_end() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    look_set.set_insert(Look::EndCRLF);
    let result = look_set.contains_anchor_crlf();
}

