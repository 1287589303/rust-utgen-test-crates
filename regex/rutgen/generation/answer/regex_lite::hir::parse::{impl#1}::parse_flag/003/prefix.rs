// Answer 0

#[test]
fn test_parse_flag_case_insensitive_enabled() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config { 
            nest_limit: 10, 
            flags: flags.clone() 
        },
        pattern: "x", 
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('x')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_flag(&mut flags, false);
}

#[test]
fn test_parse_flag_case_insensitive_disabled() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config { 
            nest_limit: 10, 
            flags: flags.clone() 
        },
        pattern: "x", 
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('x')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_flag(&mut flags, true);
    // Check flags after negation
}

#[test]
fn test_parse_flag_no_op_enabled() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config { 
            nest_limit: 10, 
            flags: flags.clone() 
        },
        pattern: "x", 
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('x')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_flag(&mut flags, false);
}

#[test]
fn test_parse_flag_no_op_disabled() {
    let mut flags = Flags::default();
    let parser = Parser {
        config: Config { 
            nest_limit: 10, 
            flags: flags.clone() 
        },
        pattern: "x", 
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('x')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags.clone()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_flag(&mut flags, true);
}

