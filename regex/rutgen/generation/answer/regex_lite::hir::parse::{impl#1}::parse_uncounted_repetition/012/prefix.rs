// Answer 0

#[test]
fn test_parse_uncounted_repetition_star_greedy() {
    let flags = Flags {
        swap_greed: true,
        ..Flags::default()
    };
    let config = Config {
        nest_limit: 10,
        flags,
    };

    let pattern = "*";
    let mut depth = Cell::new(0);
    let pos = Cell::new(0);
    let char = Cell::new(Some('*'));
    let capture_index = Cell::new(0);
    let capture_names = RefCell::new(vec!["group1".to_string()]);
    let parser = Parser {
        config,
        pattern,
        depth,
        pos,
        char,
        capture_index,
        flags: RefCell::new(Flags::default()),
        capture_names,
    };

    let concat = vec![Hir::char('a')];
    let _ = parser.parse_uncounted_repetition(concat);
}

#[test]
fn test_parse_uncounted_repetition_star_not_greedy() {
    let flags = Flags {
        swap_greed: true,
        ..Flags::default()
    };
    let config = Config {
        nest_limit: 10,
        flags,
    };

    let pattern = "*?";
    let mut depth = Cell::new(0);
    let pos = Cell::new(1); // Adjust position to indicate we are after '*'
    let char = Cell::new(Some('*'));
    let capture_index = Cell::new(0);
    let capture_names = RefCell::new(vec!["group1".to_string()]);
    let parser = Parser {
        config,
        pattern,
        depth,
        pos,
        char,
        capture_index,
        flags: RefCell::new(Flags::default()),
        capture_names,
    };

    let concat = vec![Hir::char('a')];
    let _ = parser.parse_uncounted_repetition(concat);
}

