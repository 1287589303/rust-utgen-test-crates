// Answer 0

#[test]
fn test_fmt_with_empty_state() {
    let state = State::Empty { next: ROOT }; 
    let range_trie = RangeTrie {
        states: vec![state.clone(), state.clone(), state],
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let _ = range_trie.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_fail_state() {
    let state = State::Fail; 
    let range_trie = RangeTrie {
        states: vec![state.clone(), state, state.clone()],
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let _ = range_trie.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_match_state() {
    let state = State::Match { pattern_id: PatternID::new_unchecked(1) }; 
    let range_trie = RangeTrie {
        states: vec![state.clone(), state.clone(), state],
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let _ = range_trie.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_binary_union_state() {
    let state = State::BinaryUnion { alt1: ROOT, alt2: ROOT };
    let range_trie = RangeTrie {
        states: vec![state.clone(), state.clone(), state],
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let _ = range_trie.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_union_state() {
    let state = State::Union { alternates: vec![ROOT] }; 
    let range_trie = RangeTrie {
        states: vec![state.clone(), state.clone(), state],
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };
    let _ = range_trie.fmt(&mut fmt::Formatter::new());
}

