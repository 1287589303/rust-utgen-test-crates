// Answer 0

#[test]
fn test_parse_primitive_backslash() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern: "\\",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\\')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_digit() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern: "\\0",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\\')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_unicode() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern: "\\u",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\\')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_named() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern: "\\p",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\\')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_posix() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern: "\\d",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\\')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

