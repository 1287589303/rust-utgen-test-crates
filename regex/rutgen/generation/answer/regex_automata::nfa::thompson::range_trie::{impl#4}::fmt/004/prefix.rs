// Answer 0

#[test]
fn test_fmt_valid_states_with_error_on_write() {
    let mut range_trie = RangeTrie {
        states: vec![State::Fail, State::Match { pattern_id: 0 }], // valid states
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };

    // Simulating a write error by creating a formatter that yields an error
    struct ErrorFormatter;
    
    impl fmt::Write for ErrorFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error) // Always return an error
        }
    }

    let mut formatter = ErrorFormatter;
    let _ = range_trie.fmt(&mut formatter);
}

#[test]
fn test_fmt_non_empty_states_with_error_on_write() {
    let mut range_trie = RangeTrie {
        states: vec![State::ByteRange { trans: (0, 0) }, State::Sparse { transitions: vec![] }], // valid states
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };

    struct ErrorFormatter;

    impl fmt::Write for ErrorFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error) // Always return an error
        }
    }
    
    let mut formatter = ErrorFormatter;
    let _ = range_trie.fmt(&mut formatter);
}

