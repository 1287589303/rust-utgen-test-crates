// Answer 0

#[test]
fn test_literal_trie_with_states() {
    let state_id_1 = StateID(0);
    let state_id_2 = StateID(1);
    let state_1 = State {
        id: state_id_1,
        is_match: true,
        ntrans: 1,
        input_ranges: &[b'a'],
        next: &[1],
        pattern_ids: &[0],
        accel: &[],
    };
    let state_2 = State {
        id: state_id_2,
        is_match: false,
        ntrans: 1,
        input_ranges: &[b'b'],
        next: &[0],
        pattern_ids: &[],
        accel: &[],
    };
    let trie = LiteralTrie {
        states: vec![state_1.clone(), state_2.clone()],
        rev: false,
    };
    let _ = format!("{:?}", trie); // This will invoke fmt and format the output.
}

#[test]
fn test_literal_trie_empty_states() {
    let trie = LiteralTrie {
        states: Vec::new(),
        rev: false,
    };
    let _ = format!("{:?}", trie); // This will invoke fmt and handle empty state case.
}

