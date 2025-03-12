// Answer 0

#[test]
fn test_contains_anchor_haystack_when_only_end() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::End);
    look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_when_full() {
    let look_set = LookSet::full();
    look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_when_only_start() {
    let look_set = LookSet::singleton(Look::Start);
    look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_when_empty() {
    let look_set = LookSet::empty();
    look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_without_start() {
    let look_set = LookSet::singleton(Look::WordAscii);
    look_set.contains_anchor_haystack();
}

