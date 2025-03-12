// Answer 0

#[test]
fn test_parse_hex_brace_invalid_digit_g() {
    let flags = Flags::default();
    let config = Config::default();
    let parser = Parser {
        config,
        pattern: "{g}",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('g')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_hex_brace();
}

#[test]
fn test_parse_hex_brace_invalid_digit_exclamation() {
    let flags = Flags::default();
    let config = Config::default();
    let parser = Parser {
        config,
        pattern: "{!}",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('!')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_hex_brace();
}

#[test]
fn test_parse_hex_brace_invalid_digit_space() {
    let flags = Flags::default();
    let config = Config::default();
    let parser = Parser {
        config,
        pattern: "{ }",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_hex_brace();
}

#[test]
fn test_parse_hex_brace_invalid_digit_newline() {
    let flags = Flags::default();
    let config = Config::default();
    let parser = Parser {
        config,
        pattern: "{\n}",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\n')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_hex_brace();
}

