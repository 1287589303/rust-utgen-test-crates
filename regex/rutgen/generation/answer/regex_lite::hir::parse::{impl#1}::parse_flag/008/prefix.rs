// Answer 0

#[test]
fn test_parse_flag_case_insensitive() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config {
            size_limit: None,
            nest_limit: 10,
            flags: flags.clone(),
        },
        pattern: "test pattern",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('i')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.parse_flag(&mut flags, false);
}

#[test]
fn test_parse_flag_case_insensitive_altered_flags() {
    let mut flags = Flags {
        case_insensitive: true,
        ..Flags::default()
    };
    let parser = Parser {
        config: Config {
            size_limit: None,
            nest_limit: 10,
            flags: flags.clone(),
        },
        pattern: "test pattern",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('i')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.parse_flag(&mut flags, false);
}

