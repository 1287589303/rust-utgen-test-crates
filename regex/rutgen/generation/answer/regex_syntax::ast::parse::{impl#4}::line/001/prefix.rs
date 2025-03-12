// Answer 0

#[test]
fn test_line_first_line() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 64,
        octal: true,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "test pattern");
    parser_i.line();
}

#[test]
fn test_line_middle_line() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 5, line: 2, column: 6 }),
        capture_index: Cell::new(0),
        nest_limit: 64,
        octal: true,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "test pattern\nnext line");
    parser_i.line();
}

#[test]
fn test_line_last_line() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 14, line: 3, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 64,
        octal: true,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "first line\nsecond line\nthird line");
    parser_i.line();
}

