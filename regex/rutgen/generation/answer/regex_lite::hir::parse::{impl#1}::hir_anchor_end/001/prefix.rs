// Answer 0

#[test]
fn test_hir_anchor_end_multi_line_crlf() {
    let flags = Flags {
        multi_line: true,
        crlf: true,
        ..Flags::default()
    };
    let config = Config {
        nest_limit: 10,
        flags,
    };
    let pattern = ".*";
    
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };

    let _result = parser.hir_anchor_end();
}

#[test]
fn test_hir_anchor_end_multi_line_non_crlf() {
    let flags = Flags {
        multi_line: true,
        crlf: false,
        ..Flags::default()
    };
    let config = Config {
        nest_limit: 10,
        flags,
    };
    let pattern = ".*";

    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };

    let _result = parser.hir_anchor_end();
}

