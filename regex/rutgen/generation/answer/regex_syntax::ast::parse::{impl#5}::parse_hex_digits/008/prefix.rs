// Answer 0

#[test]
fn test_parse_hex_digits_x() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Position { offset: 0, line: 1, column: 1 };
    let span_end = Position { offset: 2, line: 1, column: 3 };
    let hex_input = "1a";
    
    let parser = Parser {
        pos: Cell::new(position),
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
        scratch: RefCell::new(hex_input.to_string()),
    };
    
    let parser_i = ParserI { parser: &parser, pattern: hex_input };
    let _ = parser_i.parse_hex_digits(HexLiteralKind::X);
}

#[test]
fn test_parse_hex_digits_unicode_short() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Position { offset: 0, line: 1, column: 1 };
    let span_end = Position { offset: 4, line: 1, column: 5 };
    let hex_input = "abcd";

    let parser = Parser {
        pos: Cell::new(position),
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
        scratch: RefCell::new(hex_input.to_string()),
    };

    let parser_i = ParserI { parser: &parser, pattern: hex_input };
    let _ = parser_i.parse_hex_digits(HexLiteralKind::UnicodeShort);
}

#[test]
fn test_parse_hex_digits_unicode_long() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Position { offset: 0, line: 1, column: 1 };
    let span_end = Position { offset: 8, line: 1, column: 9 };
    let hex_input = "01234567";

    let parser = Parser {
        pos: Cell::new(position),
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
        scratch: RefCell::new(hex_input.to_string()),
    };

    let parser_i = ParserI { parser: &parser, pattern: hex_input };
    let _ = parser_i.parse_hex_digits(HexLiteralKind::UnicodeLong);
}

