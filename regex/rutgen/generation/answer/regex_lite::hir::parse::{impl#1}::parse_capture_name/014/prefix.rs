// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<valid_name>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1), // Positioned at the first character after '<'
        char: Cell::new(Some('v')), // 'v' is a valid capture char
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_empty() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1), // Positioned at the first character after '<'
        char: Cell::new(Some('>')), // '>' means we should check for empty name
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_capture_name();
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_name_invalid_char() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<invalid_name$>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1), // Positioned at the first character after '<'
        char: Cell::new(Some('i')), // 'i' is valid
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_capture_name();
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_name_done() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<valid_name";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1), // Positioned at the first character after '<'
        char: Cell::new(Some('v')), // 'v' is valid
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_capture_name();
    assert!(result.is_err());
}

