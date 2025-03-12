// Answer 0

#[test]
fn test_parse_group_empty_flags() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "(?)";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')), // starting character is '('
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()), // initial flags
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_group();
}

#[test]
fn test_parse_group_non_empty() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "(?i)";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')), // starting character is '('
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()), // initial flags
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_group();
}

