// Answer 0

#[test]
fn test_invalid_special_word_boundary() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{invalid}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.maybe_parse_special_word_boundary();
}

#[test]
fn test_valid_special_word_boundary_start() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{start}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.maybe_parse_special_word_boundary();
}

#[test]
fn test_valid_special_word_boundary_end() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{end}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.maybe_parse_special_word_boundary();
}

#[test]
fn test_unclosed_special_word_boundary() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{start";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.maybe_parse_special_word_boundary();
}

#[test]
fn test_invalid_characters_after_special_word_boundary() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{startX}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.maybe_parse_special_word_boundary();
}

