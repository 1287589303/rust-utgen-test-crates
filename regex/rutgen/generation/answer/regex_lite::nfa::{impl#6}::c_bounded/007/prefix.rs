// Answer 0

#[test]
fn test_c_bounded_success_case() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("a{2,5}"));
    let hir = Hir { kind: HirKind::Repetition(Box::new(hir::Repetition { min: 2, max: 5, greedy: true })), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let greedy = true;
    let min = 2;
    let max = 5;
    
    let result = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_zero_or_one_case() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("b{0,1}"));
    let hir = Hir { kind: HirKind::Repetition(Box::new(hir::Repetition { min: 0, max: 1, greedy: false })), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let greedy = false;
    let min = 0;
    let max = 1;

    let result = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
#[should_panic]
fn test_c_bounded_invalid_case() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("c{3,3}"));
    let hir = Hir { kind: HirKind::Repetition(Box::new(hir::Repetition { min: 3, max: 3, greedy: true })), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let greedy = true;
    let min = 3;
    let max = 3;

    let result = compiler.c_bounded(&hir, greedy, min, max);
}

