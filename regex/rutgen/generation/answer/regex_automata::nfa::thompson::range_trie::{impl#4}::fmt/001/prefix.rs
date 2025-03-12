// Answer 0

#[test]
fn test_fmt_with_empty_states() {
    let states: Vec<State> = vec![];
    let range_trie = RangeTrie {
        states,
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let mut buffer = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let _ = range_trie.fmt(formatter);
}

#[test]
fn test_fmt_with_one_state() {
    let one_state = State::Match { pattern_id: 0 };
    let states: Vec<State> = vec![one_state];
    let range_trie = RangeTrie {
        states,
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let mut buffer = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let _ = range_trie.fmt(formatter);
}

#[test]
fn test_fmt_with_multiple_states() {
    let states: Vec<State> = vec![
        State::ByteRange { trans: Transition::default() },
        State::Match { pattern_id: 0 },
        State::Fail,
        State::Empty { next: StateID::new_unchecked(1) },
    ];
    let range_trie = RangeTrie {
        states,
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let mut buffer = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let _ = range_trie.fmt(formatter);
}

#[test]
fn test_fmt_with_states_beyond_final() {
    let states: Vec<State> = vec![
        State::ByteRange { trans: Transition::default() },
        State::ByteRange { trans: Transition::default() },
        State::Match { pattern_id: 1 },
    ];
    let range_trie = RangeTrie {
        states,
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let mut buffer = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let _ = range_trie.fmt(formatter);
}

