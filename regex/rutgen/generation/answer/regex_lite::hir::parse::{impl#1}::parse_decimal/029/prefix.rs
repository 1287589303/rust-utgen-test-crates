// Answer 0

#[test]
fn test_parse_decimal_valid_input() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "   12345   "; // Whitespace around the digits
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')), // Initially whitespace
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.pos.set(pattern.len()); // Set pos to the end of the pattern
    let _result = parser.parse_decimal();
}

#[test]
fn test_parse_decimal_with_trailing_whitespace() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "   98765"; // Trailing empty space, no leading
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')), // Initially whitespace
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.pos.set(pattern.len()); // Set pos to the end of the pattern
    let _result = parser.parse_decimal();
}

#[test]
fn test_parse_decimal_valid_large_number() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "   4294967295   "; // The maximum u32 value
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')), // Initially whitespace
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.pos.set(pattern.len()); // Set pos to the end of the pattern
    let _result = parser.parse_decimal();
}

#[test]
fn test_parse_decimal_leading_whitespace() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "   42"; // Leading spaces, no trailing ones
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')), // Initially whitespace
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.pos.set(pattern.len()); // Set pos to the end of the pattern
    let _result = parser.parse_decimal();
}

