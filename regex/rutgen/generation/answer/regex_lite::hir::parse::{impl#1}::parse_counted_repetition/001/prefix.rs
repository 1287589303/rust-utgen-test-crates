// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{1,3}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["group_name".to_string()]),
    };
    
    let concat = vec![Hir::char('a')];
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_empty_concat() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{1,3}";
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
    
    let concat: Vec<Hir> = vec![];
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic(expected = "expected opening brace")]
fn test_parse_counted_repetition_no_opening_brace() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "1,3}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('1')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["group_name".to_string()]),
    };
    
    let concat = vec![Hir::char('a')];
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_malformed_decimal() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{a,b}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["group_name".to_string()]),
    };
    
    let concat = vec![Hir::char('a')];
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_no_max() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "{1,}";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec!["group_name".to_string()]),
    };
    
    let concat = vec![Hir::char('a')];
    let _ = parser.parse_counted_repetition(concat);
}

