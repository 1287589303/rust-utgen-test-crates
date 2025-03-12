// Answer 0

#[test]
fn test_parse_hex_brace_empty_string() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern: "{}",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_hex_brace();
}

#[test]
fn test_parse_hex_brace_invalid_character() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern: "{g}",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_hex_brace();
}

#[test]
fn test_parse_hex_brace_valid_input() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern: "{1F}",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_hex_brace();
}

