// Answer 0

#[test]
fn test_hir_anchor_start_multiline_crlf_false() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: true,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };

    let pattern = "some_pattern";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags {
            case_insensitive: false,
            multi_line: true,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        }),
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.hir_anchor_start();
}

#[test]
fn test_hir_anchor_start_multiline_crlf_true() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: true,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: true,
            ignore_whitespace: false,
        },
    };

    let pattern = "some_pattern";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags {
            case_insensitive: false,
            multi_line: true,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: true,
            ignore_whitespace: false,
        }),
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.hir_anchor_start();
}

