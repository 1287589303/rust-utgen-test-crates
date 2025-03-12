// Answer 0

#[test]
fn test_clear_with_populated_range_trie() {
    let mut range_trie = RangeTrie::new();
    range_trie.insert(&[Utf8Range::from(0..=5), Utf8Range::from(10..=15)]);
    range_trie.clear();
}

#[test]
fn test_clear_on_already_cleared_range_trie() {
    let mut range_trie = RangeTrie::new();
    range_trie.insert(&[Utf8Range::from(0..=5)]);
    range_trie.clear();
    range_trie.clear();
}

#[test]
fn test_clear_with_multiple_state_types() {
    let mut range_trie = RangeTrie::new();
    range_trie.states.push(State::ByteRange { trans: Transition::new(0..=5) });
    range_trie.states.push(State::Sparse { transitions: vec![] });
    range_trie.clear();
}

