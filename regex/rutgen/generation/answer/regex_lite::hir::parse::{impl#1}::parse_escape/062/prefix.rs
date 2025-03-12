// Answer 0

#[test]
fn test_parse_escape_with_unicode_class() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "p";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('p')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_unicode_class() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "P";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('P')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

