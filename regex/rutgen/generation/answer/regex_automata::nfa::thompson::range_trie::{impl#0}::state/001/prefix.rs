// Answer 0

#[test]
fn test_state_valid_index_0() {
    let mut trie = RangeTrie::new();
    let state_id = StateID::new_unchecked(0);
    let _state = trie.state(state_id);
}

#[test]
fn test_state_valid_index_1() {
    let mut trie = RangeTrie::new();
    let state_id = StateID::new_unchecked(1);
    let _state = trie.state(state_id);
}

#[test]
fn test_state_valid_index_boundary() {
    let mut trie = RangeTrie::new();
    let state_id = StateID::new_unchecked(trie.states.len() - 1);
    let _state = trie.state(state_id);
}

#[test]
fn test_state_invalid_index() {
    let mut trie = RangeTrie::new();
    let state_id = StateID::new_unchecked(trie.states.len());
    let _state = trie.state(state_id);
}

