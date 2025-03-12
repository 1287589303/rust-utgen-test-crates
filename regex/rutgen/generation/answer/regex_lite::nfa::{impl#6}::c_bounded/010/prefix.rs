// Answer 0

#[test]
fn test_c_bounded_greedy_min_less_than_max() {
    let config = Config { size_limit: None };
    let pattern = String::from("a");
    let compiler = Compiler::new(config, pattern);
    
    let hir = Hir {
        kind: HirKind::Char('a'),
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
fn test_c_bounded_non_greedy_min_less_than_max() {
    let config = Config { size_limit: None };
    let pattern = String::from("b");
    let compiler = Compiler::new(config, pattern);

    let hir = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let greedy = false;
    let min = 1;
    let max = 3;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_greedy_min_less_than_max_large() {
    let config = Config { size_limit: None };
    let pattern = String::from("c");
    let compiler = Compiler::new(config, pattern);

    let hir = Hir {
        kind: HirKind::Char('c'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let greedy = true;
    let min = 5;
    let max = 10;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_non_greedy_min_less_than_max_large() {
    let config = Config { size_limit: None };
    let pattern = String::from("d");
    let compiler = Compiler::new(config, pattern);

    let hir = Hir {
        kind: HirKind::Char('d'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let greedy = false;
    let min = 6;
    let max = 12;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

