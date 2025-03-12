// Answer 0

#[test]
fn test_parse_escape_perl_class_digit() {
    let pattern = r"\d";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 0,
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
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_space() {
    let pattern = r"\s";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 0,
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
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_word() {
    let pattern = r"\w";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 0,
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
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_not_word() {
    let pattern = r"\D";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 0,
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
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_space_negated() {
    let pattern = r"\S";
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 0,
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
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

