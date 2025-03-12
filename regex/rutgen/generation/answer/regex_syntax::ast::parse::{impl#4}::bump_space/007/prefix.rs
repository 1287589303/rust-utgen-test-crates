// Answer 0

#[test]
fn test_bump_space_no_ignore_whitespace_start() {
    let pattern = "    # This is a comment\nabc";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, pattern);
    parser_i.bump_space();
}

#[test]
fn test_bump_space_no_ignore_whitespace_within() {
    let pattern = "abc   # Comment inside\n    def";
    let pos = Position { offset: 3, line: 1, column: 4 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, pattern);
    parser_i.bump_space();
}

#[test]
fn test_bump_space_no_ignore_whitespace_end() {
    let pattern = "abc # Last comment\n";
    let pos = Position { offset: 10, line: 1, column: 10 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, pattern);
    parser_i.bump_space();
}

#[test]
fn test_bump_space_no_ignore_whitespace_multiple_spaces() {
    let pattern = "   # Comment\n   \n   abc";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, pattern);
    parser_i.bump_space();
}

