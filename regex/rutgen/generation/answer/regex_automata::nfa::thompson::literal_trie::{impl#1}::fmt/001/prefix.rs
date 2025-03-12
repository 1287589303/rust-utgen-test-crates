// Answer 0

#[test]
fn test_fmt_empty_states() {
    let trie = LiteralTrie {
        states: Vec::new(),
        rev: false,
    };
    let mut buffer = Vec::new();
    let result = trie.fmt(&mut core::fmt::Formatter::new(&mut buffer));
}

#[test]
fn test_fmt_non_empty_states() {
    let state_id_1 = StateID(0);
    let state_1 = State {
        id: state_id_1,
        is_match: false,
        ntrans: 1,
        input_ranges: &[b'a'],
        next: &[1],
        pattern_ids: &[],
        accel: &[],
    };
    let state_id_2 = StateID(1);
    let state_2 = State {
        id: state_id_2,
        is_match: true,
        ntrans: 0,
        input_ranges: &[],
        next: &[],
        pattern_ids: &[1],
        accel: &[],
    };
    
    let trie = LiteralTrie {
        states: vec![state_1.clone(), state_2.clone()],
        rev: false,
    };
    let mut buffer = Vec::new();
    let result = trie.fmt(&mut core::fmt::Formatter::new(&mut buffer));
}

