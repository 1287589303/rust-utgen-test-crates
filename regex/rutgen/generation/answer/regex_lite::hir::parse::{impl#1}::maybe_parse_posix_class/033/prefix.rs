// Answer 0

#[test]
fn test_maybe_parse_posix_class_no_class_name() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "[::]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(Vec::new()),
    };

    let result = parser.maybe_parse_posix_class();
    assert_eq!(result, None);
}

#[test]
fn test_maybe_parse_posix_class_negated() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "[[:alnum:]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(Vec::new()),
    };

    let result = parser.maybe_parse_posix_class();
    assert_eq!(result, None);
}

#[test]
fn test_maybe_parse_posix_class_double_colon() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "[[:lower:]abc]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(Vec::new()),
    };

    let result = parser.maybe_parse_posix_class();
    assert_eq!(result, None);
}

#[test]
fn test_maybe_parse_posix_class_incomplete() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "[[:lower:]]xy";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(Vec::new()),
    };

    let result = parser.maybe_parse_posix_class();
    assert_eq!(result, None);
}

