// Answer 0

#[test]
fn test_parse_counted_repetition_success() {
    let pattern = "{2,5}";
    let config = Config { nest_limit: 10, flags: Flags::default() };
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
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_empty_concat() {
    let pattern = "{0,2}";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut concat: Vec<Hir> = vec![];
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
fn test_parse_counted_repetition_invalid_comma() {
    let pattern = "{2,}";
    let config = Config { nest_limit: 10, flags: Flags::default() };
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
    let _ = parser.bump_and_bump_space();
    let result = parser.parse_counted_repetition(concat);
}

