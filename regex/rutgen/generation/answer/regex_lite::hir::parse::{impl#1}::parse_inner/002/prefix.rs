// Answer 0

#[test]
fn test_parse_inner_valid_alternation() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "(abc|def)",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_valid_counted_repetition() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "a{1,3}",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_valid_character_class() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "[a-z]",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_valid_capture_group() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "(?P<name>subpattern)",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_valid_group_with_flags() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "(?i:abc)",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { case_insensitive: true, ..Flags::default() }),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

