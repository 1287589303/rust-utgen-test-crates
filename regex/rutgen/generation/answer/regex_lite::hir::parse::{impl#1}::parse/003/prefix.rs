// Answer 0

#[test]
fn test_parse_valid_pattern_1() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern: "(abc|def)",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse();
}

#[test]
fn test_parse_valid_pattern_2() {
    let config = Config { nest_limit: 5, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern: "(abc(d|e)f)*",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse();
}

#[test]
fn test_parse_valid_pattern_with_flags() {
    let mut flags = Flags::default();
    flags.case_insensitive = true;
    let config = Config { nest_limit: 10, flags };
    let parser = Parser {
        config,
        pattern: "(?i)(abc|def)",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse();
}

#[test]
fn test_parse_valid_nested_pattern() {
    let config = Config { nest_limit: 6, flags: Flags::default() };
    let parser = Parser {
        config,
        pattern: "((a|b)|(c|d))e",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse();
}

