// Answer 0

#[test]
fn test_peek_space_ignore_whitespace_done_true() {
    let config = Config { nest_limit: 0, flags: Flags { ignore_whitespace: true, ..Default::default() }};
    let pattern = "  # a comment\n  ";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(pattern.len()), // Set position to the end of the pattern
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(config.flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.peek_space();
}

#[test]
fn test_peek_space_ignore_whitespace_done_true_empty_pattern() {
    let config = Config { nest_limit: 0, flags: Flags { ignore_whitespace: true, ..Default::default() }};
    let pattern = "";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(pattern.len()), // Set position to the end of the pattern
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(config.flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.peek_space();
}

#[test]
fn test_peek_space_ignore_whitespace_done_true_with_only_whitespace() {
    let config = Config { nest_limit: 0, flags: Flags { ignore_whitespace: true, ..Default::default() }};
    let pattern = "    "; // Only whitespace
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(pattern.len()), // Set position to the end of the pattern
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(config.flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.peek_space();
}

