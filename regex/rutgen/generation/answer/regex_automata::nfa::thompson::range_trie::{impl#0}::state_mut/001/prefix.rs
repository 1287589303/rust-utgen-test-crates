// Answer 0

#[test]
fn test_state_mut_valid_index_0() {
    let mut trie = RangeTrie::new();
    trie.states.push(State { transitions: vec![] });
    let state_id = StateID::new_unchecked(0);
    let _state = trie.state_mut(state_id);
}

#[test]
fn test_state_mut_valid_index_max() {
    let mut trie = RangeTrie::new();
    trie.states.push(State { transitions: vec![] });
    trie.states.push(State { transitions: vec![] });
    let state_id = StateID::new_unchecked(1);
    let _state = trie.state_mut(state_id);
}

#[should_panic]
#[test]
fn test_state_mut_invalid_index() {
    let mut trie = RangeTrie::new();
    trie.states.push(State { transitions: vec![] });
    let state_id = StateID::new_unchecked(2);
    let _state = trie.state_mut(state_id);
}

