// Answer 0

#[test]
fn test_parse_hex_brace_valid() {
    let config = Config { size_limit: None };
    let pattern = r"\x{1A}";
    let depth = Cell::new(0);
    let pos = Cell::new(0);
    let char = Cell::new(Some('x'));
    let capture_index = Cell::new(0);
    let flags = RefCell::new(Flags::default());
    let capture_names = RefCell::new(vec![]);
    let parser = Parser {
        config,
        pattern,
        depth,
        pos,
        char,
        capture_index,
        flags,
        capture_names,
    };
    let _result = parser.parse_hex();
}

#[test]
fn test_parse_hex_digits_after_x() {
    let config = Config { size_limit: None };
    let pattern = r"\x1A";
    let depth = Cell::new(0);
    let pos = Cell::new(0);
    let char = Cell::new(Some('x'));
    let capture_index = Cell::new(0);
    let flags = RefCell::new(Flags::default());
    let capture_names = RefCell::new(vec![]);
    let parser = Parser {
        config,
        pattern,
        depth,
        pos,
        char,
        capture_index,
        flags,
        capture_names,
    };
    let _result = parser.parse_hex();
}

#[test]
fn test_parse_hex_brace_empty() {
    let config = Config { size_limit: None };
    let pattern = r"\x{}";
    let depth = Cell::new(0);
    let pos = Cell::new(0);
    let char = Cell::new(Some('x'));
    let capture_index = Cell::new(0);
    let flags = RefCell::new(Flags::default());
    let capture_names = RefCell::new(vec![]);
    let parser = Parser {
        config,
        pattern,
        depth,
        pos,
        char,
        capture_index,
        flags,
        capture_names,
    };
    let _result = parser.parse_hex();
}

