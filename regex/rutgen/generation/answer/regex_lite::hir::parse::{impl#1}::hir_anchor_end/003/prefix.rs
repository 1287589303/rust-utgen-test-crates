// Answer 0

#[test]
fn test_hir_anchor_end_no_multiline_no_crlf() {
    let flags = Flags {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        crlf: false,
        ignore_whitespace: false,
    };
    let config = Config { nest_limit: 10, flags };
    let pattern = "";

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

    parser.hir_anchor_end();
}

#[test]
fn test_hir_anchor_end_no_multiline_with_crlf() {
    let flags = Flags {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        crlf: true,
        ignore_whitespace: false,
    };
    let config = Config { nest_limit: 10, flags };
    let pattern = "";

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

    parser.hir_anchor_end();
}

