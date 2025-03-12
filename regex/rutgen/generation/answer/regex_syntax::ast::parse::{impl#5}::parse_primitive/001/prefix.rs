// Answer 0

#[test]
fn test_parse_primitive_dot() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: ".",
    };
    let _result = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_assertion_end_line() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(1),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "$",
    };
    let _result = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(2),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "\\",
    };
    let _result = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_assertion_start_line() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(3),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "^",
    };
    let _result = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_literal() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(4),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "a",
    };
    let _result = parser.parse_primitive();
}

