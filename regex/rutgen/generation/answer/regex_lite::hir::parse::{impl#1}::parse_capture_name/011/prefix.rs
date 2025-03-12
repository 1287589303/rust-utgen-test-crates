// Answer 0

#[test]
fn test_parse_capture_name_valid_case() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<capture>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('c')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_invalid_chars() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<ca#pture>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('c')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_empty_name() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<>";
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
#[should_panic]
fn test_parse_capture_name_no_name_end() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<capture";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('c')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_valid_with_special() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "<capture_1>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('c')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

