// Answer 0

#[test]
fn test_parse_class_unclosed_after_closing() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[--]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_class();
}

#[test]
fn test_parse_class_unclosed_after_closing_empty() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[--]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('-')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_class();
}

#[test]
fn test_parse_class_unclosed_after_closing_single_dash() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[-]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('-')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_class();
}

