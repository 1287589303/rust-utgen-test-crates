// Answer 0

#[test]
fn test_fmt_correct_state_id() {
    let state_id = StateID(0.into());
    let state = State {
        id: state_id,
        is_match: false,
        ntrans: 0,
        input_ranges: &[],
        next: &[],
        pattern_ids: &[],
        accel: &[],
    };
    
    let trie = LiteralTrie {
        states: vec![state],
        rev: false,
    };
    
    let mut buffer = vec![];
    let mut formatter = core::fmt::Formatter::new(&mut buffer);
    
    let _ = trie.fmt(&mut formatter);
}

#[test]
fn test_fmt_non_empty_states() {
    let state_id_1 = StateID(1.into());
    let state_1 = State {
        id: state_id_1,
        is_match: false,
        ntrans: 1,
        input_ranges: &[],
        next: &[],
        pattern_ids: &[],
        accel: &[],
    };
    
    let state_id_2 = StateID(2.into());
    let state_2 = State {
        id: state_id_2,
        is_match: true,
        ntrans: 1,
        input_ranges: &[],
        next: &[],
        pattern_ids: &[],
        accel: &[],
    };

    let trie = LiteralTrie {
        states: vec![state_1, state_2],
        rev: false,
    };
    
    let mut buffer = vec![];
    let mut formatter = core::fmt::Formatter::new(&mut buffer);
    
    let _ = trie.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_error_in_sid_format() {
    let state_id = StateID(-1.into()); // Invalid StateID to trigger error
    let state = State {
        id: state_id,
        is_match: false,
        ntrans: 0,
        input_ranges: &[],
        next: &[],
        pattern_ids: &[],
        accel: &[],
    };

    let trie = LiteralTrie {
        states: vec![state],
        rev: false,
    };

    let mut buffer = vec![];
    let mut formatter = core::fmt::Formatter::new(&mut buffer);

    let _ = trie.fmt(&mut formatter);
}

