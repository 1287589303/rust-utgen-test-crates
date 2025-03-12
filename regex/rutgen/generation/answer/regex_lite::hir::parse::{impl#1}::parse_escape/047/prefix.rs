// Answer 0

#[test]
fn test_parse_escape_with_char_r() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "\\r";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('r')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_u() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_x() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_D() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_d() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_P() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "\\P";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('P')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_s() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_S() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_w() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_with_char_W() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
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

    parser.parse_escape().unwrap();
}

