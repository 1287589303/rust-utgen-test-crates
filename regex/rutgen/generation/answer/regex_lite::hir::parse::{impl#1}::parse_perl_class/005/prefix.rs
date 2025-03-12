// Answer 0

#[test]
fn test_parse_perl_class_lowercase_w() {
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
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_uppercase_W() {
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
    parser.parse_perl_class();
}

