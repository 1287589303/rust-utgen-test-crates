// Answer 0

#[test]
fn test_contains_anchor_with_haystack_false_and_line_true() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndLF);
    look_set.set_insert(Look::StartCRLF);
    look_set.set_insert(Look::EndCRLF);
    look_set.contains_anchor();
}

#[test]
fn test_contains_anchor_with_haystack_false_and_line_false() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndLF);
    look_set.contains_anchor();
}

#[test]
fn test_contains_anchor_with_no_anchors() {
    let look_set = LookSet::empty();
    look_set.contains_anchor();
}

