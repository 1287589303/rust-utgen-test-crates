// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let pattern = "<valid_name>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    let _ = parser.parse_capture_name(0);
}

#[test]
fn test_parse_capture_name_invalid_start() {
    let pattern = "<123name>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(1),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    let _ = parser.parse_capture_name(1);
}

#[test]
fn test_parse_capture_name_empty() {
    let pattern = "<>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(2),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    let _ = parser.parse_capture_name(2);
}

#[test]
fn test_parse_capture_name_invalid_characters() {
    let pattern = "<invalid-name>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(3),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    let _ = parser.parse_capture_name(3);
}

