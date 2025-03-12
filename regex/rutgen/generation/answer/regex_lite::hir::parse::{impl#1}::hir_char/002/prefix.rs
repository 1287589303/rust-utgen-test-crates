// Answer 0

#[test]
fn test_hir_char_lowercase_a() {
    let flags = Flags { case_insensitive: true, ..Flags::default() };
    let config = Config { nest_limit: 10, flags };
    let parser = Parser {
        config,
        pattern: "",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.hir_char('a');
}

#[test]
fn test_hir_char_lowercase_b() {
    let flags = Flags { case_insensitive: true, ..Flags::default() };
    let config = Config { nest_limit: 10, flags };
    let parser = Parser {
        config,
        pattern: "",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('b')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.hir_char('b');
}

#[test]
fn test_hir_char_lowercase_z() {
    let flags = Flags { case_insensitive: true, ..Flags::default() };
    let config = Config { nest_limit: 10, flags };
    let parser = Parser {
        config,
        pattern: "",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('z')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.hir_char('z');
}

#[test]
fn test_hir_char_uppercase_a() {
    let flags = Flags { case_insensitive: true, ..Flags::default() };
    let config = Config { nest_limit: 10, flags };
    let parser = Parser {
        config,
        pattern: "",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('A')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.hir_char('A');
}

#[test]
fn test_hir_char_uppercase_z() {
    let flags = Flags { case_insensitive: true, ..Flags::default() };
    let config = Config { nest_limit: 10, flags };
    let parser = Parser {
        config,
        pattern: "",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('Z')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.hir_char('Z');
}

