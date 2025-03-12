// Answer 0

#[test]
fn test_c_bounded_with_valid_regex_and_greedy() {
    let config = Config { nest_limit: 1, flags: Default::default() };
    let pattern = String::from("a");
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(hir::Repetition {
            min: 1,
            max: Some(5),
            greedy: true,
        })),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let compiler = Compiler::new(config, pattern);
    let _result = compiler.c_bounded(&hir, true, 1, 5);
}

#[test]
fn test_c_bounded_with_valid_regex_and_non_greedy() {
    let config = Config { nest_limit: 1, flags: Default::default() };
    let pattern = String::from("a");
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(hir::Repetition {
            min: 1,
            max: Some(5),
            greedy: false,
        })),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    
    let compiler = Compiler::new(config, pattern);
    let _result = compiler.c_bounded(&hir, false, 1, 5);
}

