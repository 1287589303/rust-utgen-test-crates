// Answer 0

#[test]
#[should_panic]
fn test_add_empty_too_many_states() {
    let mut range_trie = RangeTrie::new();
    
    // Set self.states to exceed the maximum allowable states.
    range_trie.states = vec![State::default(); StateID::MAX as usize + 1];

    range_trie.add_empty();
}

