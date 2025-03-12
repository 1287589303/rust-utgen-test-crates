// Answer 0

#[test]
fn test_parse_hex_digits_2_digits_unexpected_eof() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "\\xNN",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('N')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_hex_digits(2);
}

#[test]
fn test_parse_hex_digits_4_digits_unexpected_eof() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "\\uNNNN",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('N')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_hex_digits(4);
}

#[test]
fn test_parse_hex_digits_8_digits_unexpected_eof() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "\\UNNNNNNNN",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('N')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_hex_digits(8);
}

