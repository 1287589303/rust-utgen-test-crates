// Answer 0

#[test]
fn test_add_transition_at_valid_insertion() {
    let mut trie = RangeTrie::new();
    let from_id = ROOT;
    let next_id = FINAL;
    let range = Utf8Range::new(0, 10); // valid range
    
    trie.add_transition(0, range.clone(), next_id);
    trie.add_transition_at(0, from_id, range.clone(), next_id); // insert at index 0
}

#[test]
fn test_add_transition_at_boundary_insertion_first() {
    let mut trie = RangeTrie::new();
    let from_id = ROOT;
    let next_id = FINAL;
    let range1 = Utf8Range::new(0, 10);
    let range2 = Utf8Range::new(10, 20);

    trie.add_transition(0, range1.clone(), next_id);
    trie.add_transition_at(0, from_id, range2.clone(), next_id); // insert at index 0 when the count is 1
}

#[test]
fn test_add_transition_at_boundary_insertion_last() {
    let mut trie = RangeTrie::new();
    let from_id = ROOT;
    let next_id = FINAL;
    let range1 = Utf8Range::new(0, 10);
    let range2 = Utf8Range::new(10, 20);

    trie.add_transition(0, range1.clone(), next_id);
    trie.add_transition_at(1, from_id, range2.clone(), next_id); // attempt to insert at index 1 (valid insertion)
}

#[test]
fn test_add_transition_at_empty_transition_list() {
    let mut trie = RangeTrie::new();
    let from_id = ROOT;
    let next_id = FINAL;
    let range = Utf8Range::new(0, 10); // valid range

    trie.add_transition_at(0, from_id, range.clone(), next_id); // inserting into an empty list
}

#[test]
fn test_add_transition_at_overboundary_index() {
    let mut trie = RangeTrie::new();
    let from_id = ROOT;
    let next_id = FINAL;
    let range = Utf8Range::new(0, 10); // valid range
    
    trie.add_transition(0, range.clone(), next_id);
    trie.add_transition_at(10, from_id, range.clone(), next_id); // inserting at index greater than current transitions count (should not panic)
}

