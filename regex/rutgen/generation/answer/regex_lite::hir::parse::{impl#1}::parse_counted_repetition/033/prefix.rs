// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{2,3}";

    let hir = Hir::char('a');
    let mut concat = vec![hir];

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
    
    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_invalid_case() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{2,}";

    let hir = Hir::char('b');
    let mut concat = vec![hir];

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

    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_unexpected_eof() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{,}";

    let hir = Hir::char('c');
    let mut concat = vec![hir];

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

    let result = parser.parse_counted_repetition(concat);
} 

#[test]
fn test_parse_counted_repetition_duplicate_min_max() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{3,3}";

    let hir = Hir::char('d');
    let mut concat = vec![hir];

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

    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_invalid_range() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{3,2}";

    let hir = Hir::char('e');
    let mut concat = vec![hir];

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

    let result = parser.parse_counted_repetition(concat);
} 

