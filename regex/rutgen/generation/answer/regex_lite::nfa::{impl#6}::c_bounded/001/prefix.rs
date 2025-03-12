// Answer 0

#[test]
fn test_c_bounded_failure_due_to_min_greater_than_max() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("pattern"));
    let hir = Hir { 
        kind: HirKind::Char('a'), 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None 
    };
    let greedy = true;
    let min = 2;
    let max = 1;
    let _result = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_failure_due_to_hir_empty() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("pattern"));
    let hir = Hir { 
        kind: HirKind::Empty, 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None 
    };
    let greedy = false;
    let min = 1;
    let max = 3;
    let _result = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_failure_due_to_invalid_character() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("pattern"));
    let hir = Hir { 
        kind: HirKind::Class(hir::Class::new(vec![('a', 'b')])), 
        is_start_anchored: true, 
        is_match_empty: false, 
        static_explicit_captures_len: None 
    };
    let greedy = true;
    let min = 5;
    let max = 5;
    let _result = compiler.c_bounded(&hir, greedy, min, max);
}

