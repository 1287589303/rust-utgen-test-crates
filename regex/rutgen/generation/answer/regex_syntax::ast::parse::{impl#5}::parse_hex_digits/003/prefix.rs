// Answer 0

#[test]
fn test_parse_hex_digits_with_invalid_char_after_valid_char() {
    let pattern = "\\x1g"; 
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 3, line: 1, column: 4 };
    
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 100,
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

    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    parser_i.parse_hex_digits(HexLiteralKind::X);
}

#[test]
fn test_parse_hex_digits_with_invalid_char_after_valid_char_unicode_short() {
    let pattern = "\\u1g2"; 
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 4, line: 1, column: 5 };
    
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 100,
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

    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    parser_i.parse_hex_digits(HexLiteralKind::UnicodeShort);
}

#[test]
fn test_parse_hex_digits_with_invalid_char_after_valid_char_unicode_long() {
    let pattern = "\\U1g2"; 
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = position;
    let span_end = Position { offset: 4, line: 1, column: 5 };
    
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 100,
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

    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    parser_i.parse_hex_digits(HexLiteralKind::UnicodeLong);
}

