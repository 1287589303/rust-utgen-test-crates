// Answer 0

#[test]
fn test_empty_range_trie_debug_output() {
    use core::fmt::Write;
    use regex_automata::RangeTrie;

    let mut output = String::new();
    let empty_trie = RangeTrie {
        states: vec![],
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };

    let _ = empty_trie.fmt(&mut output);
}

#[test]
fn test_range_trie_with_final_state_only() {
    use core::fmt::Write;
    use regex_automata::{RangeTrie, State};

    let mut output = String::new();
    let trie_with_final_state = RangeTrie {
        states: vec![State::Match { pattern_id: 0 }],
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };

    let _ = trie_with_final_state.fmt(&mut output);
}

