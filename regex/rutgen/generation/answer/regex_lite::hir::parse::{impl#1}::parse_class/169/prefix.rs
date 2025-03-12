// Answer 0

#[test]
fn test_parse_class_with_multiple_dashes() {
    let config = Config { size_limit: Some(10) };
    let pattern = "[---]";
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
fn test_parse_class_with_negation_and_literal() {
    let config = Config { size_limit: Some(10) };
    let pattern = "[^x&]";
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
fn test_parse_class_with_posix_class() {
    let config = Config { size_limit: Some(10) };
    let pattern = "[[:digit:]]";
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
fn test_parse_class_with_nested_class_error() {
    let config = Config { size_limit: Some(10) };
    let pattern = "[[abc]]";
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
fn test_parse_class_with_unexpected_character() {
    let config = Config { size_limit: Some(10) };
    let pattern = "[--~]";
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

