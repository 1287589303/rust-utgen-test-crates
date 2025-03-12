// Answer 0

#[test]
fn test_parse_group_with_empty_flags() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "(?):";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.parse_group();
}

#[test]
fn test_parse_group_with_flags_and_no_inner() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "(?i):";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::from_bits_truncate(1)),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.parse_group();
}

