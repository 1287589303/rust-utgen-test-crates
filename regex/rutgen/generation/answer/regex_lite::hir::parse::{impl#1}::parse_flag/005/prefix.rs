// Answer 0

#[test]
fn test_parse_flag_enabled_U() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "test",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('U')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_flag(&mut flags, false);
}

#[test]
fn test_parse_flag_disabled_U() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "test",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('U')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_flag(&mut flags, true);
}

