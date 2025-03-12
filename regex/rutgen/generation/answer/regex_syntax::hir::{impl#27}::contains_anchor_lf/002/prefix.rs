// Answer 0

#[test]
fn test_contains_anchor_lf_empty() {
    let look_set = LookSet::empty();
    let result = look_set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_only_endlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndLF);
    let result = look_set.contains_anchor_lf();
}

