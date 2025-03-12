// Answer 0

#[test]
fn test_contains_anchor_haystack_with_start_only() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::Start);
    look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_with_start_and_end() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::Start);
    look_set.set_insert(Look::End);
    look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_with_end_only() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::End);
    look_set.contains_anchor_haystack();
}

#[test]
fn test_contains_anchor_haystack_with_no_anchors() {
    let look_set = LookSet { bits: 0 };
    look_set.contains_anchor_haystack();
}

