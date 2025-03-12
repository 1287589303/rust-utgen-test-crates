// Answer 0

#[test]
fn test_parse_uncounted_repetition_empty_concat() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };

    let parser = Parser {
        config,
        pattern: "",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let concat: Vec<Hir> = vec![];

    let _ = parser.parse_uncounted_repetition(concat);
}

