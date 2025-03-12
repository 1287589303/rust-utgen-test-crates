// Answer 0

#[test]
fn test_parse_inner_nested_repetition() {
    let config = Config {
        nest_limit: 100,
        flags: Flags::default(),
    };
    let pattern = "{1,2}";
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
    let _ = parser.increment_depth().expect("increment_depth failed");
    
    parser.bump_space();
    assert_eq!(parser.is_done(), false);
    
    parser.bump(); // move past '{'
    parser.char.set(Some('}')); // simulate parsing
    let concat = vec![Hir::char('a')]; // Provide a simple concatenation for test
    let _ = parser.parse_counted_repetition(concat).expect("parse_counted_repetition failed");
    
    parser.bump_space();
    assert_eq!(parser.is_done(), true);
    
    let _ = parser.parse_inner().expect("parse_inner failed");
}

#[test]
fn test_parse_inner_unfinished_repetition() {
    let config = Config {
        nest_limit: 100,
        flags: Flags::default(),
    };
    let pattern = "{3,";
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
    let _ = parser.increment_depth().expect("increment_depth failed");
    
    parser.bump_space();
    assert_eq!(parser.is_done(), false);
    
    parser.bump(); // move past '{'
    parser.char.set(Some('}')); // simulate parsing
    let concat = vec![Hir::char('b')]; // Provide a simple concatenation for test
    let _ = parser.parse_counted_repetition(concat).expect("parse_counted_repetition failed");
    
    parser.bump_space();
    assert_eq!(parser.is_done(), true);
    
    let _ = parser.parse_inner().expect("parse_inner failed");
}

