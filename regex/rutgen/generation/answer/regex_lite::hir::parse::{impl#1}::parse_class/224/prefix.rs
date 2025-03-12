// Answer 0

#[test]
fn test_parse_class_unclosed() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[";
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
    let _ = parser.parse_class();
}

#[test]
fn test_parse_class_unclosed_after_negation() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[^";
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
    let _ = parser.parse_class();
}

#[test]
fn test_parse_class_done_without_closing() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[abc";
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
    let _ = parser.parse_class();
}

#[test]
fn test_parse_class_empty() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[]";
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
    let _ = parser.parse_class();
}

#[test]
fn test_parse_class_nested() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "[[a-z]";
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
    let _ = parser.parse_class();
}

