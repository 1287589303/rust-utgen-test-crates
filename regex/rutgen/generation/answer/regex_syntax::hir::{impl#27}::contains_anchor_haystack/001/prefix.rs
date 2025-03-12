// Answer 0

#[test]
fn test_contains_anchor_haystack_with_start() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::Start);
    let result = look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_with_end() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::End);
    let result = look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_with_both() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::Start);
    look_set.set_insert(Look::End);
    let result = look_set.contains_anchor_haystack();
}

