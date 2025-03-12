// Answer 0

#[test]
fn test_parse_class_with_multiple_hyphens() {
    let config = Config {
        size_limit: None,
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

    let _result = parser.parse_class();
}

#[test]
fn test_parse_class_with_hyphen_and_characters() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };

    let pattern = "[a-z-]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let _result = parser.parse_class();
}

#[test]
fn test_parse_class_empty_with_hyphens() {
    let config = Config {
        size_limit: None,
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

    let _result = parser.parse_class();
}

#[test]
fn test_parse_class_hyphen_light() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };

    let pattern = "[-a-]";
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

    let _result = parser.parse_class();
}

