// Answer 0

#[test]
fn test_parse_hex_digits_invalid_digit() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "12g"; // 'g' is not a hex character
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('1')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_hex_digits(3);
}

#[test]
fn test_parse_hex_digits_invalid_digit_even() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "4A2B5C"; // valid hex up to '4', then '2' is okay, but '2' at i == 2 fails
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('A')), 
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_hex_digits(4);
}

#[test]
fn test_parse_hex_digits_invalid_digit_max() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\U0000000G"; // 'G' is not a hex character
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('0')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_hex_digits(8);
}

