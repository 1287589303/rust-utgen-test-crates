// Answer 0

#[test]
fn test_parse_octal_single_digit() {
    let pattern = "0abc";
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start),
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
    
    let parser_i = ParserI { parser: &parser, pattern };
    let _result = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_two_digits() {
    let pattern = "07abc";
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start),
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

    let parser_i = ParserI { parser: &parser, pattern };
    let _result = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_three_digits() {
    let pattern = "077abc";
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start),
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

    let parser_i = ParserI { parser: &parser, pattern };
    let _result = parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_invalid() {
    let pattern = "08abc"; // invalid octal digit
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start),
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

    let parser_i = ParserI { parser: &parser, pattern };
    let _result = parser_i.parse_octal();
}

