// Answer 0

#[test]
fn test_hir_dot_crlf() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: true,
            ignore_whitespace: false,
        },
    };
    let pattern = "...";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\x00')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: true,
            ignore_whitespace: false,
        }),
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.hir_dot();
}

#[test]
fn test_hir_dot_crlf_with_other_characters() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: true,
            ignore_whitespace: false,
        },
    };
    let pattern = "...";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1),
        char: Cell::new(Some('\u{10FFFF}')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: true,
            ignore_whitespace: false,
        }),
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.hir_dot();
}

