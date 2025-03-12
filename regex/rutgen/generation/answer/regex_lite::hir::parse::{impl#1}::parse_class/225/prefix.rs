// Answer 0

#[test]
fn test_parse_class_valid_characters() {
    let config = Config { size_limit: None, };
    let pattern = "[a-zA-Z]";
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
fn test_parse_class_negation() {
    let config = Config { size_limit: None, };
    let pattern = "[^a-z]";
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
fn test_parse_class_empty_class() {
    let config = Config { size_limit: None, };
    let pattern = "[]]";
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
fn test_parse_class_double_negation() {
    let config = Config { size_limit: None, };
    let pattern = "[^^a-z]";
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
fn test_parse_class_invalid_character() {
    let config = Config { size_limit: None, };
    let pattern = "[a-zA-Z&&[^]]";
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
fn test_parse_class_multiple_consecutive_dashes() {
    let config = Config { size_limit: None, };
    let pattern = "[a--z]";
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
fn test_parse_class_unmatched_closing_bracket() {
    let config = Config { size_limit: None, };
    let pattern = "[a-b-c]";
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

