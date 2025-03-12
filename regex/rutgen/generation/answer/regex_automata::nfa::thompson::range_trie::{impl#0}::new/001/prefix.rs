// Answer 0

#[test]
fn test_new_range_trie() {
    let trie: RangeTrie = RangeTrie::new();
}

#[test]
fn test_new_range_trie_empty() {
    let trie: RangeTrie = RangeTrie::new();
    assert!(trie.states.is_empty());
    assert!(trie.free.is_empty());
    assert!(trie.iter_stack.borrow().is_empty());
    assert!(trie.iter_ranges.borrow().is_empty());
    assert!(trie.dupe_stack.is_empty());
    assert!(trie.insert_stack.is_empty());
}

