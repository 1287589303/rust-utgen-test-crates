// Answer 0

#[test]
fn test_root_empty_states() {
    let mut trie = PreferenceTrie {
        states: Vec::new(),
        matches: Vec::new(),
        next_literal_index: 1,
    };
    let root_state_id = trie.root();
}

#[test]
fn test_root_create_state() {
    let mut trie = PreferenceTrie {
        states: Vec::new(),
        matches: Vec::new(),
        next_literal_index: 1,
    };
    let root_state_id = trie.root();
    let state_created = trie.states.len() == 1;
}

