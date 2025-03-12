// Answer 0

#[test]
fn test_parse_hex_digits_valid_2_digits() {
    let pattern = "\\x61"; // valid hex for 'a'
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    };
    let parser_i = ParserI { parser: &parser, pattern };
    let result = parser_i.parse_hex_digits(HexLiteralKind::X);
}

#[test]
fn test_parse_hex_digits_valid_4_digits() {
    let pattern = "\\u0061"; // valid hex for 'a'
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    };
    let parser_i = ParserI { parser: &parser, pattern };
    let result = parser_i.parse_hex_digits(HexLiteralKind::UnicodeShort);
}

#[test]
fn test_parse_hex_digits_invalid_not_unicode() {
    let pattern = "\\u00zz"; // invalid hex characters
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    };
    let parser_i = ParserI { parser: &parser, pattern };
    let result = parser_i.parse_hex_digits(HexLiteralKind::UnicodeShort);
}

#[test]
fn test_parse_hex_digits_invalid_escaped_char() {
    let pattern = "\\u006g"; // contains an invalid hex digit 'g'
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_pos),
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
    };
    let parser_i = ParserI { parser: &parser, pattern };
    let result = parser_i.parse_hex_digits(HexLiteralKind::UnicodeShort);
}

