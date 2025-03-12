// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    let flags = Flags {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        crlf: false,
        ignore_whitespace: false,
    };
    
    let config = Config { nest_limit: 10, flags };
    
    let hir_sub = Hir::char('a');  // Represents the sub-expression to repeat
    let mut concat = vec![hir_sub];
    
    let parser = Parser {
        config,
        pattern: "{2,4} ",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_min_max_case() {
    let flags = Flags {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        crlf: false,
        ignore_whitespace: false,
    };
    
    let config = Config { nest_limit: 10, flags };
    
    let hir_sub = Hir::char('b');  // Represents the sub-expression to repeat
    let mut concat = vec![hir_sub];
    
    let parser = Parser {
        config,
        pattern: "{1,3} ",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_single_value_case() {
    let flags = Flags {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        crlf: false,
        ignore_whitespace: false,
    };
    
    let config = Config { nest_limit: 10, flags };
    
    let hir_sub = Hir::char('c');  // Represents the sub-expression to repeat
    let mut concat = vec![hir_sub];
    
    let parser = Parser {
        config,
        pattern: "{5} ",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_counted_repetition(concat);
}

