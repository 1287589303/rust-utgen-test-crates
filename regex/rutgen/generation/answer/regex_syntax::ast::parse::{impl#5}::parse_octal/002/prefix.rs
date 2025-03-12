// Answer 0

#[test]
fn test_parse_octal_valid_three_digit() {
    let pattern = "0777";
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 3, line: 1, column: 4 };
    let span = Span::new(pos_start, pos_end);

    let parser = Parser {
        pos: Cell::new(pos_start),
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

    let parser_i = ParserI { parser, pattern };

    let result = parser_i.parse_octal();
} 

#[test]
fn test_parse_octal_valid_two_digit() {
    let pattern = "075";
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 3, line: 1, column: 4 };
    let span = Span::new(pos_start, pos_end);

    let parser = Parser {
        pos: Cell::new(pos_start),
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

    let parser_i = ParserI { parser, pattern };

    let result = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_valid_one_digit() {
    let pattern = "0";
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos_start, pos_end);

    let parser = Parser {
        pos: Cell::new(pos_start),
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

    let parser_i = ParserI { parser, pattern };

    let result = parser_i.parse_octal();
} 

#[test]
#[should_panic]
fn test_parse_octal_invalid() {
    let pattern = "8";
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos_start),
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

    let parser_i = ParserI { parser, pattern };

    let _result = parser_i.parse_octal();
}

