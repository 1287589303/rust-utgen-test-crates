// Answer 0

#[test]
fn test_literal_trie_empty_states() {
    let trie = LiteralTrie {
        states: Vec::new(),
        rev: false,
    };
    let mut output = vec![];
    let result = trie.fmt(&mut output);
}

#[test]
fn test_literal_trie_empty_states_rev() {
    let trie = LiteralTrie {
        states: Vec::new(),
        rev: true,
    };
    let mut output = vec![];
    let result = trie.fmt(&mut output);
}

