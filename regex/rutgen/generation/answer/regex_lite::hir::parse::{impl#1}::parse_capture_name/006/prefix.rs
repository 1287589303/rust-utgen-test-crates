// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let pattern = "<valid_capture_name>";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('v')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_invalid_character() {
    let pattern = "<invalid@name>";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('i')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_duplicate() {
    let pattern = "<duplicate_name><duplicate_name>";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('d')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["duplicate_name".to_string()]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_empty_name() {
    let pattern = "<>";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('>')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_unclosed() {
    let pattern = "<valid_capture_name";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('v')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

