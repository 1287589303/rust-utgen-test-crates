// Answer 0

#[test]
fn test_parse_escape_with_a() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "a";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_meta_character() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "!";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('!')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.parse_escape();
}

