// Answer 0

#[test]
fn test_parse_class_err_difference_unsupported() {
    let pattern = "[--]";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('-')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_class();
}

#[test]
fn test_parse_class_err_difference_unsupported_additional_case() {
    let pattern = "[--a-z]";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('-')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_class();
}

