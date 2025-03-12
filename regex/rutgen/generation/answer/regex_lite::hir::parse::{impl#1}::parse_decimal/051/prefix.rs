// Answer 0

#[test]
fn test_parse_decimal_no_digits() {
    let config = Config { nest_limit: 5, flags: Flags::default() };
    let pattern = "   0abc   ";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_decimal();
}

#[test]
fn test_parse_decimal_invalid_string() {
    let config = Config { nest_limit: 5, flags: Flags::default() };
    let pattern = "   0xyz   ";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_decimal();
}

