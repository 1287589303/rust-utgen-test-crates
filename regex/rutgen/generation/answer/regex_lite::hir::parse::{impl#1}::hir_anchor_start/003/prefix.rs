// Answer 0

#[test]
fn test_hir_anchor_start_multiline_false_crlf_false() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "abc";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { multi_line: false, crlf: false, ..Flags::default() }),
        capture_names: RefCell::new(Vec::new()),
    };
    parser.hir_anchor_start();
}

#[test]
fn test_hir_anchor_start_multiline_false_crlf_true() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "abc";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { multi_line: false, crlf: true, ..Flags::default() }),
        capture_names: RefCell::new(Vec::new()),
    };
    parser.hir_anchor_start();
}

