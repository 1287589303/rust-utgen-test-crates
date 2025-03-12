// Answer 0

#[test]
fn test_parse_escape_with_digit() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\d";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('d')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_d() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\D";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('D')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_whitespace() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\s";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('s')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_whitespace() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\S";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('S')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_word() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\w";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('w')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_word() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\W";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('W')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_hex_x() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\x";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('x')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_hex_u() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\u";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('u')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_hex_U() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\U";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('U')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_perl_class_d() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\d";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('d')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_perl_class_s() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\s";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('s')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

