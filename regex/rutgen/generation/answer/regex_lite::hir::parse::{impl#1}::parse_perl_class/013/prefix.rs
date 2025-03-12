// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\dabc"; // `\d` followed by other characters
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('d')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_space() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\sxyz"; // `\s` followed by other characters
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('s')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_word() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\w123"; // `\w` followed by other characters
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('w')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_perl_class();
}

