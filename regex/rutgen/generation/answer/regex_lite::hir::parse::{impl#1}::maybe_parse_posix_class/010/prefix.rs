// Answer 0

#[test]
fn test_maybe_parse_posix_class_valid_posix_in_character_class() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "[[:alnum:]A]",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.maybe_parse_posix_class();
}

#[test]
fn test_maybe_parse_posix_class_invalid_posix_class() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "[[:loower:]]",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.maybe_parse_posix_class();
}

#[test]
fn test_maybe_parse_posix_class_combination_of_valid_and_invalid() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "[[:lower:][:upper:]B]",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.maybe_parse_posix_class();
}

#[test]
fn test_maybe_parse_posix_class_valid_nested_posix() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "[[:lower:][:upper:]A]",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.maybe_parse_posix_class();
}

