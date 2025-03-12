// Answer 0

#[test]
fn test_contains_anchor_line_with_end_crlf_true() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndCRLF);
    let result = look_set.contains_anchor_line();
}

#[test]
fn test_contains_anchor_line_with_only_start_lf_false() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
    let result = look_set.contains_anchor_line();
}

#[test]
fn test_contains_anchor_line_with_only_end_lf_false() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndLF);
    let result = look_set.contains_anchor_line();
}

#[test]
fn test_contains_anchor_line_with_only_start_crlf_false() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartCRLF);
    let result = look_set.contains_anchor_line();
}

