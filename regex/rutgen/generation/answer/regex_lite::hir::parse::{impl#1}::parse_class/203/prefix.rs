// Answer 0

#[test]
fn test_parse_class_nested_class_error() {
    let config = Config { size_limit: None };
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
    
    let result = parser.parse_class();
}

#[test]
fn test_parse_class_with_negation() {
    let config = Config { size_limit: None };
    let pattern = "[^abc]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { case_insensitive: true, ..Flags::default() }),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class();
}

#[test]
fn test_parse_class_with_literal_dash() {
    let config = Config { size_limit: None };
    let pattern = "[-abc]";
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

