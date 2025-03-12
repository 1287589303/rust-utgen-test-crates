// Answer 0

#[test]
fn test_parse_flag_enable_multi_line() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config {
            nest_limit: 10,
            flags: Flags::default(),
        },
        pattern: "some_pattern",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('m')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_flag(&mut flags, false);
}

#[test]
fn test_parse_flag_disable_multi_line() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config {
            nest_limit: 10,
            flags: Flags::default(),
        },
        pattern: "some_pattern",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('m')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_flag(&mut flags, true);
}

