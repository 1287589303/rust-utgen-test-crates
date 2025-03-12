// Answer 0

#[test]
fn test_c_bounded_greedy_valid_input() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let compiler = Compiler::new(config, String::from("a{1,2}"));
    let hir = Hir {
        kind: HirKind::Capture(Capture {
            index: 0,
            name: None,
            sub: Hir::new_class(vec![('a', 'a')]),
        }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let greedy = true;
    let min = 1;
    let max = 2;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_non_greedy_valid_input() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let compiler = Compiler::new(config, String::from("a{1,2}"));
    let hir = Hir {
        kind: HirKind::Capture(Capture {
            index: 0,
            name: None,
            sub: Hir::new_class(vec![('a', 'a')]),
        }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let greedy = false;
    let min = 1;
    let max = 2;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_incremental_range() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let compiler = Compiler::new(config, String::from("a{2,3}"));
    let hir = Hir {
        kind: HirKind::Capture(Capture {
            index: 0,
            name: None,
            sub: Hir::new_class(vec![('a', 'a')]),
        }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let greedy = true;
    let min = 2;
    let max = 3;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_with_higher_bounds() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    let compiler = Compiler::new(config, String::from("a{3,5}"));
    let hir = Hir {
        kind: HirKind::Capture(Capture {
            index: 0,
            name: None,
            sub: Hir::new_class(vec![('a', 'a')]),
        }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let greedy = false;
    let min = 3;
    let max = 5;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

