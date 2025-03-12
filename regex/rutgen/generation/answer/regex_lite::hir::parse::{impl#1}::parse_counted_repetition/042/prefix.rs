// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_range() {
    let config = Config {
        nest_limit: 5,
        flags: Flags {
            swap_greed: true,
            ..Flags::default()
        },
    };
    
    let pattern = "{5,3}"; // Example that implies a range min > max
    let mut capture_names = RefCell::new(Vec::new());
    let flags = RefCell::new(config.flags);
    
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags,
        capture_names,
    };
    
    let last_hir = Hir {
        kind: HirKind::concat(vec![]), // assuming a valid last Hir
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    
    let mut concat = vec![last_hir];
    
    let _result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_no_comma() {
    let config = Config {
        nest_limit: 5,
        flags: Flags { 
            swap_greed: true,
            ..Flags::default()
        },
    };

    let pattern = "{5}"; // No comma, a valid single case for repetition
    let mut capture_names = RefCell::new(Vec::new());
    let flags = RefCell::new(config.flags);
    
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags,
        capture_names,
    };
    
    let last_hir = Hir {
        kind: HirKind::concat(vec![]), // assuming a valid last Hir
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    
    let mut concat = vec![last_hir];
    
    let _result = parser.parse_counted_repetition(concat);
}

