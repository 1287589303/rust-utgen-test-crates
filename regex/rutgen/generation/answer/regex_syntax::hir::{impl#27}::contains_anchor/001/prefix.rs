// Answer 0

#[test]
fn test_contains_anchor_haystack_only_start() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::Start);
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_haystack_only_end() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::End);
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_haystack_start_and_end() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::Start);
    lookset.set_insert(Look::End);
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_line_only_start_lf() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::StartLF);
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_line_only_end_lf() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::EndLF);
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_line_start_and_end_lf() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::StartLF);
    lookset.set_insert(Look::EndLF);
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_haystack_and_line_combined() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::Start);
    lookset.set_insert(Look::EndLF);
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_empty() {
    let lookset = LookSet::empty();
    let result = lookset.contains_anchor();
}

#[test]
fn test_contains_anchor_full() {
    let mut lookset = LookSet::full();
    let result = lookset.contains_anchor();
}

