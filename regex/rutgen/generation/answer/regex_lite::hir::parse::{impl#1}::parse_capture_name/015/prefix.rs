// Answer 0

#[test]
fn test_valid_capture_name_1() {
    let pattern = "<valid_name>";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('v')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["valid_name".to_string()]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_valid_capture_name_2() {
    let pattern = "<_validName123>";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('_')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["validName123".to_string()]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_valid_capture_name_3() {
    let pattern = "<_alpha_123>";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('_')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["alpha_123".to_string()]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_valid_capture_name_4() {
    let pattern = "<name123>";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('n')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["name123".to_string()]),
    };
    let _ = parser.parse_capture_name();
}

