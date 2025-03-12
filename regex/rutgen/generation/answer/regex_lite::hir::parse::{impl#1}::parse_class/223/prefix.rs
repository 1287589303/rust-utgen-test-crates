// Answer 0

#[test]
fn test_parse_class_unclosed_after_negation() {
    let config = Config { size_limit: None };
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
    let _result = parser.parse_class();
}

#[test]
fn test_parse_class_unclosed_after_negation_with_spaces() {
    let config = Config { size_limit: None };
    let pattern = "[ ^";
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
    let _result = parser.parse_class();
}

#[test]
fn test_parse_class_unclosed_after_negation_with_other_characters() {
    let config = Config { size_limit: None };
    let pattern = "[^a";
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
    let _result = parser.parse_class();
}

