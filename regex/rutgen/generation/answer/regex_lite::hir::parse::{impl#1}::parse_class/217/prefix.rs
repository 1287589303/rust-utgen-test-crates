// Answer 0

#[test]
fn test_parse_class_negated_with_cases() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: true,
            ..Flags::default()
        },
    };

    let pattern = "[^a-z]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class();
    // Ignoring assertions as per guidelines, only generating the input and function call
}

#[test]
fn test_parse_class_with_multiple_negated_ranges() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: true,
            ..Flags::default()
        },
    };

    let pattern = "[^a-zA-Z0-9]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class();
}

#[test]
fn test_parse_class_with_repeated_negations() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: true,
            ..Flags::default()
        },
    };

    let pattern = "[^^a-z]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class();
}

#[test]
fn test_parse_class_with_single_valid_character() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: true,
            ..Flags::default()
        },
    };

    let pattern = "[^x]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class();
}

#[test]
fn test_parse_class_with_multiple_literally_negated_items() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: true,
            ..Flags::default()
        },
    };

    let pattern = "[^!@#]";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class();
}

