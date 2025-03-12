// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "{2,}";
    let parser = Parser {
        config: config,
        pattern: pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let hir = Hir { kind: HirKind::Repetition(Repetition { min: 2, max: None, greedy: true, sub: Box::new(Hir::empty()) }), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let concat = vec![hir.clone()];

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_empty_concat() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "{3}";
    let parser = Parser {
        config: config,
        pattern: pattern,
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
fn test_parse_counted_repetition_invalid_min() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "{invalid}";
    let parser = Parser {
        config: config,
        pattern: pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let hir = Hir { kind: HirKind::Repetition(Repetition { min: 0, max: None, greedy: true, sub: Box::new(Hir::empty()) }), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let concat = vec![hir];

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_exceeding_max() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "{1,3}";
    let parser = Parser {
        config: config,
        pattern: pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let hir = Hir { kind: HirKind::Repetition(Repetition { min: 4, max: Some(2), greedy: true, sub: Box::new(Hir::empty()) }), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let concat = vec![hir];

    let _ = parser.parse_counted_repetition(concat);
}

