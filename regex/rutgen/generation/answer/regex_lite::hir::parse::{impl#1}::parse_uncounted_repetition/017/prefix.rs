// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = "a?b";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('?')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let concat = vec![Hir::char('a')];
    let _ = parser.parse_uncounted_repetition(concat);
}

#[test]
fn test_parse_uncounted_repetition_with_star() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = "a*b";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('*')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let concat = vec![Hir::char('a')];
    let _ = parser.parse_uncounted_repetition(concat);
}

#[test]
fn test_parse_uncounted_repetition_with_plus() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = "a+b";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('+')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let concat = vec![Hir::char('a')];
    let _ = parser.parse_uncounted_repetition(concat);
}

