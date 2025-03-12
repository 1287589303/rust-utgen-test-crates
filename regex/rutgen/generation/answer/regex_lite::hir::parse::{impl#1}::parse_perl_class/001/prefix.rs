// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\d";
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
fn test_parse_perl_class_digit_uppercase() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\D";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('D')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_space() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\s";
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
fn test_parse_perl_class_space_uppercase() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\S";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('S')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_word() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\w";
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

#[test]
fn test_parse_perl_class_word_uppercase() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "\\W";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('W')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.parse_perl_class();
}

